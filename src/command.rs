use std::{
    borrow::Cow,
    fs,
    io::{Write, stdout},
    sync::{Arc, Mutex},
};

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};

use crate::info::InfoLine;

#[warn(unused_variables)] // variabel yg tidak di gunakan 
fn ls(info_message: Arc<Mutex<Option<InfoLine>>>) {
    let mut lock = info_message.lock().unwrap();

    // "." : path saat ini
    match fs::read_dir(".") {
        Ok(entrys) => {
            for entry in entrys {
                let entry = entry.unwrap();
                let file_type = entry.file_type().unwrap();

                if file_type.is_dir() {
                    println!("[DIR]  {}", entry.file_name().to_string_lossy());
                } else if file_type.is_file() {
                    println!("[FILE] {}", entry.file_name().to_string_lossy());
                } else {
                    // println!("[???]  {}", entry.file_name().to_string_lossy());
                    *lock = Some(InfoLine::ErrorNotfound(Cow::Borrowed(
                        "Tidak di temukna file dan Dir",
                    )));
                }
            }
        }
        Err(err) => *lock = Some(InfoLine::Error(Cow::Borrowed("gagal membaca directory"))),
    }
}

fn clear(info_message: Arc<Mutex<Option<InfoLine>>>) {
    // Gunakan crossterm::execute secara idiomatik
    let mut out = stdout();

    // Eksekusi clear terminal
    // MoveTo : untuk pindahkan Cursor , 0,0 : ke posisi semua di atas
    if let Err(e) = execute!(out, Clear(ClearType::All), MoveTo(0, 0)) {
        let mut lock = info_message.lock().unwrap();
        let error = format!("Gagal clear: {}", e);
        *lock = Some(InfoLine::Error(Cow::Owned(error)));
        return;
    }

    // Flush output untuk memastikan layar benar-benar terhapus
    if let Err(e) = out.flush() {
        let mut lock = info_message.lock().unwrap();
        let error = format!("Gagal flush: {}", e);
        *lock = Some(InfoLine::Error(Cow::Owned(error)));
    }
}

// pub fn mkdir(path: &str, dirname: &str) -> Result<Box<dyn std::error::Error>> {}

pub fn exit() -> bool {
    return true;
}

//functon utama untuk macching command yang di jalanakna
pub fn exec_commad(cmd: &str, info_message: Arc<Mutex<Option<InfoLine>>>) {
    if cmd == "ls" {
        ls(Arc::clone(&info_message));
    } else if cmd == "clear" {
        clear(Arc::clone(&info_message));
    }
}
