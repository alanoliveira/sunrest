# Sunrest

Sunrest is a NES emulator written with the purpose of learning Rust and understanding how an 
emulator works.

## Usage

```sh
sunrest <rom path>
```

**The emulation is not accurate, games might display various glitches**

### Buttons

| Keyboard Key | Nes Pad |
| ------------ | ------- |
| A            | Left    |
| S            | Down    |
| D            | Right   |
| W            | Up      |
| J            | A       |
| K            | B       |
| Enter        | start   |
| Backspace    | select  |


### Settings

The following environment variables can be used to configure the emulator:

| env            | description                         |
| -------------- | ----------------------------------- |
| SUNREST_SPEED  | emulator speed ratio (default: 1.0) |
| SUNREST_VOLUME | audio volume (default: 1.0)         |

### Supported Roms

- Only the iNes v1 format (`.nes`) is supported for ROMs.

The implemented mappers and the list supported ROMs for each mapper are:

| mapper | rom list                                     |
| ------ | -------------------------------------------- |
| 000    | https://nescartdb.com/search/advanced?ines=0 |
| 001    | https://nescartdb.com/search/advanced?ines=1 |
| 002    | https://nescartdb.com/search/advanced?ines=2 |
| 003    | https://nescartdb.com/search/advanced?ines=3 |
| 004    | https://nescartdb.com/search/advanced?ines=4 |


## Building

To ensure optimal performance, make sure to build the emulator with the `--release` flag enabled.
Without this flag, the emulator may run too slowly.

```sh
cargo build --release
```

## Tests

There are some optional tests (marked as `ignore`) relying on test ROMs available at
[nes-test-roms](https://github.com/christopherpow/nes-test-roms).  
To run these tests, follow the steps below:

1. Clone the [nes-test-roms repository](https://github.com/christopherpow/nes-test-roms) to your local machine.
2. Set the `NES_TEST_ROMS_PATH` environment variable to the path where you cloned the `nes-test-roms` repository.
3. Run the tests, ensuring that you enable the **ignored** tests.

```sh
git clone https://github.com/christopherpow/nes-test-roms $HOME/nes-test-roms
NES_TEST_ROMS_PATH=/$HOME/nes-test-roms cargo test -- --include-ignored
```

## Acknowledgments

While creating this emulator, I heavily consulted the following sources:

- https://www.nesdev.org/ - The bible
- https://www.masswerk.at/6502/6502_instruction_set.html - 6502 instruction set documentation
- javidx9's [NES Emulator From Scratch](https://www.youtube.com/playlist?list=PLrOv9FMX8xJHqMvSGB_9G9nZZ_4IgteYf) Youtube series
- https://github.com/ulfalizer/nesalizer - A NES emulator written in C++
- https://github.com/fogleman/nes - A NES emulator written in Go
