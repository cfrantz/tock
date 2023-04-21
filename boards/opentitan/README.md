OpenTitan RISC-V Board
======================

- https://opentitan.org/

OpenTitan is the first open source project building a transparent,
high-quality reference design and integration guidelines for
silicon root of trust (RoT) chips.

Tock currently supports OpenTitan on the ChipWhisperer
CW310 FPGA board. For more details on the boards see:
https://docs.opentitan.org/doc/ug/fpga_boards/

You can get started with OpenTitan using either the, ChipWhisperer CW310
board or a simulation. See the OpenTitan
[getting started](https://docs.opentitan.org/doc/ug/getting_started/index.html)
for more details.

Programming
-----------

Tock on OpenTitan requires
lowRISC/opentitan@217a0168ba118503c166a9587819e3811eeb0c0c or newer. In
general it is recommended that users start with the latest OpenTitan bitstream
and if that results in issues try the one mentioned above.

For more information you can follow the
[OpenTitan development flow](https://docs.opentitan.org/doc/ug/getting_started_fpga/index.html#testing-the-demo-design)
to flash the image.

First setup the development board using the steps here:
https://docs.opentitan.org/doc/ug/getting_started_fpga/index.html.
You need to make sure the boot ROM is working and that your machine can
communicate with the OpenTitan ROM. You will need to use the `PROG` USB
port on the board for this.

ChipWhisper CW310
-----------------

To use `make flash` you first need to clone the OpenTitan repo and ensure that
the Python dependencies are installed.

```shell
python3 pip install -r python-requirements.txt
```

Then you need to build the OpenTitan tools:

```shell
./bazelisk.sh build //sw/host/opentitantool
```

You might need to run these commands to get it to work

```shell
ln -s /usr/bin/ld.lld /usr/sbin/ld.lld
ln -s /usr/bin/gcc /usr/sbin/gcc
```

Next connect to the board's serial with a second terminal:

```shell
screen /dev/ttyACM1 115200,cs8,-ixon,-ixoff
```

Then you need to flash the bitstream with:


```shell
./bazel-bin/sw/host/opentitantool/opentitantool.runfiles/lowrisc_opentitan/sw/host/opentitantool/opentitantool --interface=cw310 fpga load-bitstream lowrisc_systems_chip_earlgrey_cw310_0.1.bit
```

After which you should see some output in the serial window.

Then in the Tock board directoty export the `OPENTITAN_TREE` environment
variable to point to the OpenTitan tree.

```shell
export OPENTITAN_TREE=/home/opentitan/
```

then you can run `make flash` or `make test-hardware` to use the board.

Verilator
---------

Opentitan is supported on both an FPGA and in Verilator. Slightly different
versions of the EarlGrey chip implementation are required for the different
platforms. By default the kernel is compiled for the FPGA.

## Setting up Verilator

For a full guide see the official [OpenTitan Verilator documentation](https://docs.opentitan.org/doc/ug/getting_started_verilator/)

A quick summary on how to do this is included below though

### Build Boot (test) Rom/OTP Image and FuseSOC

Build **only the targets** we care about. To speed up the building, multi-thread the build process with `--jobs x` where x is the thread count.

```shell
# To build the test-ROM
$ ./bazelisk.sh build //sw/device/lib/testing/test_rom:test_rom

# To build OTP
$ ./bazelisk.sh build //hw/ip/otp_ctrl/...

# To build FuseSOC
$ ./bazelisk.sh build //hw:verilator

# To build OTBN Binary (optional for testing rsa/otbn)
$ ./bazelisk.sh build //sw/device/tests:otbn_rsa_test
```

### Test Verilator

You can use the following to automatically build the relevant targets and run a quick test with

```shell
./bazelisk.sh test --test_output=streamed //sw/device/tests:uart_smoketest_sim_verilator
```

or manually with

```shell

bazel-out/k8-fastbuild/bin/hw/build.verilator_real/sim-verilator/Vchip_sim_tb \
                                    --meminit=rom,./bazel-out/k8-fastbuild-ST-97f470ee3b14/bin/sw/device/lib/testing/test_rom/test_rom_sim_verilator.scr.39.vmem \
                                    --meminit=otp,./bazel-out/k8-fastbuild/bin/hw/ip/otp_ctrl/data/rma_image_verilator.vmem

# Read the output, you want to attach screen to UART, for example
# "UART: Created /dev/pts/4 for uart0. Connect to it with any terminal program, "

screen /dev/pts/4

# Wait a few minutes
# You should eventually see messages in screen
# Once you see "Test ROM complete, jumping to flash!" you know it works, note at this point we haven't provided flash image (so it ends here).
```

At this point Opentitan on Verilator should be ready to go!

### Build and Run Tock

You can also use the Tock Make target to automatically build Tock and run it with Verilator (within `boards/opentitan/earlgrey-cw310`) run:

```shell
make BOARD_CONFIGURATION=sim_verilator verilator
```
The above command should **compile relevant targets and start Verilator simulation**.

However, to manually compile Tock for Verilator, run:

```shell
make BOARD_CONFIGURATION=sim_verilator
```

You will then need to generate a vmem file (must be at the TOP_DIR of tock to execute the following):

```shell
srec_cat \
    target/riscv32imc-unknown-none-elf/release/earlgrey-cw310.bin \
    --binary --offset 0 --byte-swap 8 --fill 0xff \
    -within target/riscv32imc-unknown-none-elf/release/earlgrey-cw310.bin\
    -binary -range-pad 8 --output binary.64.vmem --vmem 64
```

And Verilator can be run with:

TODO: update to reflect changes in opentitan upstream
```shell
${OPENTITAN_TREE}/bazel-out/k8-fastbuild/bin/hw/build.verilator_real/sim-verilator/Vchip_sim_tb \
    --meminit=rom,${OPENTITAN_TREE}/bazel-out/k8-fastbuild-ST-97f470ee3b14/bin/sw/device/lib/testing/test_rom/test_rom_sim_verilator.scr.39.vmem \
    --meminit=flash,./binary.64.vmem \
    --meminit=otp,${OPENTITAN_TREE}/bazel-out/k8-fastbuild/bin/hw/ip/otp_ctrl/data/rma_image_verilator.vmem
````

You can also use the Tock Make target to automatically build Tock and run it with Verilator

```shell
make BOARD_CONFIGURATION=sim_verilator verilator
```

In both cases expect Verilator to run for tens of minutes before you see anything.

Programming Apps
----------------

Tock apps for OpenTitan must be included in the Tock binary file flashed with
the steps mentioned above.

**Apps are built out of tree.**

The OpenTitan Makefile can also handle this process automatically. Follow
the steps above but instead run the `flash-app` make target.

```shell
$ make flash-app APP=<...> OPENTITAN_TREE=/home/opentitan/
```

You will need to have the GCC version of [RISC-V 32-bit objcopy](https://github.com/riscv-collab/riscv-gnu-toolchain/blob/master/README.md) installed as
the LLVM one doesn't support updating sections.

### Programming Apps in Verilator

A **single app** in `.tbf (tock binary format)` can be bundled and loaded with the kernel into Verilator with:

```shell
$ APP=<...> make BOARD_CONFIGURATION=sim_verilator verilator
```

### Libtock-C App Verilator Example

To load a libtock-c app, we can do the following to load the `c_hello` sample app:

**Build app:**
```
$ git clone https://github.com/tock/libtock-c.git
$ cd libtock-c/examples/c_hello/
$ make RISCV=1
```
**Load and Run:**

Now, in the Opentitan board directory in tock (`tock/boards/opentitan/earlgrey-cw310`)
```
$ APP=<PATH_TO_LIBTOCK-C>/examples/c_hello/build/rv32imc/rv32imc.0x20030080.0x10005000.tbf make BOARD_CONFIGURATION=sim_verilator verilator
```
Note: be sure to use the correct `tbf` file here, that is,
> use: `rv32imc.0x20030080.0x10005000.tbf`

as this falls within the supported app flash region (0x20030080) and ram (0x10005000) regions for Opentitan.

**App Output:**

To see the output, when Verilator loads up it will show the endpoint for the
pseudoterminal slave, something like `UART: Created /dev/pts/7 for uart0`.

```
$ screen /dev/pts/7
.
...
OpenTitan initialisation complete. Entering main loop
Hello World!
```

Running in QEMU
---------------

The OpenTitan application can be run in the QEMU emulation platform for
RISC-V, allowing quick and easy testing. This is also a good option for
those who can't afford the FPGA development board.

Unfortunately you need QEMU 7.2, which at the time of writing is unlikely
to be available in your distro. Luckily Tock can build QEMU for you. From
the top level of the Tock source just run `make ci-setup-qemu` and
follow the steps.

QEMU can be started with Tock using the `qemu` make target:

```shell
$ make qemu
```

Where OPENTITAN_BOOT_ROM is set to point to the OpenTitan ELF file. This is
usually located at `sw/device/boot_rom/boot_rom_fpga_nexysvideo.elf` in the
OpenTitan build output. Note that the `make ci-setup-qemu` target will also
download a ROM file.

QEMU can be started with Tock and a userspace app with the `qemu-app` make
target:

```shell
$ make APP=/path/to/app.tbf qemu-app
```

The TBF must be compiled for the OpenTitan board. For example, you can build
the Hello World example app from the libtock-rs repository by running:

```
$ cd [LIBTOCK-RS-DIR]
$ make flash-opentitan
$ tar xf target/riscv32imac-unknown-none-elf/tab/opentitan/hello_world.tab
$ cd [TOCK_ROOT]/boards/opentitan
$ make APP=[LIBTOCK-RS-DIR]/rv32imac.tbf qemu-app
```

QEMU GDB Debugging [**earlgrey-cw310**]
------------------

GDB can be used for debugging with QEMU. This can be useful when debugging a particular application/kernel. 

Start by installing the respective version of gdb.

**Arch**:

```shell
$ sudo pacman -S riscv32-elf-gdb    
```
**Ubuntu**:
```shell
$ sudo apt-get install gdb-multiarch
```

In the board directory, QEMU can be started in a suspended state with gdb ready to be connected. 

```shell
$ make qemu-gdb
```

or with an app ready to be loaded.

```shell
$ make APP=/path/to/app.tbf qemu-app-gdb
```

In a separate shell, start gdb

**Arch**

```shell
$ riscv32-elf-gdb [/path/to/tock.elf]
> target remote:1234            #1234 is the specified default port
```

**Ubuntu**

```shell
$ gdb-multiarch [/path/to/tock.elf]
> set arch riscv
> target remote:1234            #1234 is the specified default port
```

Once attached, standard gdb functionality is available. Additional debug symbols can be added with.
```
add-symbol-file <tock.elf>
add-symbol-file <app.elf>
```

Unit tests
----------
The Tock OpenTitan boards include automated unit tests to test the kernel.

To run the unit tests on QEMU, just run:

```shell
$ make test
```

in the specific board directory.

To run the test on hardware use these commands to build the OTBN binary and run it on hardware:

```shell
$ cd ${OPENTITAN_TREE}
# Build OTBN Binary
$ ./bazelisk.sh build //sw/device/tests:otbn_rsa_test
```

```shell
$ elf2tab --verbose -n "otbn-rsa" --kernel-minor 0 --kernel-major 2 --disable --app-heap 0 --kernel-heap 0 --stack 0 ./bazel-out/k8-fastbuild-.../bin/sw/otbn/crypto/rsa.elf

$ OPENTITAN_TREE=<...> APP=${OPENTITAN_TREE}/build-out/sw/otbn/rsa.tbf make test-hardware
```
### For Verilator
To load the OTBN binary and run it on Verilator, use:
```shell
$ elf2tab --verbose -n "otbn-rsa" --kernel-minor 0 --kernel-major 2 --disable --app-heap 0 --kernel-heap 0 --stack 0 ./bazel-out/k8-fastbuild-ST-2cc462681f62/bin/sw/otbn/crypto/rsa.elf

$ APP=${OPENTITAN_TREE}/bazel-out/k8-fastbuild-.../bin/sw/otbn/crypto/rsa.tbf make BOARD_CONFIGURATION=sim_verilator test-verilator
```

The output on a CW310 should look something like this:

```
OpenTitan initialisation complete. Entering main loop
check run AES128 ECB...
aes_test passed (ECB Enc Src/Dst)
aes_test passed (ECB Dec Src/Dst)
aes_test passed (ECB Enc In-place)
aes_test passed (ECB Dec In-place)
    [ok]
check run AES128 CBC...
aes_test passed (CBC Enc Src/Dst)
aes_test passed (CBC Dec Src/Dst)
aes_test passed (CBC Enc In-place)
aes_test passed (CBC Dec In-place)
    [ok]
check run AES128 CTR...
aes_test CTR passed: (CTR Enc Ctr Src/Dst)
aes_test CTR passed: (CTR Dec Ctr Src/Dst)
    [ok]
check run CSRNG Entropy 32...
Entropy32 test: first get Ok(())
Entropy test: obtained all 8 values. They are:
[00]: 11358ec6
[01]: cad739e8
[02]: 236b897e
[03]: 707c0162
[04]: 2627c579
[05]: 86b6562c
[06]: a8e0e4f8
[07]: 4b298bcd
    [ok]
check hmac load binary...
    [ok]
check hmac check verify...
    [ok]
start multi alarm test...
    [ok]
check otbn run binary...
    [ok]
start TicKV append key test...
---Starting TicKV Tests---
Key: [18, 52, 86, 120, 154, 188, 222, 240] with value [16, 32, 48] was added
Now retrieving the key
Key: [18, 52, 86, 120, 154, 188, 222, 240] with value [16, 32, 48, 0] was retrieved
Removed Key: [18, 52, 86, 120, 154, 188, 222, 240]
Try to read removed key: [18, 52, 86, 120, 154, 188, 222, 240]
Unable to find key: [18, 52, 86, 120, 154, 188, 222, 240]
Let's start a garbage collection
Finished garbage collection
---Finished TicKV Tests---
    [ok]
trivial assertion...
    [ok]
```

The tests can also be run on Verilator with:

```shell
$ make BOARD_CONFIGURATION=sim_verilator test-verilator
```

Note that the Verilator tests can take hours to complete.
