use std::{borrow::Cow, fmt::format, fs, iter::Enumerate};

use crate::shell_config::get_path_log;

// menggunakn Cow agar flexsibel String atau &str
pub enum InfoLine<'a> {
    ErrorLine(Cow<'a, str>),
    ErrorNotfound(Cow<'a, str>),
    Error(Cow<'a, str>),
    PermisionDenied(Cow<'a, str>),
    RuntimeError(Cow<'a, str>), // kesalaha saat program di jalanakn
}

impl<'a> InfoLine<'a> {
    pub fn display_info(&self) -> String {
        match self {
            InfoLine::ErrorLine(info)
            | InfoLine::Error(info)
            | InfoLine::ErrorNotfound(info)
            | InfoLine::PermisionDenied(info)
            | InfoLine::RuntimeError(info) => {
                format!("{}", info)
            }
        }
    }
    // simpan ke log file nantik
        pub fn wrirte_log(&self ) {
        // let var = get_path_log(config)

        // dapatkan path file dari shell_config.r 

    }
}
