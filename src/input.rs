use crossterm::event::{self, KeyCode};
use crate::mode::Mode;

pub fn handle_input(
    content: &mut String, 
    cursor_pos: &mut (u16, u16), 
    mode: &mut Mode, 
    in_command_mode: &mut bool, 
    command_buffer: &mut String
) -> std::io::Result<bool> {
    if let event::Event::Key(key_event) = event::read()? {
        if *in_command_mode {
            match key_event.code {
                KeyCode::Enter => {
                    return Ok(true);
                }
                KeyCode::Char(c) => {
                    command_buffer.push(c);
                }
                KeyCode::Backspace => {
                    command_buffer.pop();
                }
                KeyCode::Esc => {
                    *in_command_mode = false;
                    command_buffer.clear();
                }
                _ => {}
            }
            return Ok(false);
        }

        match mode {
            Mode::Normal => match key_event.code {
                KeyCode::Char('i') => *mode = Mode::Insert,
                KeyCode::Char('v') => *mode = Mode::Visual,
                KeyCode::Char(':') => {
                    *in_command_mode = true;
                    command_buffer.push(':');
                }
                _ => {}
            },
            Mode::Insert => match key_event.code {
                KeyCode::Char(c) => {
                    content.push(c);
                    cursor_pos.0 += 1;
                }
                KeyCode::Backspace => {
                    content.pop();
                    if cursor_pos.0 > 0 {
                        cursor_pos.0 -= 1;
                    }
                }
                KeyCode::Esc => *mode = Mode::Normal,
                _ => {}
            },
            Mode::Visual => match key_event.code {
                KeyCode::Esc => *mode = Mode::Normal,
                _ => {}
            },
        }
    }
    Ok(false)
}
