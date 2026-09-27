use crate::semantic::action::KeyTrigger;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io;
use std::time::Duration;

/// Read a single key event conforming to Biscuit Logic grammar.
pub fn read_key(timeout: Duration) -> io::Result<Option<KeyTrigger>> {
    if event::poll(timeout)? {
        if let Event::Key(KeyEvent {
            code,
            modifiers,
            kind: crossterm::event::KeyEventKind::Press,
            ..
        }) = event::read()?
        {
            if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
                return Ok(Some(KeyTrigger::Esc));
            }

            let trigger = match code {
                KeyCode::Enter => KeyTrigger::Enter,
                KeyCode::Esc => KeyTrigger::Esc,
                KeyCode::Up => KeyTrigger::Up,
                KeyCode::Down => KeyTrigger::Down,
                KeyCode::Left => KeyTrigger::Left,
                KeyCode::Right => KeyTrigger::Right,
                KeyCode::Char(' ') => KeyTrigger::Space,
                KeyCode::Char(c) => KeyTrigger::Char(c),
                _ => return Ok(None),
            };
            return Ok(Some(trigger));
        }
    }
    Ok(None)
}
