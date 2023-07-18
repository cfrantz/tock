//! Driver for the AT24C32/64 EEPROM memory. Built on top of the I2C interface.
//! Provides interface for the NonvolatileToPages driver.
//!
//! Datasheet:
//! <https://ww1.microchip.com/downloads/en/devicedoc/doc0336.pdf>
//!
//! > The AT24C32/64 provides 32,768/65,536 bits of serial electrically erasable and programmable
//! > read only memory (EEPROM) organized as 4096/8192 words of 8 bits each. The device’s cascadable
//! > feature allows up to 8 devices to share a common 2- wire bus. The device is optimized for use
//! > in many industrial and commercial applica- tions where low power and low voltage operation are
//! > essential. The AT24C32/64 is available in space saving 8-pin JEDEC PDIP, 8-pin JEDEC SOIC,
//! > 8-pin EIAJ SOIC, and 8-pin TSSOP (AT24C64) packages and is accessed via a 2-wire serial
//! > interface.
//!
//! Usage
//! -----
//!
//! ```rust
//! let i2cmux = I2CMuxComponent::new(i2c0, None).finalize(components::i2c_mux_component_static!());
//!
//! let eeprom_buffer = static_init!([u8; 34], [0; 34]);
//!
//! let eeprom_i2c_device = static_init!(I2CDevice, I2CDevice::new(i2cmux, 0x50));
//! let eeprom_capsule = static_init!(capsules_extra::eeprom::EEPROM,capsules_extra::eeprom::EEPROM::new(
//!             eeprom_i2c_device,
//!             eeprom_buffer,
//!         ) );
//! eeprom_i2c_device.set_client(eeprom_capsule);
//!
//! let nonvolatile_storage = components::nonvolatile_storage::NonvolatileStorageComponent::new(
//!         board_kernel,
//!         capsules_extra::nonvolatile_storage_driver::DRIVER_NUM,
//!         eeprom_capsule,
//!         0x0,
//!         0x10000,
//!         0x0,
//!         0x0,
//!     ).finalize(components::nonvolatile_storage_component_static!(capsules_extra::eeprom::EEPROM));
//! ```

use core::cell::Cell;
use core::cmp;

use kernel::hil::i2c::{Error, I2CClient, I2CDevice};
use kernel::utilities::cells::{OptionalCell, TakeCell};
use kernel::{hil, ErrorCode};

const PAGE_SIZE: usize = 32;

pub struct EEPROMPage(pub [u8; PAGE_SIZE]);

impl Default for EEPROMPage {
    fn default() -> Self {
        Self([0; PAGE_SIZE])
    }
}

impl AsMut<[u8]> for EEPROMPage {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

#[derive(Copy, Clone, Debug)]
enum State {
    Idle,
    Reading,
    Writing,
    Erasing,
}

pub struct EEPROM<'a> {
    i2c: &'a dyn I2CDevice,
    buffer: TakeCell<'static, [u8]>,
    client_page: TakeCell<'a, EEPROMPage>,
    flash_client: OptionalCell<&'a dyn hil::flash::Client<EEPROM<'a>>>,
    state: Cell<State>,
}

impl<'a> EEPROM<'a> {
    pub fn new(i2c: &'a dyn I2CDevice, buffer: &'static mut [u8]) -> Self {
        Self {
            i2c,
            buffer: TakeCell::new(buffer),
            client_page: TakeCell::empty(),
            flash_client: OptionalCell::empty(),
            state: Cell::new(State::Idle),
        }
    }

    fn read_sector(
        &self,
        page_number: usize,
        buf: &'static mut EEPROMPage,
    ) -> Result<(), (ErrorCode, &'static mut EEPROMPage)> {
        self.i2c.enable();
        let address = page_number * PAGE_SIZE;
        if let Some(rxbuffer) = self.buffer.take() {
            rxbuffer[0] = ((address >> 8) & 0x00ff) as u8;
            rxbuffer[1] = (address & 0x00ff) as u8;

            self.state.set(State::Reading);
            if let Err((error, local_buffer)) = self.i2c.write_read(rxbuffer, 2, PAGE_SIZE) {
                self.buffer.replace(local_buffer);
                self.i2c.disable();
                Err((error.into(), buf))
            } else {
                self.client_page.replace(buf);
                Ok(())
            }
        } else {
            Err((ErrorCode::RESERVE, buf))
        }
    }

    fn write_sector(
        &self,
        page_number: usize,
        buf: &'static mut EEPROMPage,
    ) -> Result<(), (ErrorCode, &'static mut EEPROMPage)> {
        let address = page_number * PAGE_SIZE;
        let length = buf.0.len();
        // Schedule page write and do first
        if let Some(txbuffer) = self.buffer.take() {
            txbuffer[0] = ((address >> 8) & 0x00ff) as u8;
            txbuffer[1] = (address & 0x00ff) as u8;

            let write_len = cmp::min(txbuffer.len() - 2, length);

            for i in 0..write_len {
                txbuffer[i + 2] = buf.0[i];
            }

            self.state.set(State::Writing);
            if let Err((error, txbuffer)) = self.i2c.write(txbuffer, write_len + 2) {
                self.buffer.replace(txbuffer);
                self.i2c.disable();
                Err((error.into(), buf))
            } else {
                self.client_page.replace(buf);
                Ok(())
            }
        } else {
            Err((ErrorCode::RESERVE, buf))
        }
    }

    fn erase_sector(&self, page_number: usize) -> Result<(), ErrorCode> {
        self.i2c.enable();
        let address = page_number * PAGE_SIZE;
        // Schedule page write and do first
        if let Some(txbuffer) = self.buffer.take() {
            txbuffer[0] = ((address >> 8) & 0x00ff) as u8;
            txbuffer[1] = (address & 0x00ff) as u8;

            let write_len = cmp::min(txbuffer.len() - 2, PAGE_SIZE);

            for i in 0..write_len {
                txbuffer[i + 2] = 0;
            }

            self.state.set(State::Erasing);
            if let Err((error, txbuffer)) = self.i2c.write(txbuffer, write_len + 2) {
                self.buffer.replace(txbuffer);
                self.i2c.disable();
                Err(error.into())
            } else {
                Ok(())
            }
        } else {
            Err(ErrorCode::RESERVE)
        }
    }
}

impl I2CClient for EEPROM<'static> {
    fn command_complete(&self, buffer: &'static mut [u8], status: Result<(), Error>) {
        match self.state.get() {
            State::Reading => {
                self.state.set(State::Idle);
                self.i2c.disable();
                if let Some(client_page) = self.client_page.take() {
                    for i in 0..PAGE_SIZE {
                        client_page.0[i] = buffer[i];
                    }
                    self.buffer.replace(buffer);
                    self.flash_client.map(|client| {
                        if status.is_err() {
                            client.read_complete(client_page, hil::flash::Error::FlashError);
                        } else {
                            client.read_complete(client_page, hil::flash::Error::CommandComplete);
                        }
                    });
                } else {
                    self.buffer.replace(buffer);
                }
            }
            State::Writing => {
                self.state.set(State::Idle);
                self.buffer.replace(buffer);
                self.i2c.disable();
                self.flash_client.map(|client| {
                    if let Some(client_page) = self.client_page.take() {
                        if status.is_err() {
                            client.write_complete(client_page, hil::flash::Error::FlashError);
                        } else {
                            client.write_complete(client_page, hil::flash::Error::CommandComplete);
                        }
                    } else {
                        panic!("EEPROM Writing op completed without client_page");
                    }
                });
            }
            State::Erasing => {
                self.state.set(State::Idle);
                self.buffer.replace(buffer);
                self.i2c.disable();
                self.flash_client.map(move |client| {
                    if status.is_err() {
                        client.erase_complete(hil::flash::Error::FlashError);
                    } else {
                        client.erase_complete(hil::flash::Error::CommandComplete);
                    }
                });
            }
            State::Idle => {}
        }
    }
}

impl<'a> hil::flash::Flash for EEPROM<'a> {
    type Page = EEPROMPage;

    fn read_page(
        &self,
        page_number: usize,
        buf: &'static mut Self::Page,
    ) -> Result<(), (ErrorCode, &'static mut Self::Page)> {
        self.read_sector(page_number, buf)
    }

    fn write_page(
        &self,
        page_number: usize,
        buf: &'static mut Self::Page,
    ) -> Result<(), (ErrorCode, &'static mut Self::Page)> {
        self.write_sector(page_number, buf)
    }

    fn erase_page(&self, page_number: usize) -> Result<(), ErrorCode> {
        self.erase_sector(page_number)
    }
}

impl<'a, C: hil::flash::Client<Self>> hil::flash::HasClient<'a, C> for EEPROM<'a> {
    fn set_client(&'a self, client: &'a C) {
        self.flash_client.set(client);
    }
}
