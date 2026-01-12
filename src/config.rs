use core::panic;
use std::{
    fs::{self, OpenOptions},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
// di sini config config utuk `m-shell` dengan file Toml

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub path_log: String,
    pub path_config: String,

    #[serde(default)]
    pub alias: Option<Alias>,

    #[serde(default)]
    pub plugin: Option<Plugin>,

    #[serde(default)]
    pub keys: Option<Keys>,

    #[serde(default)]
    pub shell: Option<Shell>,

    #[serde(default)]
    pub command_selection: Option<CommandSelection>,
}

// sama dengan isi dari file m-shell.toml
#[derive(Debug, Deserialize, Serialize)]
pub struct Alias {
    pub ls: String, // isi commad untuk ls , misalnyanntil ls = ls -a
    pub clear: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Plugin {
    #[serde(default)]
    _dummy: bool,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Keys {
    pub copy: String,
    pub stop: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Shell {
    // untuk bentuk Shell nya
    pub schema: String,    // yang bagina atas ----------
    pub character: String, // bagian bawah misalnya arrow
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommandSelection {
    pub tui_tools: Vec<String>,
    pub gui_app: Vec<String>,
}

impl Config {
    /// inisialisaisi Struct
    pub fn new() -> Config {
        Config {
            path_log: String::new(), // default kosong
            path_config: String::new(),
            alias: Some(Alias {
                ls: String::new(),
                clear: String::new(),
            }),
            plugin: Some(Plugin { _dummy: false }),
            keys: Some(Keys {
                copy: String::new(),
                stop: String::new(),
            }),
            shell: Some(Shell {
                schema: String::new(),
                character: String::new(),
            }),
            command_selection: Some(CommandSelection {
                tui_tools: Vec::new(),
                gui_app: Vec::new(),
            }),
        }
    }

    /// gunakan untuk mengubah alias cmd dari comand shell
    pub fn update_alias(&self, cmd: &String, alias: &String) {
        let mut config = self.load_config();

        // // pastikan alias ada , jika tidak buat baru | gak perlu lagi karena sudah pasti dengan
        // new()
        // let alias_cfg = config.alias.get_or_insert(Alias {
        //     ls: String::new(),
        //     clear: String::new(),
        // });

        match cmd.as_str() {
            "ls" => {
                // Update alias ls
                config.alias.as_mut().unwrap().ls = alias.clone();
            }

            "clear" => {
                // Update alias clear
                config.alias.as_mut().unwrap().clear = alias.clone();
            }

            _ => {
                // Command tidak dikenal
                eprintln!("Alias `{}` tidak didukung", cmd);
                return;
            }
        }
        self.save_config(&config);
    }

    /// dapatkan scruture shell
    pub fn get_shell(&self) -> Shell {
        let load = self.load_config();
        let shell = load.shell.expect("shell config tidak ditemukan");

        Shell {
            schema: shell.schema,
            character: shell.character,
        }
    }

    /// dapatkan jenis jenis gui app dari m-shell.toml
    pub fn get_gui_apps(&self) -> Vec<String> {
        let load = self.load_config();

        let gui_apps = load
            .command_selection
            .expect("gagal mendapatkan Gui Apps")
            .gui_app;
        gui_apps
    }
    /// dapatkan jenis jenis tui app dari m-shell.toml
    pub fn get_tui_tools(&self) -> Vec<String> {
        let load = self.load_config();

        let tui_tools = load
            .command_selection
            .expect("gagal mendapatkan Gui Tools")
            .gui_app;
        tui_tools
    }

    /// update path confiog dan log , berdasarkan file m-shell.toml
    pub fn update_path(&mut self) {
        let load = self.load_config();

        self.path_config = load.path_config;
        self.path_log = load.path_log;
    }

    /// load config dan m-shell.toml
    fn load_config(&self) -> Config {
        let config = dirs::config_dir().expect("Path tidak di temukan");
        let path = config.join("m-shell.toml");

        if !path.exists() {
            self.default_config(&path);
        }

        let content = fs::read_to_string(&path).expect("Gagal membaca file m-shell.toml");

        toml::from_str(&content).expect("Format TOML tidak valid")
    }

    fn save_config(&self, config: &Config) {
        let toml_str = toml::to_string_pretty(config).expect("Gagal serialize config");

        let config_dir = dirs::config_dir().expect("Tidak bisa menemukan config directory");

        fs::create_dir_all(&config_dir).expect("Gagal membuat config directory");

        let path = config_dir.join("m-shell.toml");

        fs::write(path, toml_str).expect("Gagal menulis config");
    }

    ///create Deafult jika tidak ada di ~/.config/m-shell.toml
    fn default_config(&self, path: &Path) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent) // aman untuk memastikan dir induk ada
                .expect("Gagal membuat directory config");
        }

        let default_conf = include_str!("../default/m-shell.default.toml");

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&path)
            .expect("Gagal membuka atau membuat file toml ");
        file.write_all(default_conf.as_bytes())
            .expect("gagal menulis default file")
    }
}

// Gunakan ini agar sesuai dengan Format Struct , jika kosong atau salah penulisan
// impl fmt::Debug for Config{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Config")
//            .field("path log", &self.path_log)
//             .field("path config", &self.path_config)
//             .finish()
//     }
//
// }
