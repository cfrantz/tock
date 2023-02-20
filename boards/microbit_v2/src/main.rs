//! Tock kernel for the Micro:bit v2.
//!
//! It is based on nRF52833 SoC (Cortex M4 core with a BLE).

#![no_std]
// Disable this attribute when documenting, as a workaround for
// https://github.com/rust-lang/rust/issues/62184.
#![cfg_attr(not(doc), no_main)]
#![deny(missing_docs)]

use kernel::capabilities;
use kernel::component::Component;
use kernel::dynamic_deferred_call::{DynamicDeferredCall, DynamicDeferredCallClientState};
use kernel::hil::time::Counter;
use kernel::platform::{KernelResources, SyscallDriverLookup};
use kernel::scheduler::round_robin::RoundRobinSched;

#[allow(unused_imports)]
use kernel::{create_capability, debug, debug_gpio, debug_verbose, static_init};

use nrf52833::gpio::Pin;
use nrf52833::interrupt_service::Nrf52833DefaultPeripherals;

// Kernel LED (same as microphone LED)
const LED_KERNEL_PIN: Pin = Pin::P0_20;
const LED_MICROPHONE_PIN: Pin = Pin::P0_20;

// Buttons
const BUTTON_A: Pin = Pin::P0_14;
const BUTTON_B: Pin = Pin::P0_23;
const TOUCH_LOGO: Pin = Pin::P1_04;

// GPIOs

// P0, P1 and P2 are used as ADC, comment them in the ADC section to use them as GPIO
const _GPIO_P0: Pin = Pin::P0_02;
const _GPIO_P1: Pin = Pin::P0_03;
const _GPIO_P2: Pin = Pin::P0_04;
const GPIO_P8: Pin = Pin::P0_10;
const GPIO_P9: Pin = Pin::P0_09;
const GPIO_P16: Pin = Pin::P1_02;

const UART_TX_PIN: Pin = Pin::P0_06;
const UART_RX_PIN: Pin = Pin::P1_08;

/// LED matrix
const LED_MATRIX_COLS: [Pin; 5] = [Pin::P0_28, Pin::P0_11, Pin::P0_31, Pin::P1_05, Pin::P0_30];
const LED_MATRIX_ROWS: [Pin; 5] = [Pin::P0_21, Pin::P0_22, Pin::P0_15, Pin::P0_24, Pin::P0_19];

// Speaker

const SPEAKER_PIN: Pin = Pin::P0_00;

/// I2C pins for all of the sensors.
const I2C_SDA_PIN: Pin = Pin::P0_16;
const I2C_SCL_PIN: Pin = Pin::P0_08;

/// UART Writer for panic!()s.
pub mod io;

// State for loading and holding applications.
// How should the kernel respond when a process faults.
const FAULT_RESPONSE: kernel::process::PanicFaultPolicy = kernel::process::PanicFaultPolicy {};

// Number of concurrent processes this platform supports.
const NUM_PROCS: usize = 4;

static mut PROCESSES: [Option<&'static dyn kernel::process::Process>; NUM_PROCS] =
    [None; NUM_PROCS];

static mut CHIP: Option<&'static nrf52833::chip::NRF52<Nrf52833DefaultPeripherals>> = None;
static mut PROCESS_PRINTER: Option<&'static kernel::process::ProcessPrinterText> = None;

/// Dummy buffer that causes the linker to reserve enough space for the stack.
#[no_mangle]
#[link_section = ".stack_buffer"]
pub static mut STACK_MEMORY: [u8; 0x1100] = [0; 0x1100];
// debug mode requires more stack space
// pub static mut STACK_MEMORY: [u8; 0x2000] = [0; 0x2000];

// Function for the process console to use to reboot the board
fn reset() -> ! {
    unsafe {
        cortexm4::scb::reset();
    }
    loop {
        cortexm4::support::nop();
    }
}

/// Supported drivers by the platform
pub struct MicroBit {
    ble_radio: &'static extra_capsules::ble_advertising_driver::BLE<
        'static,
        nrf52::ble_radio::Radio<'static>,
        core_capsules::virtualizers::virtual_alarm::VirtualMuxAlarm<'static, nrf52::rtc::Rtc<'static>>,
    >,
    console: &'static core_capsules::console::Console<'static>,
    gpio: &'static core_capsules::gpio::GPIO<'static, nrf52::gpio::GPIOPin<'static>>,
    led: &'static core_capsules::led::LedDriver<
        'static,
        extra_capsules::led_matrix::LedMatrixLed<
            'static,
            nrf52::gpio::GPIOPin<'static>,
            core_capsules::virtualizers::virtual_alarm::VirtualMuxAlarm<'static, nrf52::rtc::Rtc<'static>>,
        >,
        25,
    >,
    button: &'static core_capsules::button::Button<'static, nrf52::gpio::GPIOPin<'static>>,
    rng: &'static core_capsules::rng::RngDriver<'static>,
    ninedof: &'static extra_capsules::ninedof::NineDof<'static>,
    lsm303agr: &'static extra_capsules::lsm303agr::Lsm303agrI2C<'static>,
    temperature: &'static extra_capsules::temperature::TemperatureSensor<'static>,
    ipc: kernel::ipc::IPC<{ NUM_PROCS as u8 }>,
    adc: &'static core_capsules::adc::AdcVirtualized<'static>,
    alarm: &'static core_capsules::alarm::AlarmDriver<
        'static,
        core_capsules::virtualizers::virtual_alarm::VirtualMuxAlarm<'static, nrf52::rtc::Rtc<'static>>,
    >,
    buzzer_driver: &'static extra_capsules::buzzer_driver::Buzzer<
        'static,
        extra_capsules::buzzer_pwm::PwmBuzzer<
            'static,
            core_capsules::virtualizers::virtual_alarm::VirtualMuxAlarm<'static, nrf52833::rtc::Rtc<'static>>,
            core_capsules::virtualizers::virtual_pwm::PwmPinUser<'static, nrf52833::pwm::Pwm>,
        >,
    >,
    pwm: &'static extra_capsules::pwm::Pwm<'static, 1>,
    app_flash: &'static extra_capsules::app_flash_driver::AppFlash<'static>,
    sound_pressure: &'static extra_capsules::sound_pressure::SoundPressureSensor<'static>,

    scheduler: &'static RoundRobinSched<'static>,
    systick: cortexm4::systick::SysTick,
}

impl SyscallDriverLookup for MicroBit {
    fn with_driver<F, R>(&self, driver_num: usize, f: F) -> R
    where
        F: FnOnce(Option<&dyn kernel::syscall::SyscallDriver>) -> R,
    {
        match driver_num {
            core_capsules::console::DRIVER_NUM => f(Some(self.console)),
            core_capsules::gpio::DRIVER_NUM => f(Some(self.gpio)),
            core_capsules::alarm::DRIVER_NUM => f(Some(self.alarm)),
            core_capsules::button::DRIVER_NUM => f(Some(self.button)),
            core_capsules::led::DRIVER_NUM => f(Some(self.led)),
            extra_capsules::ninedof::DRIVER_NUM => f(Some(self.ninedof)),
            core_capsules::adc::DRIVER_NUM => f(Some(self.adc)),
            extra_capsules::temperature::DRIVER_NUM => f(Some(self.temperature)),
            extra_capsules::lsm303agr::DRIVER_NUM => f(Some(self.lsm303agr)),
            core_capsules::rng::DRIVER_NUM => f(Some(self.rng)),
            extra_capsules::ble_advertising_driver::DRIVER_NUM => f(Some(self.ble_radio)),
            extra_capsules::buzzer_driver::DRIVER_NUM => f(Some(self.buzzer_driver)),
            extra_capsules::pwm::DRIVER_NUM => f(Some(self.pwm)),
            extra_capsules::app_flash_driver::DRIVER_NUM => f(Some(self.app_flash)),
            extra_capsules::sound_pressure::DRIVER_NUM => f(Some(self.sound_pressure)),
            kernel::ipc::DRIVER_NUM => f(Some(&self.ipc)),
            _ => f(None),
        }
    }
}

impl KernelResources<nrf52833::chip::NRF52<'static, Nrf52833DefaultPeripherals<'static>>>
    for MicroBit
{
    type SyscallDriverLookup = Self;
    type SyscallFilter = ();
    type ProcessFault = ();
    type CredentialsCheckingPolicy = ();
    type Scheduler = RoundRobinSched<'static>;
    type SchedulerTimer = cortexm4::systick::SysTick;
    type WatchDog = ();
    type ContextSwitchCallback = ();

    fn syscall_driver_lookup(&self) -> &Self::SyscallDriverLookup {
        &self
    }
    fn syscall_filter(&self) -> &Self::SyscallFilter {
        &()
    }
    fn process_fault(&self) -> &Self::ProcessFault {
        &()
    }
    fn credentials_checking_policy(&self) -> &'static Self::CredentialsCheckingPolicy {
        &()
    }
    fn scheduler(&self) -> &Self::Scheduler {
        self.scheduler
    }
    fn scheduler_timer(&self) -> &Self::SchedulerTimer {
        &self.systick
    }
    fn watchdog(&self) -> &Self::WatchDog {
        &()
    }
    fn context_switch_callback(&self) -> &Self::ContextSwitchCallback {
        &()
    }
}

/// This is in a separate, inline(never) function so that its stack frame is
/// removed when this function returns. Otherwise, the stack space used for
/// these static_inits is wasted.
#[inline(never)]
unsafe fn create_peripherals() -> &'static mut Nrf52833DefaultPeripherals<'static> {
    // Initialize chip peripheral drivers
    let nrf52833_peripherals = static_init!(
        Nrf52833DefaultPeripherals,
        Nrf52833DefaultPeripherals::new()
    );

    nrf52833_peripherals
}

/// Main function called after RAM initialized.
#[no_mangle]
pub unsafe fn main() {
    nrf52833::init();

    let nrf52833_peripherals = create_peripherals();

    // set up circular peripheral dependencies
    nrf52833_peripherals.init();

    let base_peripherals = &nrf52833_peripherals.nrf52;

    let board_kernel = static_init!(kernel::Kernel, kernel::Kernel::new(&PROCESSES));

    //--------------------------------------------------------------------------
    // CAPABILITIES
    //--------------------------------------------------------------------------

    // Create capabilities that the board needs to call certain protected kernel
    // functions.
    let process_management_capability =
        create_capability!(capabilities::ProcessManagementCapability);
    let main_loop_capability = create_capability!(capabilities::MainLoopCapability);
    let memory_allocation_capability = create_capability!(capabilities::MemoryAllocationCapability);

    //--------------------------------------------------------------------------
    // DEBUG GPIO
    //--------------------------------------------------------------------------

    // Configure kernel debug GPIOs as early as possible. These are used by the
    // `debug_gpio!(0, toggle)` macro. We uconfigure these early so that the
    // macro is available during most of the setup code and kernel exection.
    kernel::debug::assign_gpios(
        Some(&nrf52833_peripherals.gpio_port[LED_KERNEL_PIN]),
        None,
        None,
    );

    //--------------------------------------------------------------------------
    // GPIO
    //--------------------------------------------------------------------------

    let gpio = components::gpio::GpioComponent::new(
        board_kernel,
        core_capsules::gpio::DRIVER_NUM,
        components::gpio_component_helper!(
            nrf52833::gpio::GPIOPin,
            // Used as ADC, comment them out in the ADC section to use them as GPIO
            // 0 => &nrf52833_peripherals.gpio_port[GPIO_P0],
            // 1 => &nrf52833_peripherals.gpio_port[_GPIO_P1],
            // 2 => &nrf52833_peripherals.gpio_port[_GPIO_P2],
            // Used as PWM, comment them out in the PWM section to use them as GPIO
            //8 => &nrf52833_peripherals.gpio_port[GPIO_P8],
            9 => &nrf52833_peripherals.gpio_port[GPIO_P9],
            16 => &nrf52833_peripherals.gpio_port[GPIO_P16],
        ),
    )
    .finalize(components::gpio_component_static!(nrf52833::gpio::GPIOPin));

    //--------------------------------------------------------------------------
    // Buttons
    //--------------------------------------------------------------------------
    let button = components::button::ButtonComponent::new(
        board_kernel,
        core_capsules::button::DRIVER_NUM,
        components::button_component_helper!(
            nrf52833::gpio::GPIOPin,
            (
                &nrf52833_peripherals.gpio_port[BUTTON_A],
                kernel::hil::gpio::ActivationMode::ActiveLow,
                kernel::hil::gpio::FloatingState::PullNone
            ), // A
            (
                &nrf52833_peripherals.gpio_port[BUTTON_B],
                kernel::hil::gpio::ActivationMode::ActiveLow,
                kernel::hil::gpio::FloatingState::PullNone
            ), // B
            (
                &nrf52833_peripherals.gpio_port[TOUCH_LOGO],
                kernel::hil::gpio::ActivationMode::ActiveLow,
                kernel::hil::gpio::FloatingState::PullNone
            ), // Touch Logo
        ),
    )
    .finalize(components::button_component_static!(
        nrf52833::gpio::GPIOPin
    ));

    //--------------------------------------------------------------------------
    // Deferred Call (Dynamic) Setup
    //--------------------------------------------------------------------------

    let dynamic_deferred_call_clients =
        static_init!([DynamicDeferredCallClientState; 3], Default::default());
    let dynamic_deferred_caller = static_init!(
        DynamicDeferredCall,
        DynamicDeferredCall::new(dynamic_deferred_call_clients)
    );
    DynamicDeferredCall::set_global_instance(dynamic_deferred_caller);

    //--------------------------------------------------------------------------
    // ALARM & TIMER
    //--------------------------------------------------------------------------

    let rtc = &base_peripherals.rtc;
    let _ = rtc.start();

    let mux_alarm = components::alarm::AlarmMuxComponent::new(rtc)
        .finalize(components::alarm_mux_component_static!(nrf52::rtc::Rtc));
    let alarm = components::alarm::AlarmDriverComponent::new(
        board_kernel,
        core_capsules::alarm::DRIVER_NUM,
        mux_alarm,
    )
    .finalize(components::alarm_component_static!(nrf52::rtc::Rtc));

    //--------------------------------------------------------------------------
    // PWM & BUZZER
    //--------------------------------------------------------------------------

    use kernel::hil::buzzer::Buzzer;
    use kernel::hil::time::Alarm;

    let mux_pwm = components::pwm::PwmMuxComponent::new(&base_peripherals.pwm0)
        .finalize(components::pwm_mux_component_static!(nrf52833::pwm::Pwm));

    let virtual_pwm_buzzer = static_init!(
        core_capsules::virtualizers::virtual_pwm::PwmPinUser<'static, nrf52833::pwm::Pwm>,
        core_capsules::virtualizers::virtual_pwm::PwmPinUser::new(
            mux_pwm,
            nrf52833::pinmux::Pinmux::new(SPEAKER_PIN as u32)
        )
    );
    virtual_pwm_buzzer.add_to_mux();

    let virtual_alarm_buzzer = static_init!(
        core_capsules::virtualizers::virtual_alarm::VirtualMuxAlarm<'static, nrf52833::rtc::Rtc>,
        core_capsules::virtualizers::virtual_alarm::VirtualMuxAlarm::new(mux_alarm)
    );
    virtual_alarm_buzzer.setup();

    let pwm_buzzer = static_init!(
        extra_capsules::buzzer_pwm::PwmBuzzer<
            'static,
            core_capsules::virtualizers::virtual_alarm::VirtualMuxAlarm<'static, nrf52833::rtc::Rtc>,
            core_capsules::virtualizers::virtual_pwm::PwmPinUser<'static, nrf52833::pwm::Pwm>,
        >,
        extra_capsules::buzzer_pwm::PwmBuzzer::new(
            virtual_pwm_buzzer,
            virtual_alarm_buzzer,
            extra_capsules::buzzer_pwm::DEFAULT_MAX_BUZZ_TIME_MS,
        )
    );

    let buzzer_driver = static_init!(
        extra_capsules::buzzer_driver::Buzzer<
            'static,
            extra_capsules::buzzer_pwm::PwmBuzzer<
                'static,
                core_capsules::virtualizers::virtual_alarm::VirtualMuxAlarm<'static, nrf52833::rtc::Rtc>,
                core_capsules::virtualizers::virtual_pwm::PwmPinUser<'static, nrf52833::pwm::Pwm>,
            >,
        >,
        extra_capsules::buzzer_driver::Buzzer::new(
            pwm_buzzer,
            extra_capsules::buzzer_driver::DEFAULT_MAX_BUZZ_TIME_MS,
            board_kernel.create_grant(
                extra_capsules::buzzer_driver::DRIVER_NUM,
                &memory_allocation_capability
            )
        )
    );

    pwm_buzzer.set_client(buzzer_driver);

    virtual_alarm_buzzer.set_alarm_client(pwm_buzzer);

    let pwm =
        components::pwm::PwmVirtualComponent::new(board_kernel, extra_capsules::pwm::DRIVER_NUM)
            .finalize(components::pwm_syscall_component_helper!(
                components::pwm::PwmPinComponent::new(
                    &mux_pwm,
                    nrf52833::pinmux::Pinmux::new(GPIO_P8 as u32)
                )
                .finalize(components::pwm_pin_user_component_static!(
                    nrf52833::pwm::Pwm
                ))
            ));

    //--------------------------------------------------------------------------
    // UART & CONSOLE & DEBUG
    //--------------------------------------------------------------------------

    base_peripherals.uarte0.initialize(
        nrf52::pinmux::Pinmux::new(UART_TX_PIN as u32),
        nrf52::pinmux::Pinmux::new(UART_RX_PIN as u32),
        None,
        None,
    );

    // Create a shared UART channel for the console and for kernel debug.
    let uart_mux = components::console::UartMuxComponent::new(
        &base_peripherals.uarte0,
        115200,
        dynamic_deferred_caller,
    )
    .finalize(components::uart_mux_component_static!());

    // Setup the console.
    let console = components::console::ConsoleComponent::new(
        board_kernel,
        core_capsules::console::DRIVER_NUM,
        uart_mux,
    )
    .finalize(components::console_component_static!());
    // Create the debugger object that handles calls to `debug!()`.
    components::debug_writer::DebugWriterComponent::new(uart_mux)
        .finalize(components::debug_writer_component_static!());

    //--------------------------------------------------------------------------
    // RANDOM NUMBERS
    //--------------------------------------------------------------------------

    let rng = components::rng::RngComponent::new(
        board_kernel,
        core_capsules::rng::DRIVER_NUM,
        &base_peripherals.trng,
    )
    .finalize(components::rng_component_static!());

    //--------------------------------------------------------------------------
    // SENSORS
    //--------------------------------------------------------------------------

    base_peripherals.twi0.configure(
        nrf52833::pinmux::Pinmux::new(I2C_SCL_PIN as u32),
        nrf52833::pinmux::Pinmux::new(I2C_SDA_PIN as u32),
    );

    let sensors_i2c_bus = components::i2c::I2CMuxComponent::new(
        &base_peripherals.twi0,
        None,
        dynamic_deferred_caller,
    )
    .finalize(components::i2c_mux_component_static!());

    // LSM303AGR

    let lsm303agr = components::lsm303agr::Lsm303agrI2CComponent::new(
        sensors_i2c_bus,
        None,
        None,
        board_kernel,
        extra_capsules::lsm303agr::DRIVER_NUM,
    )
    .finalize(components::lsm303agr_component_static!());

    if let Err(error) = lsm303agr.configure(
        extra_capsules::lsm303xx::Lsm303AccelDataRate::DataRate25Hz,
        false,
        extra_capsules::lsm303xx::Lsm303Scale::Scale2G,
        false,
        true,
        extra_capsules::lsm303xx::Lsm303MagnetoDataRate::DataRate3_0Hz,
        extra_capsules::lsm303xx::Lsm303Range::Range1_9G,
    ) {
        debug!("Failed to configure LSM303AGR sensor ({:?})", error);
    }

    let ninedof = components::ninedof::NineDofComponent::new(
        board_kernel,
        extra_capsules::ninedof::DRIVER_NUM,
    )
    .finalize(components::ninedof_component_static!(lsm303agr));

    // Temperature

    let temperature = components::temperature::TemperatureComponent::new(
        board_kernel,
        extra_capsules::temperature::DRIVER_NUM,
        &base_peripherals.temp,
    )
    .finalize(components::temperature_component_static!());

    //--------------------------------------------------------------------------
    // ADC
    //--------------------------------------------------------------------------
    base_peripherals.adc.calibrate();

    let adc_mux = components::adc::AdcMuxComponent::new(&base_peripherals.adc)
        .finalize(components::adc_mux_component_static!(nrf52833::adc::Adc));

    // Comment out the following to use P0, P1 and P2 as GPIO
    let adc_syscall =
        components::adc::AdcVirtualComponent::new(board_kernel, core_capsules::adc::DRIVER_NUM)
            .finalize(components::adc_syscall_component_helper!(
                // ADC Ring 0 (P0)
                components::adc::AdcComponent::new(
                    &adc_mux,
                    nrf52833::adc::AdcChannelSetup::new(nrf52833::adc::AdcChannel::AnalogInput0)
                )
                .finalize(components::adc_component_static!(nrf52833::adc::Adc)),
                // ADC Ring 1 (P1)
                components::adc::AdcComponent::new(
                    &adc_mux,
                    nrf52833::adc::AdcChannelSetup::new(nrf52833::adc::AdcChannel::AnalogInput1)
                )
                .finalize(components::adc_component_static!(nrf52833::adc::Adc)),
                // ADC Ring 2 (P2)
                components::adc::AdcComponent::new(
                    &adc_mux,
                    nrf52833::adc::AdcChannelSetup::new(nrf52833::adc::AdcChannel::AnalogInput2)
                )
                .finalize(components::adc_component_static!(nrf52833::adc::Adc))
            ));

    // Microphone

    let adc_microphone = components::adc_microphone::AdcMicrophoneComponent::new(
        adc_mux,
        nrf52833::adc::AdcChannelSetup::setup(
            nrf52833::adc::AdcChannel::AnalogInput3,
            nrf52833::adc::AdcChannelGain::Gain4,
            nrf52833::adc::AdcChannelResistor::Bypass,
            nrf52833::adc::AdcChannelResistor::Pulldown,
            nrf52833::adc::AdcChannelSamplingTime::us3,
        ),
        Some(&nrf52833_peripherals.gpio_port[LED_MICROPHONE_PIN]),
    )
    .finalize(components::adc_microphone_component_static!(
        // adc
        nrf52833::adc::Adc,
        // buffer size
        50,
        // gpio
        nrf52833::gpio::GPIOPin
    ));

    let _ = &nrf52833_peripherals.gpio_port[LED_MICROPHONE_PIN].set_high_drive(true);

    let sound_pressure = components::sound_pressure::SoundPressureComponent::new(
        board_kernel,
        extra_capsules::sound_pressure::DRIVER_NUM,
        adc_microphone,
    )
    .finalize(components::sound_pressure_component_static!());

    //--------------------------------------------------------------------------
    // STORAGE
    //--------------------------------------------------------------------------

    let mux_flash = components::flash::FlashMuxComponent::new(&base_peripherals.nvmc).finalize(
        components::flash_mux_component_static!(nrf52833::nvmc::Nvmc),
    );

    // App Flash

    let virtual_app_flash = components::flash::FlashUserComponent::new(mux_flash).finalize(
        components::flash_user_component_static!(nrf52833::nvmc::Nvmc),
    );

    let app_flash = components::app_flash_driver::AppFlashComponent::new(
        board_kernel,
        extra_capsules::app_flash_driver::DRIVER_NUM,
        virtual_app_flash,
    )
    .finalize(components::app_flash_component_static!(
        core_capsules::virtualizers::virtual_flash::FlashUser<'static, nrf52833::nvmc::Nvmc>,
        512
    ));

    //--------------------------------------------------------------------------
    // WIRELESS
    //--------------------------------------------------------------------------

    let ble_radio = components::ble::BLEComponent::new(
        board_kernel,
        extra_capsules::ble_advertising_driver::DRIVER_NUM,
        &base_peripherals.ble_radio,
        mux_alarm,
    )
    .finalize(components::ble_component_static!(
        nrf52833::rtc::Rtc,
        nrf52833::ble_radio::Radio
    ));

    //--------------------------------------------------------------------------
    // LED Matrix
    //--------------------------------------------------------------------------

    let led_matrix = components::led_matrix::LedMatrixComponent::new(
        mux_alarm,
        components::led_line_component_static!(
            nrf52833::gpio::GPIOPin,
            &nrf52833_peripherals.gpio_port[LED_MATRIX_COLS[0]],
            &nrf52833_peripherals.gpio_port[LED_MATRIX_COLS[1]],
            &nrf52833_peripherals.gpio_port[LED_MATRIX_COLS[2]],
            &nrf52833_peripherals.gpio_port[LED_MATRIX_COLS[3]],
            &nrf52833_peripherals.gpio_port[LED_MATRIX_COLS[4]],
        ),
        components::led_line_component_static!(
            nrf52833::gpio::GPIOPin,
            &nrf52833_peripherals.gpio_port[LED_MATRIX_ROWS[0]],
            &nrf52833_peripherals.gpio_port[LED_MATRIX_ROWS[1]],
            &nrf52833_peripherals.gpio_port[LED_MATRIX_ROWS[2]],
            &nrf52833_peripherals.gpio_port[LED_MATRIX_ROWS[3]],
            &nrf52833_peripherals.gpio_port[LED_MATRIX_ROWS[4]],
        ),
        kernel::hil::gpio::ActivationMode::ActiveLow,
        kernel::hil::gpio::ActivationMode::ActiveHigh,
        60,
    )
    .finalize(components::led_matrix_component_static!(
        nrf52833::gpio::GPIOPin,
        nrf52::rtc::Rtc<'static>,
        5,
        5
    ));

    let led = static_init!(
        core_capsules::led::LedDriver<
            'static,
            extra_capsules::led_matrix::LedMatrixLed<
                'static,
                nrf52::gpio::GPIOPin<'static>,
                core_capsules::virtualizers::virtual_alarm::VirtualMuxAlarm<'static, nrf52::rtc::Rtc<'static>>,
            >,
            25,
        >,
        core_capsules::led::LedDriver::new(components::led_matrix_leds!(
            nrf52::gpio::GPIOPin<'static>,
            core_capsules::virtualizers::virtual_alarm::VirtualMuxAlarm<'static, nrf52::rtc::Rtc<'static>>,
            led_matrix,
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (0, 1),
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 1),
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 2),
            (4, 2),
            (0, 3),
            (1, 3),
            (2, 3),
            (3, 3),
            (4, 3),
            (0, 4),
            (1, 4),
            (2, 4),
            (3, 4),
            (4, 4)
        )),
    );

    //--------------------------------------------------------------------------
    // Process Console
    //--------------------------------------------------------------------------
    let process_printer = components::process_printer::ProcessPrinterTextComponent::new()
        .finalize(components::process_printer_text_component_static!());
    PROCESS_PRINTER = Some(process_printer);

    let _process_console = components::process_console::ProcessConsoleComponent::new(
        board_kernel,
        uart_mux,
        mux_alarm,
        process_printer,
        Some(reset),
    )
    .finalize(components::process_console_component_static!(
        nrf52833::rtc::Rtc
    ));
    let _ = _process_console.start();

    //--------------------------------------------------------------------------
    // FINAL SETUP AND BOARD BOOT
    //--------------------------------------------------------------------------

    // it seems that microbit v2 has no external clock
    base_peripherals.clock.low_stop();
    base_peripherals.clock.high_stop();
    base_peripherals.clock.low_start();
    base_peripherals.clock.high_start();
    while !base_peripherals.clock.low_started() {}
    while !base_peripherals.clock.high_started() {}

    let scheduler = components::sched::round_robin::RoundRobinComponent::new(&PROCESSES)
        .finalize(components::round_robin_component_static!(NUM_PROCS));

    let microbit = MicroBit {
        ble_radio,
        console,
        gpio,
        button,
        led,
        rng,
        temperature,
        lsm303agr,
        ninedof,
        buzzer_driver,
        pwm,
        sound_pressure,
        adc: adc_syscall,
        alarm,
        app_flash,
        ipc: kernel::ipc::IPC::new(
            board_kernel,
            kernel::ipc::DRIVER_NUM,
            &memory_allocation_capability,
        ),

        scheduler,
        systick: cortexm4::systick::SysTick::new_with_calibration(64000000),
    };

    let chip = static_init!(
        nrf52833::chip::NRF52<Nrf52833DefaultPeripherals>,
        nrf52833::chip::NRF52::new(nrf52833_peripherals)
    );
    CHIP = Some(chip);

    debug!("Initialization complete. Entering main loop.");

    //--------------------------------------------------------------------------
    // PROCESSES AND MAIN LOOP
    //--------------------------------------------------------------------------

    // These symbols are defined in the linker script.
    extern "C" {
        /// Beginning of the ROM region containing app images.
        static _sapps: u8;
        /// End of the ROM region containing app images.
        static _eapps: u8;
        /// Beginning of the RAM region for app memory.
        static mut _sappmem: u8;
        /// End of the RAM region for app memory.
        static _eappmem: u8;
    }

    kernel::process::load_processes(
        board_kernel,
        chip,
        core::slice::from_raw_parts(
            &_sapps as *const u8,
            &_eapps as *const u8 as usize - &_sapps as *const u8 as usize,
        ),
        core::slice::from_raw_parts_mut(
            &mut _sappmem as *mut u8,
            &_eappmem as *const u8 as usize - &_sappmem as *const u8 as usize,
        ),
        &mut PROCESSES,
        &FAULT_RESPONSE,
        &process_management_capability,
    )
    .unwrap_or_else(|err| {
        debug!("Error loading processes!");
        debug!("{:?}", err);
    });

    board_kernel.kernel_loop(&microbit, chip, Some(&microbit.ipc), &main_loop_capability);
}
