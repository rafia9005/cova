use crossterm::{execute, terminal};
use std::io::{self};
use std::time::{Duration, Instant};
use crate::mode::Mode;
use crate::input::handle_input;
use crate::rendering::render;
use crate::command::{handle_command, render_command};

pub fn run() -> std::io::Result<()> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen)?;

    let mut content = String::new();
    let mut cursor_pos = (0, 0);
    let mut mode = Mode::Normal;
    let mut in_command_mode = false;
    let mut command_buffer = String::new();

    let mut show_cursor = true;
    let mut last_blink = Instant::now();
    let blink_duration = Duration::from_millis(500);

    loop {
        render(&mut stdout, &content, cursor_pos, &mode, show_cursor, &command_buffer)?;
        render_command(&mut stdout, &command_buffer)?;

        if last_blink.elapsed() >= blink_duration {
            show_cursor = !show_cursor;
            last_blink = Instant::now();
        }

        if handle_input(&mut content, &mut cursor_pos, &mut mode, &mut in_command_mode, &mut command_buffer)? {
            if in_command_mode {
                if let Ok(true) = handle_command(&command_buffer) {
                    break;
                }
                in_command_mode = false;
                command_buffer.clear();
            }
        }
    }

    execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
