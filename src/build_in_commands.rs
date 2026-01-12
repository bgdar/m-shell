// commad commad yang hanya atau cuman 1 kali di execute

use core::{error, str};
use std::{
    env,
    io::{Write, stdout},
    path::PathBuf,
    process::{Child, Command},
    string,
    sync::{Arc, Mutex},
};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::info::{Info, MessageType};

#[derive(Default)]
pub struct BuildInCommands<'a> {
    cwd: Option<PathBuf>, //current path saat ini
    tui_tools: Vec<String>,
    gui_apps: Vec<String>,
    pub info_message: Arc<Mutex<Option<Info<'a>>>>,
}

impl<'a> BuildInCommands<'a> {
    pub fn new(
        info_message: Arc<Mutex<Option<Info<'a>>>>,
        tui_tools: Vec<String>,
        gui_apps: Vec<String>,
    ) -> Self {
        BuildInCommands {
            cwd: Some(env::current_dir().expect("gagal mendapatkan folder saat ini ")),
            info_message: info_message,
            tui_tools: tui_tools,
            gui_apps: gui_apps,
        }
    }

    /// manfatin Commad dari roces untuk execusi ;angsu ke terminal
    pub fn build_in_commands(&self, command: &str) {
        if command.trim().is_empty() {
            return;
        }

        //sebelumnya menggunakan contains untuk pengecekan
        //( tapi perlu type yang sama , jika di convert akan boros memory karena sering alokasi)
        if self.tui_tools.iter().any(|a| a == command.trim()) {
            let _ = self.run_tui_command(command.trim());
        } else if self.gui_apps.iter().any(|a| a == command.trim()) {
            let _ = self.run_gui_command(command.trim());
        } else {
            match self.run_base_command(command.trim()) {
                Ok(cmd) => {
                    Info::set_message_info(
                        Arc::clone(&self.info_message),
                        std::borrow::Cow::Owned(cmd),
                        MessageType::INFO,
                    );
                }
                Err(_) => {
                    Info::set_message_info(
                        Arc::clone(&self.info_message),
                        std::borrow::Cow::Borrowed("Gagal menjalankan Command"),
                        MessageType::INFO,
                    );
                }
            }
        }
    }
    ///CD , tidak bisa di dukung secara langsung dengan process::Command
    pub fn change_dir(&self, command: &str) {
        let parts = command.trim().split_whitespace();
        let args: Vec<&str> = parts.collect();

        if let Some(dir) = args.get(1) {
            if let Err(_) = env::set_current_dir(dir) {
                Info::set_message_info(
                    Arc::clone(&self.info_message),
                    std::borrow::Cow::Borrowed("Gagal Berpindah Directory"),
                    MessageType::INFO,
                );
            }
        }
    }

    /// spaw aplikasi dan rus akan menunggu
    fn run_gui_command(&self, command: &str) -> Result<(), Box<dyn error::Error>> {
        let first_word = command.split_whitespace().next().unwrap_or("");

        if self.gui_apps.iter().any(|a| a == first_word) {
            // spawn GUI app tanpa wait

            disable_raw_mode()?;
            #[cfg(target_os = "windows")]
            {
                Command::new("cmd").args(["/C", command]).spawn()?;
            }

            #[cfg(not(target_os = "windows"))]
            {
                Command::new("sh").args(["-c", command]).spawn()?;
            }

            enable_raw_mode()?;
        }
        Ok(())
    }

    /// jalnakn aplikasi TUI yang interaktif perlu di tunggu
    fn run_tui_command(&self, command: &str) -> Result<(), Box<dyn error::Error>> {
        // Tools interaktif yang perlu terminal langsung
        // let interactive_tools = ["top", "htop", "btop"]; // bisa tambah sesuai kebutuhan
        let first_word = command.split_whitespace().next().unwrap_or("");

        if self.tui_tools.iter().any(|a| a == first_word) {
            // Spawn child process dengan inherit agar tampil langsung

            //keluar dari raw mode agar bisa di execusi | biar tidak crash
            disable_raw_mode()?;
            #[cfg(target_os = "windows")]
            {
                let mut child = Command::new("cmd")
                    .args(["/C", command])
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()?;

                child.wait()?;
            }

            #[cfg(not(target_os = "windows"))]
            {
                use std::process::Stdio;

                let mut child = Command::new("sh")
                    .args(["-c", command])
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()?;

                child.wait()?;
            }

            enable_raw_mode()?;
        }
        Ok(())
    }

    /// jalankan commad Unix || Windows
    fn run_base_command(&self, command: &str) -> std::io::Result<String> {
        // dengan ini bisa langsung menjalanakn commad tampa perlu split , jadi ni menjalankan
        // commad tungggla sesui OS
        // ls : bisa
        // ls -a | grep word : bisa juga

        // tapi ni kurang flexsible

        // Command biasa → capture output
        let output = {
            #[cfg(target_os = "windows")]
            {
                Command::new("cmd").args(["/C", command]).output()?
            }
            #[cfg(not(target_os = "windows"))]
            {
                Command::new("sh").args(["-c", command]).output()?
            }
        };
        //  Command gagal (command not found / exit != 0)
        if !output.status.success() {
            let stderr = format!(
                "Command Not Found {}",
                String::from_utf8_lossy(&output.stderr)
            );

            Info::set_message_info(
                Arc::clone(&self.info_message),
                std::borrow::Cow::Owned(stderr),
                MessageType::ErrorNotfound,
            );
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn run_one_command(&self, command: &str) {
        let output = Command::new(command).output();

        match output {
            Ok(out) => {
                let result_str = String::from_utf8_lossy(&out.stdout).into();
                Info::set_message_info(
                    self.info_message.clone(),
                    std::borrow::Cow::Owned(result_str),
                    MessageType::INFO,
                );
            }

            Err(_) => {
                Info::set_message_info(
                    Arc::clone(&self.info_message),
                    std::borrow::Cow::Borrowed("Gagal menjalankan Commad"),
                    MessageType::ErrorLine,
                );
            }
        }
    }
    /// fn untuk Commad yang memeiliki Parameter
    fn run_loats_command(&self, command: &str) {
        let splite_cmd: Vec<&str> = command.split_whitespace().collect();

        let output = Command::new(&splite_cmd[0]).args(&splite_cmd[1..]).output();

        match output {
            Ok(out) => {
                let result_str = String::from_utf8_lossy(&out.stdout).into();
                Info::set_message_info(
                    self.info_message.clone(),
                    std::borrow::Cow::Owned(result_str),
                    MessageType::INFO,
                );
            }

            Err(_) => {
                Info::set_message_info(
                    Arc::clone(&self.info_message),
                    std::borrow::Cow::Borrowed("Gagal menjalankan Commad"),
                    MessageType::ErrorLine,
                );
            }
        }
    }

    pub fn is_command(&self, command: &str, target_command: &str) -> bool {
        let cmd: Vec<&str> = command.split_whitespace().collect();
        cmd[0] == target_command
    }
    /// apakah command cuman 1 perintah
    fn is_one_cmd(&self, command: &str) -> bool {
        let cmd: Vec<&str> = command.split_whitespace().collect();
        cmd.len() == 1
    }
    ///cek selain commaad yang ada
    pub fn is_else_commads(&self, command: &str) -> bool {
        // ubah dengan algo nantik
        let cmd = command.split_whitespace().next().unwrap_or("");
        // self.gui_apps.contains(&cmd) || self.tui_tools.contains(&cmd)
        self.gui_apps.iter().any(|a| a == &cmd) || self.tui_tools.iter().any(|a| a == &cmd)
    }
}
