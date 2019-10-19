![Lines of Code][s7] [![Latest Version][s1]][l1] [![MIT][s2]][l2] [![docs][s3]][l3] [![Join us on Discord][s5]][l5]

# Crossterm Terminal

**The `crossterm_terminal` crate is deprecated and no longer maintained. The GitHub repository will
be archived soon. All the code is being moved to the `crossterm`
[crate](https://github.com/crossterm-rs/crossterm). You can learn more in the
[Merge sub-crates to the crossterm crate](https://github.com/crossterm-rs/crossterm/issues/265)
issue.**

This crate allows you to perform terminal related actions cross-platform e.g clearing, resizing etc. 
It supports all UNIX and Windows terminals down to Windows 7 (not all terminals are tested, see
[Tested Terminals](https://github.com/crossterm-rs/crossterm/blob/master/README.md#tested-terminals) for more info).

`crossterm_terminal` is a sub-crate of the [crossterm](https://crates.io/crates/crossterm) crate. You can use it
directly, but it's **highly recommended** to use the [crossterm](https://crates.io/crates/crossterm) crate with
the `terminal` feature enabled.

## Future

> The `crossterm_terminal` crate code will be moved to the `crossterm` crate (it's reexported there now).
> Date is not set yet, but it doesn't make a lot of sense to start a new project with it. Please, use
> the `crossterm` crate with the `terminal` feature enabled.

Issues in this repository are disabled for the same reason. Please, report all issues in the
[crossterm-rs/crossterm](https://github.com/crossterm-rs/crossterm/issues) repository.
 
## Features

- Cross-platform
- Multi-threaded (send, sync)
- Detailed Documentation
- Few Dependencies
- Terminal
  - Clear (all lines, current line, from cursor down and up, until new line)
  - Scroll up, down
  - Set/get the terminal size
  - Exit current process

## Getting Started

<details>
<summary>
Click to show Cargo.toml.
</summary>

```toml
[dependencies]
# All crossterm features are enabled by default.
crossterm = "0.11"
```

</details>
<p></p>

```rust
use std::io::{stdout, Write};  
use crossterm::{execute, SetSize, ScrollUp, Result};

fn main() -> Result<()> {
    execute!(stdout(), SetSize(10, 10), ScrollUp(5))
}
```

## Other Resources

- [API documentation](https://docs.rs/crossterm_terminal/) (with other examples)
- [Examples repository](https://github.com/crossterm-rs/examples)

## Authors

* **Timon Post** - *Project Owner & creator*

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details

[s1]: https://img.shields.io/crates/v/crossterm_terminal.svg
[l1]: https://crates.io/crates/crossterm_terminal

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: ./LICENSE

[s3]: https://docs.rs/crossterm_terminal/badge.svg
[l3]: https://docs.rs/crossterm_terminal/

[s5]: https://img.shields.io/discord/560857607196377088.svg?logo=discord
[l5]: https://discord.gg/K4nyTDB

[s7]: https://travis-ci.org/crossterm-rs/crossterm.svg?branch=master
