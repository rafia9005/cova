use std::io::{self, Write};

pub fn handle_command(command_buffer: &str) -> io::Result<bool> {
    match command_buffer {
        ":q" => Ok(true),
        _ => Ok(false),
    }
}

pub fn render_command(stdout: &mut impl Write, command_buffer: &str) -> io::Result<()> {
    if !command_buffer.is_empty() {
        let (rows) = crossterm::terminal::size()?;
        crossterm::execute!(stdout, crossterm::cursor::MoveTo(0, rows - 2))?;
        write!(stdout, "{}", command_buffer)?;
        stdout.flush()?;
    }
    Ok(())
}
