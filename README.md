# gb_emulator

A Game Boy and Game Boy Color emulator written in Rust.

## Features

- Accurate CPU emulation (all instructions and timings)
- Full GPU support (classic monochrome mode and CGB color mode)
- Sound and audio via `cpal`
- Support for MBC0, MBC1, MBC2, MBC3 (with optional RTC), MBC4 cartridges
- Battery-backed save RAM (save files written as `<gamename>.gbsave`)
- Mouse-free, keyboard-driven input

## Installation and Running
1. Place you ROM file in home directory

2. Run in release mode with GUI support:

   ```bash
   cargo run --release -- <rom_file>
   ```



## Controls

| Key            | Action      |
| -------------- | ----------- |
| Z              | Button A    |
| X              | Button B    |
| ↑ ↓ ← →        | D-Pad       |
| Space          | Select      |
| Enter/Return   | Start       |
| Esc            | Quit/window close |

## Save Files

When playing cartridges with battery-backed RAM (MBC1/3/5), progress is automatically saved to:

```
<rom_filename>.gbsave
```

Place your ROM alongside the emulator or provide a full path.

## License

