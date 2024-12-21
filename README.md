# Basic Blinky
This repo demonstrates how to use the stm32-metapac with the stm32h5 nucleo board.

## How to Build
```
cargo build
```

## Prepare binary
The binary by default doesn't contain a file extension. STM32CubeProgrammer only accepts files that end with '.elf' or '.bin'.
Until a build script is setup to automatically rename the output file, manually rename it with '.elf'
```
ren .\target\thumbv8m.main-none-eabihf\debug\embassy-blinky embassy-blinky.elf
```

## How to Flash
1. Open STM32CubeProgrammer
2. Using default stlink settings, click connect
3. Go to erasing & programming tab
4. Browse to the file path of the compiled binary
5. Click start programming
