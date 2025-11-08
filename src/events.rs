use std::sync::{Arc, Mutex};

use crossterm::event::{Event, KeyCode};

use crate::{command::exec_commad, info::InfoLine};

// use crate::{comman, command};

/// function untuk handle input key
/// ! dia akan di pangil di berulangan jadi jika ada suatu variabel maka akn ke reset
pub fn handle_event(
    event: Event,
    command: &mut String,
    info_message: Arc<Mutex<Option<InfoLine>>>,
) -> bool {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Enter => {
                if command == "exit" || command == "quit" {
                    return false;
                }
                if !command.is_empty() {
                    exec_commad(&command, Arc::clone(&info_message));
                }
                command.clear();
            }
            KeyCode::Backspace => {
                command.pop(); //ambil caracter nya 
            }
            KeyCode::Char(c) => {
                // tampilkan tulisanya untuk setiap chard yang di dapat
                command.push(c);
            }
            //abaikan yang lain
            _ => {}
        }
    }
    true // agar terus loop  
}
