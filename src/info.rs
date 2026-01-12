use std::{
    borrow::Cow,
    sync::{Arc, Mutex},
};

// menggunakn Cow agar flexsibel String atau &str
pub enum MessageType {
    ErrorLine,
    ErrorNotfound,
    Error,
    PermisionDenied,
    RuntimeError, // kesalaha saat program di jalanakn

    INFO,
    SUCCESS,
}

pub struct Info<'a> {
    pub message: Cow<'a, str>,
    pub message_type: MessageType,
}

impl<'a> Info<'a> {
    /// tampilkan aja langsung
    pub fn display_info(&self) {
        match self.message_type {
            MessageType::ErrorLine
            | MessageType::Error
            | MessageType::ErrorNotfound
            | MessageType::PermisionDenied
            | MessageType::RuntimeError
            | MessageType::SUCCESS
            | MessageType::INFO => {
                println!("{}", self.message)
            }
        }
    }
    // simpan ke log file nantik
    pub fn wrirte_log(&self) {
        // let var = get_path_log(config)

        // dapatkan path file dari shell_config.r
    }

    // METHOD yang di panggil langsung

    /// update message Global nya
    pub fn set_message_info<'b>(
        info_message: Arc<Mutex<Option<Info<'b>>>>,
        message: Cow<'b, str>,
        message_type: MessageType,
    ) {
        let mut lock = info_message.lock().unwrap();
        // atur warna atau apapaun nantik untuk command command nya
        match message_type {
            // *lock = Some(InfoLine::Error(Cow::Owned(error))); // cara lama
            MessageType::ErrorNotfound => {
                *lock = Some(Info {
                    message: message,
                    message_type: message_type,
                });
            }
            MessageType::RuntimeError => {
                *lock = Some(Info {
                    message: message,
                    message_type: message_type,
                });
            }
            MessageType::ErrorNotfound => {
                *lock = Some(Info {
                    message: message,
                    message_type: message_type,
                });
            }
            MessageType::Error => {
                *lock = Some(Info {
                    message: message,
                    message_type: message_type,
                });
            }

            MessageType::INFO => {
                *lock = Some(Info {
                    message: message,
                    message_type: message_type,
                });
            }
            MessageType::SUCCESS => {
                *lock = Some(Info {
                    message: message,
                    message_type: message_type,
                });
            }

            _ => {
                *lock = None;
            }
        }
    }
    pub fn clean_message_info(info_message: Arc<Mutex<Option<Info>>>) {
        let mut lock = info_message.lock().unwrap();
        *lock = None // None kembali 
    }
}
