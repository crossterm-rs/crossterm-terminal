![Lines of Code][s7] [![Latest Version][s1]][l1] [![MIT][s2]][l2] [![docs][s3]][l3] [![Join us on Discord][s5]][l5]

# Crossterm Terminal

This crate allows you to perform terminal related actions cross-platform e.g clearing, resizing etc. 
It supports all UNIX and Windows terminals down to Windows 7 (not all terminals are tested see
[Tested Terminals](#tested-terminals) for more info)

`crossterm_terminal` is a sub-crate of the [crossterm](https://crates.io/crates/crossterm) crate. You can use it
directly, but it's **highly recommended** to use the [crossterm](https://crates.io/crates/crossterm) crate with
the `terminal` feature enabled (see [feature flags](https://crossterm-rs.github.io/crossterm/docs/feature_flags.html)
for more info).

## Future

> The `crossterm_terminal` crate code will be moved to the `crossterm` crate (it's reexported there now).
> Date is not set yet, but it doesn't make a lot of sense to start a new project with it. Please, use
> the `crossterm` crate with the `terminal` feature enabled.

Issues in this repository are disabled for the same reason. Please, report all issues in the
[crossterm-rs/crossterm](https://github.com/crossterm-rs/crossterm/issues) repository.
 
## Table of contents:

- [Getting started](#getting-started)
- [Useful links](#useful-links)
- [Features](#features)
- [Examples](#examples)
- [Tested Terminals](#tested-terminals)
- [Authors](#authors)
- [License](#license)

## Getting Started

All examples of how `crossterm_terminal` works can be found in the
[examples](https://github.com/crossterm-rs/examples) repository.

Add the `crossterm_terminal` package to your `Cargo.toml` file.

```
[dependencies]
crossterm_terminal = "0.3"
```

And import the `crossterm_terminal` modules you want to use.

```rust  
pub use crossterm_terminal::{terminal, Terminal, ClearType};
```

### Useful Links

- [Documentation](https://docs.rs/crossterm_terminal/)
- [Crates.io](https://crates.io/crates/crossterm_terminal)
- [Examples](https://github.com/crossterm-rs/examples)

## Features

These are the features of this crate:

- Cross-platform
- Multi-threaded (send, sync)
- Detailed Documentation
- Few Dependencies
- Terminal
    - Clearing (all lines, current line, from cursor down and up, until new line)
    - Scrolling (up, down)
    - Terminal Size (get/set)
    - Exit Current Process

## Command API

My first recommendation is to use the [command API](https://crossterm-rs.github.io/crossterm/docs/command.html)
because this might replace some of the existing API in the future. It is more convenient, faster, and easier to use.

## Examples

The [examples](https://github.com/crossterm-rs/examples) repository has more complete and verbose examples.

```rust 
use crossterm::terminal::{terminal,ClearType};

let mut terminal = terminal();

// Clear all lines in terminal;
terminal.clear(ClearType::All)?;
// Clear all cells from current cursor position down.
terminal.clear(ClearType::FromCursorDown)?;
// Clear all cells from current cursor position down.
terminal.clear(ClearType::FromCursorUp)?;
// Clear current line cells.
terminal.clear(ClearType::CurrentLine)?;
// Clear all the cells until next line.
terminal.clear(ClearType::UntilNewLine)?;

// Get terminal size
let (width, height) = terminal.size()?;
print!("X: {}, y: {}", width, height);

// Scroll down, up 10 lines.
terminal.scroll_down(10)?;
terminal.scroll_up(10)?;

// Set terminal size (width, height)
terminal.set_size(10,10)?;

// exit the current process.
terminal.exit();

// write to the terminal whether you are on the main screen or alternate screen.
terminal.write("Some text\n Some text on new line");
```

## Tested terminals

- Windows Powershell
    - Windows 10 (pro)
- Windows CMD
    - Windows 10 (pro)
    - Windows 8.1 (N)
- Ubuntu Desktop Terminal
    - Ubuntu 17.10
- (Arch, Manjaro) KDE Konsole
- Linux Mint

This crate supports all Unix terminals and windows terminals down to Windows 7 but not all of them have been tested.
If you have used this library for a terminal other than the above list without issues feel free to add it to the above list, I really would appreciate it.

## Authors

* **Timon Post** - *Project Owner & creator*

## License

This project is licensed under the MIT License - see the [LICENSE.md](./LICENSE) file for details

[s1]: https://img.shields.io/crates/v/crossterm_terminal.svg
[l1]: https://crates.io/crates/crossterm_terminal

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: ./LICENSE

[s3]: https://docs.rs/crossterm_terminal/badge.svg
[l3]: https://docs.rs/crossterm_terminal/

[s5]: https://img.shields.io/discord/560857607196377088.svg?logo=discord
[l5]: https://discord.gg/K4nyTDB

[s7]: https://travis-ci.org/crossterm-rs/crossterm.svg?branch=master
