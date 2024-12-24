use crossterm::{cursor, execute, style, terminal};
use std::io::{Write};
use crate::mode::Mode;

pub fn render(
    stdout: &mut impl Write, 
    content: &str, 
    cursor_pos: (u16, u16), 
    mode: &Mode, 
    show_cursor: bool, 
    command_buffer: &str
) -> std::io::Result<()> {
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    write!(stdout, "{}", content)?;

    if show_cursor {
        execute!(stdout, cursor::MoveTo(cursor_pos.0, cursor_pos.1))?;
        write!(stdout, "|")?;
    }

    let (cols, rows) = terminal::size()?;
    execute!(stdout, cursor::MoveTo(0, rows - 1))?;

    let mode_str = match mode {
        Mode::Normal => " NORMAL ",
        Mode::Insert => " INSERT ",
        Mode::Visual => " VISUAL ",
    };

    let mode_color = match mode {
        Mode::Normal => style::Color::Green,
        Mode::Insert => style::Color::Blue,
        Mode::Visual => style::Color::Yellow,
    };

    let padding = " ".repeat((cols as usize).saturating_sub(mode_str.len()));
    write!(
        stdout,
        "{}{}{}{}{}",
        style::SetForegroundColor(mode_color),
        style::Attribute::Reverse,
        mode_str,
        style::ResetColor,
        padding
    )?;

    if !command_buffer.is_empty() {
        execute!(stdout, cursor::MoveTo(0, rows - 2))?;
        write!(stdout, "{}", command_buffer)?;
    }

    execute!(stdout, style::ResetColor)?;

    stdout.flush()?;
    Ok(())
}
