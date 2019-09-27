#![deny(unused_imports, unused_must_use)]

use std::fmt;

#[cfg(windows)]
use crossterm_utils::supports_ansi;
#[doc(no_inline)]
pub use crossterm_utils::{execute, queue, Command, ExecutableCommand, QueueableCommand, Result};
use crossterm_utils::{impl_display, write_cout};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use self::terminal::ansi::AnsiTerminal;
#[cfg(windows)]
use self::terminal::winapi::WinApiTerminal;
use self::terminal::Terminal as TerminalTrait;

pub(crate) mod sys;
pub(crate) mod terminal;

/// Enum with the different values to clear the terminal.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum ClearType {
    /// clear all cells in terminal.
    All,
    /// clear all cells from the cursor position downwards in terminal.
    FromCursorDown,
    /// clear all cells from the cursor position upwards in terminal.
    FromCursorUp,
    /// clear current line cells in terminal.
    CurrentLine,
    /// clear all cells from cursor position until new line in terminal.
    UntilNewLine,
}

/// Allows you to preform actions on the terminal.
///
/// # Features:
///
/// - Clearing (all lines, current line, from cursor down and up, until new line)
/// - Scrolling (Up, down)
/// - Get the size of the terminal
/// - Set the size of the terminal
/// - Alternate screen
/// - Raw screen
/// - Exit the current process
///
/// Check `/examples/` in the library for more specific examples.
pub struct Terminal {
    #[cfg(windows)]
    terminal: Box<(dyn TerminalTrait + Sync + Send)>,
    #[cfg(unix)]
    terminal: AnsiTerminal,
}

impl Terminal {
    /// Create new terminal instance whereon terminal related actions can be performed.
    pub fn new() -> Terminal {
        #[cfg(windows)]
        let terminal = if supports_ansi() {
            Box::from(AnsiTerminal::new()) as Box<(dyn TerminalTrait + Sync + Send)>
        } else {
            WinApiTerminal::new() as Box<(dyn TerminalTrait + Sync + Send)>
        };

        #[cfg(unix)]
        let terminal = AnsiTerminal::new();

        Terminal { terminal }
    }

    /// Clear the current cursor by specifying the `ClearType`.
    ///
    /// # Example
    /// ```rust
    /// # use crossterm_terminal as crossterm;
    /// # use crossterm_terminal::terminal;
    /// let mut term = terminal();
    ///
    /// // clear all cells in terminal.
    /// term.clear(crossterm::ClearType::All);
    /// // clear all cells from the cursor position downwards in terminal.
    /// term.clear(crossterm::ClearType::FromCursorDown);
    /// // clear all cells from the cursor position upwards in terminal.
    /// term.clear(crossterm::ClearType::FromCursorUp);
    /// // clear current line cells in terminal.
    /// term.clear(crossterm::ClearType::CurrentLine);
    /// // clear all cells from cursor position until new line in terminal.
    /// term.clear(crossterm::ClearType::UntilNewLine);
    /// ```
    pub fn clear(&self, clear_type: ClearType) -> Result<()> {
        self.terminal.clear(clear_type)
    }

    /// Get the terminal size `(x,y)`.
    pub fn size(&self) -> Result<(u16, u16)> {
        self.terminal.size()
    }

    /// Scroll `n` lines up in the current terminal.
    ///
    /// # Parameter
    /// - `count`: the number of rows should be shifted up.
    pub fn scroll_up(&self, count: u16) -> Result<()> {
        self.terminal.scroll_up(count)
    }

    /// Scroll `n` lines down in the current terminal.
    ///
    /// # Parameter
    /// - `count`: the number of rows should be shifted down.
    pub fn scroll_down(&self, count: u16) -> Result<()> {
        self.terminal.scroll_down(count)
    }

    /// Set the terminal size. Note that not all terminals can be set to a very small scale.
    ///
    /// ```rust
    /// # use crossterm_terminal::terminal;
    /// let mut term = terminal();
    ///
    /// // Set of the size to X: 10 and Y: 10
    /// let size = term.set_size(10,10);
    /// ```
    pub fn set_size(&self, width: u16, height: u16) -> Result<()> {
        self.terminal.set_size(width, height)
    }

    // TODO - Marked as no_run, because it's failing on Travis CI
    /// Exit the current process.
    ///
    /// ```no_run
    /// # use crossterm_terminal::terminal;
    /// let mut term = terminal();
    ///
    /// let size = term.exit();
    /// ```
    pub fn exit(&self) {
        crate::sys::exit();
    }

    /// Write any displayable content to the current terminal screen.
    ///
    /// ```rust
    /// # use crossterm_terminal::terminal;
    /// let mut term = terminal();
    ///
    /// let size = term.write("Some text \n Some text on new line");
    /// ```
    ///
    /// This will also flush the standard output.
    pub fn write<D: fmt::Display>(&self, value: D) -> Result<usize> {
        write_cout!(format!("{}", value))
    }
}

/// Get a `Terminal` instance whereon terminal related actions can be performed.
pub fn terminal() -> Terminal {
    Terminal::new()
}

/// When executed, this command will scroll up the terminal buffer by the given number of times.
///
/// See `crossterm/examples/command.rs` for more information on how to execute commands.
pub struct ScrollUp(pub u16);

impl Command for ScrollUp {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        terminal::ansi::scroll_up_csi_sequence(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WinApiTerminal::new().scroll_up(self.0)
    }
}

/// When executed, this command will scroll down the terminal buffer by the given number of times.
///
/// See `crossterm/examples/command.rs` for more information on how to execute commands.
pub struct ScrollDown(pub u16);

impl Command for ScrollDown {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        terminal::ansi::scroll_down_csi_sequence(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WinApiTerminal::new().scroll_down(self.0)
    }
}

/// When executed, this command will clear the terminal buffer based on the type provided.
///
/// See `crossterm/examples/command.rs` for more information on how to execute commands.
pub struct Clear(pub ClearType);

impl Command for Clear {
    type AnsiType = &'static str;

    fn ansi_code(&self) -> Self::AnsiType {
        match self.0 {
            ClearType::All => {
                return terminal::ansi::CLEAR_ALL_CSI_SEQUENCE;
            }
            ClearType::FromCursorDown => {
                return terminal::ansi::CLEAR_FROM_CURSOR_DOWN_CSI_SEQUENCE;
            }
            ClearType::FromCursorUp => {
                return terminal::ansi::CLEAR_FROM_CURSOR_UP_CSI_SEQUENCE;
            }
            ClearType::CurrentLine => return terminal::ansi::CLEAR_FROM_CURRENT_LINE_CSI_SEQUENCE,
            ClearType::UntilNewLine => return terminal::ansi::CLEAR_UNTIL_NEW_LINE_CSI_SEQUENCE,
        }
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WinApiTerminal::new().clear(self.0.clone())
    }
}

/// When executed, this command will set the terminal sie to the given (`width` and `height`)
///
/// See `crossterm/examples/command.rs` for more information on how to execute commands.
pub struct SetSize(pub u16, pub u16);

impl Command for SetSize {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        terminal::ansi::set_size_csi_sequence(self.0, self.1)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WinApiTerminal::new().set_size(self.0, self.1)
    }
}

impl_display!(for ScrollUp);
impl_display!(for ScrollDown);
impl_display!(for SetSize);
impl_display!(for Clear);
