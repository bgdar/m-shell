mod build_in_commands;
mod config;
mod events;
mod info;
mod m_shell_command;
mod style_shell;

use crossterm::{
    ExecutableCommand, cursor,
    event::{Event, read},
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
};

use crate::{config::Config, info::Info};

/// fn handle Inisialisasi terminal (raw mode)
fn setup_terminal() -> io::Result<()> {
    enable_raw_mode().map_err(|e| println!("error saat memulai terminal {}", e));
    Ok(())
}

/// fn handleterminal  mode normal
fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()
}

fn main() -> io::Result<()> {
    setup_terminal()?;
    // simpan char command
    let mut command = String::new();

    let config = Config::new();
    let shell: config::Shell = config.get_shell();

    // simpan informasi kejadian di clic | default = none
    // Arc : karena akan di gunkan di beberapa tempat
    // Mutex dengan Arc agar bisa mutable
    let info_message: Arc<Mutex<Option<Info>>> = Arc::new(Mutex::new(None));

    loop {
        let event: Event = read()?;
        if !events::handle_event(event, &mut command, &config, Arc::clone(&info_message)) {
            break;
        }
        // agar tida di cetak berulang
        io::stdout()
            .execute(Clear(ClearType::CurrentLine))? // hapus baris aktiv
            .execute(cursor::MoveToColumn(0))?; // pindah ke awal baris

        // some ref inf : mengambil reference nilai tampa memindahkan nilainya
        let mut lock = info_message.lock().unwrap();
        if let Some(inf) = lock.take() {
            // ambil nilai dan set None
            inf.display_info();
        }

        // tampilan shell di terminal
        print!("{} {}", shell.character, command);

        io::stdout().flush().unwrap(); // agar print langsung muncul 
    }
    restore_terminal()?;

    Ok(())
}
