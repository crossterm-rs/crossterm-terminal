//! This is a `WINAPI` specific implementation for terminal related action.
//! This module is used for non supporting `ANSI` windows terminals.
//!
//! Windows versions lower then windows 10 are not supporting ANSI codes. Those versions
//! will use this implementation instead.

use crossterm_cursor::TerminalCursor;
use crossterm_utils::{ErrorKind, Result};
use crossterm_winapi::{Console, Coord, Handle, ScreenBuffer, Size};

use crate::sys::winapi::get_terminal_size;

use super::{ClearType, Terminal};

/// This struct is a winapi implementation for terminal related actions.
pub(crate) struct WinApiTerminal;

impl WinApiTerminal {
    pub(crate) fn new() -> Box<WinApiTerminal> {
        Box::from(WinApiTerminal {})
    }
}

impl Terminal for WinApiTerminal {
    fn clear(&self, clear_type: ClearType) -> Result<()> {
        let screen_buffer = ScreenBuffer::current()?;
        let csbi = screen_buffer.info()?;

        let pos = csbi.cursor_pos();
        let buffer_size = csbi.buffer_size();
        let current_attribute = csbi.attributes();

        match clear_type {
            ClearType::All => {
                clear_entire_screen(buffer_size, current_attribute)?;
            }
            ClearType::FromCursorDown => clear_after_cursor(pos, buffer_size, current_attribute)?,
            ClearType::FromCursorUp => clear_before_cursor(pos, buffer_size, current_attribute)?,
            ClearType::CurrentLine => clear_current_line(pos, buffer_size, current_attribute)?,
            ClearType::UntilNewLine => clear_until_line(pos, buffer_size, current_attribute)?,
        };
        Ok(())
    }

    fn size(&self) -> Result<(u16, u16)> {
        get_terminal_size()
    }

    fn scroll_up(&self, count: u16) -> Result<()> {
        let csbi = ScreenBuffer::current()?;
        let mut window = csbi.info()?.terminal_window();

        // Check whether the window is too close to the screen buffer top
        let count = count as i16;
        if window.top >= count {
            window.top -= count; // move top down
            window.bottom = count; // move bottom down

            Console::new()?.set_console_info(false, window)?;
        }
        Ok(())
    }

    fn scroll_down(&self, count: u16) -> Result<()> {
        let screen_buffer = ScreenBuffer::current()?;
        let csbi = screen_buffer.info()?;
        let mut window = csbi.terminal_window();
        let buffer_size = csbi.buffer_size();

        // Check whether the window is too close to the screen buffer top
        let count = count as i16;
        if window.bottom < buffer_size.height - count {
            window.top += count; // move top down
            window.bottom += count; // move bottom down

            Console::new()?.set_console_info(false, window)?;
        }
        Ok(())
    }

    /// Set the current terminal size
    fn set_size(&self, width: u16, height: u16) -> Result<()> {
        if width <= 0 {
            return Err(ErrorKind::ResizingTerminalFailure(String::from(
                "Cannot set the terminal width lower than 1",
            )));
        }

        if height <= 0 {
            return Err(ErrorKind::ResizingTerminalFailure(String::from(
                "Cannot set the terminal height lower then 1",
            )));
        }

        // Get the position of the current console window
        let screen_buffer = ScreenBuffer::current()?;
        let console = Console::from(**screen_buffer.handle());
        let csbi = screen_buffer.info()?;

        let current_size = csbi.buffer_size();
        let window = csbi.terminal_window();

        let mut new_size = Size::new(current_size.width, current_size.height);

        // If the buffer is smaller than this new window size, resize the
        // buffer to be large enough.  Include window position.
        let mut resize_buffer = false;

        let width = width as i16;
        if current_size.width < window.left + width {
            if window.left >= i16::max_value() - width {
                return Err(ErrorKind::ResizingTerminalFailure(String::from(
                    "Argument out of range when setting terminal width.",
                )));
            }

            new_size.width = window.left + width;
            resize_buffer = true;
        }
        let height = height as i16;
        if current_size.height < window.top + height {
            if window.top >= i16::max_value() - height {
                return Err(ErrorKind::ResizingTerminalFailure(String::from(
                    "Argument out of range when setting terminal height.",
                )));
            }

            new_size.height = window.top + height;
            resize_buffer = true;
        }

        if resize_buffer {
            if let Err(_) = screen_buffer.set_size(new_size.width - 1, new_size.height - 1) {
                return Err(ErrorKind::ResizingTerminalFailure(String::from(
                    "Something went wrong when setting screen buffer size.",
                )));
            }
        }

        let mut window = window.clone();
        // Preserve the position, but change the size.
        window.bottom = window.top + height - 1;
        window.right = window.left + width - 1;
        console.set_console_info(true, window)?;

        // If we resized the buffer, un-resize it.
        if resize_buffer {
            if let Err(_) = screen_buffer.set_size(current_size.width - 1, current_size.height - 1)
            {
                return Err(ErrorKind::ResizingTerminalFailure(String::from(
                    "Something went wrong when setting screen buffer size.",
                )));
            }
        }

        let bounds = console.largest_window_size();

        if width > bounds.x {
            return Err(ErrorKind::ResizingTerminalFailure(format!(
                "Argument width: {} out of range when setting terminal width.",
                width
            )));
        }
        if height > bounds.y {
            return Err(ErrorKind::ResizingTerminalFailure(format!(
                "Argument height: {} out of range when setting terminal height",
                width
            )));
        }

        Ok(())
    }
}

fn clear_after_cursor(location: Coord, buffer_size: Size, current_attribute: u16) -> Result<()> {
    let (mut x, mut y) = (location.x, location.y);

    // if cursor position is at the outer right position
    if x as i16 > buffer_size.width {
        y += 1;
        x = 0;
    }

    // location where to start clearing
    let start_location = Coord::new(x, y);

    // get sum cells before cursor
    let cells_to_write = buffer_size.width as u32 * buffer_size.height as u32;

    clear(start_location, cells_to_write, current_attribute)
}

fn clear_before_cursor(location: Coord, buffer_size: Size, current_attribute: u16) -> Result<()> {
    let (xpos, ypos) = (location.x, location.y);

    // one cell after cursor position
    let x = 0;
    // one at row of cursor position
    let y = 0;

    // location where to start clearing
    let start_location = Coord::new(x, y);

    // get sum cells before cursor
    let cells_to_write = (buffer_size.width as u32 * ypos as u32) + (xpos as u32 + 1);

    // clear everything before cursor position
    clear(start_location, cells_to_write, current_attribute)
}

fn clear_entire_screen(buffer_size: Size, current_attribute: u16) -> Result<()> {
    // get sum cells before cursor
    let cells_to_write = buffer_size.width as u32 * buffer_size.height as u32;

    // location where to start clearing
    let start_location = Coord::new(0, 0);

    // clear the entire screen
    clear(start_location, cells_to_write, current_attribute)?;

    // put the cursor back at cell 0,0
    let cursor = TerminalCursor::new();
    cursor.goto(0, 0)?;
    Ok(())
}

fn clear_current_line(location: Coord, buffer_size: Size, current_attribute: u16) -> Result<()> {
    // location where to start clearing
    let start_location = Coord::new(0, location.y);

    // get sum cells before cursor
    let cells_to_write = buffer_size.width as u32;

    // clear the whole current line
    clear(start_location, cells_to_write, current_attribute)?;

    // put the cursor back at cell 1 on current row
    let cursor = TerminalCursor::new();
    cursor.goto(0, location.y as u16)?;
    Ok(())
}

fn clear_until_line(location: Coord, buffer_size: Size, current_attribute: u16) -> Result<()> {
    let (x, y) = (location.x, location.y);

    // location where to start clearing
    let start_location = Coord::new(x, y);

    // get sum cells before cursor
    let cells_to_write = (buffer_size.width - x as i16) as u32;

    // clear until the current line
    clear(start_location, cells_to_write, current_attribute)?;

    // put the cursor back at original cursor position before we did the clearing
    let cursor = TerminalCursor::new();
    cursor.goto(x as u16, y as u16)?;
    Ok(())
}

fn clear(start_location: Coord, cells_to_write: u32, current_attribute: u16) -> Result<()> {
    let console = Console::from(Handle::current_out_handle()?);
    console.fill_whit_character(start_location, cells_to_write, ' ')?;
    console.fill_whit_attribute(start_location, cells_to_write, current_attribute)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Terminal, WinApiTerminal};

    #[test]
    fn test_resize_winapi() {
        let terminal = WinApiTerminal::new();

        let (width, height) = terminal.size().unwrap();

        terminal.set_size(30, 30).unwrap();
        assert_eq!((30, 30), terminal.size().unwrap());

        // reset to previous size
        terminal.set_size(width, height).unwrap();
        assert_eq!((width, height), terminal.size().unwrap());
    }
}
