use std::{
    io::{Cursor, stdout},
    sync::{Arc, Mutex},
};

use crossterm::{
    ExecutableCommand, cursor,
    event::{Event, KeyCode},
};

use crate::{build_in_commands::BuildInCommands, config::Config, info::Info};

//functon utama untuk macching command yang di jalanakna
// pub fn exec_commad(cmd: &str, shell_config: Config, info_message: Arc<Mutex<Option<InfoLine>>>) {
//     let commads = Commands {
//         info_message: info_message,
//     };
//
//     if cmd == "ls" {
//         commads::<'_>::ls(Arc::clone(&info_message));
//     } else if cmd == "clear" {
//         commads.clear(Arc::clone(&info_message));
//     }
// }
/// function untuk handle input key
/// ! dia akan di pangil di berulangan jadi jika ada suatu variabel maka akn ke reset
pub fn handle_event(
    event: Event,
    command: &mut String,
    config: &Config,
    info_message: Arc<Mutex<Option<Info>>>,
) -> bool {
    // sementara aja
    let apps: Vec<String> = config.get_gui_apps();
    let tools: Vec<String> = config.get_tui_tools();

    let build_in_commands = BuildInCommands::new(Arc::clone(&info_message), tools, apps);

    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Enter => {
                if command.trim().is_empty() {
                    command.clear();
                    // enter
                    stdout().execute(cursor::MoveToNextLine(1)).unwrap();
                    return true;
                }

                if command == "exit" || command == "quit" {
                    command.clear();
                    return false;
                }

                if build_in_commands.is_command(command, "cd") {
                    build_in_commands.change_dir(command);
                }

                build_in_commands.build_in_commands(command);
                command.clear();
            }

            KeyCode::Backspace => {
                command.pop();
            }
            KeyCode::Char(c) => {
                command.push(c);
            }

            _ => {}
        }
    }
    true // agar terus loop  
}
