mod command;
mod events;
mod info;

use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
};

use crossterm::{
    ExecutableCommand, cursor,
    event::{Event, read},
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};

use crate::info::InfoLine;

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
    // simpan informasi kejadian di clic | default = none
    // Arc : karena akan di gunkan di beberapa tempat
    // Mutex dengan Arc agar bisa mutable
    let info_message: Arc<Mutex<Option<InfoLine>>> = Arc::new(Mutex::new(None));

    loop {
        let event: Event = read()?;
        if !events::handle_event(event, &mut command, Arc::clone(&info_message)) {
            break;
        }
        // agar tida di cetak berulang
        io::stdout()
            .execute(Clear(ClearType::CurrentLine))? // hapus baris aktiv
            .execute(cursor::MoveToColumn(0))?; // pindah ke awal baris
        print!("{}", command);
        io::stdout().flush().unwrap(); // agar print langsung muncul 

        // some ref inf : mengambil reference nilai tampa memindahkan nilainya
        if let Some(ref inf) = *info_message.lock().unwrap() {
            println!("{}", inf.display_info());
        }
    }
    restore_terminal()?;

    Ok(())
}
