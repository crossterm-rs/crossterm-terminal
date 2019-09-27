#![deny(unused_imports, unused_must_use)]

#[doc(no_inline)]
pub use crossterm_utils::{execute, queue, Command, ExecutableCommand, QueueableCommand, Result};

pub use self::terminal::{terminal, Clear, ClearType, ScrollDown, ScrollUp, SetSize, Terminal};

mod sys;
mod terminal;
