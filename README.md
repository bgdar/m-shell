# my shell

### tect stack

<p align="center">
  <!-- Ikon skill -->
  <a href="https://skillicons.dev">
    <img src="https://skillicons.dev/icons?i=rust&theme=light&perline=2" />
  </a> 
</p>

- `crossterm` => untuk menghandle input dan parsing key di terminal
- `toml` > untuk management config file 

### daftar target fitur shell

Input loop (read–eval–print loop)
Baca baris perintah dari user → parse → eksekusi → kembali ke prompt.

Lexing & parsing
Pisahkan token (kata, operator seperti |, >, &) → buat struktur perintah (command AST sederhana).

Builtin vs eksternal
Jika perintah adalah builtin (cd, exit, export, history, alias), jalankan di proses yang sama; kalau eksternal, fork() + execve().

Redirection & pipes
Atur file descriptors (dup2) sebelum execve(); untuk pipe, buat pasangan fd dan hubungkan output proses kiri ke input proses kanan.

Job control & background
Tangani &, kontrol grup proses (setpgid, tcsetpgrp), signals (SIGCHLD, SIGINT, SIGTSTP) untuk foreground/background.

Signal handling
Shell harus menangani sinyal sendiri (mis. ignore SIGINT pada shell, tetapi forward ke foreground job).

History / completion / config
Simpan riwayat (file, mis. ~/.myshell_history), sediakan tab-completion (readline atau implement sendiri), dan baca file konfigurasi (~/.myshellrc).

Security & sandboxing
Validasi path, hindari injection bila mengimplementasikan builtins yang menjalankan ekspresi, batasi privilege eskalasi.

### source code

- `main` => file atau handle utama
- `command`=> untuk daftar commnad command yang akan di gunakan
- `events` => fn utama untuk handle event
- `info` => tangani error , error , info ,

#### Info source code

- **Arc** && **Mutex** : saya guanakna untuk membagi State secara maximal
- **Cow** : untuk menangani Inputan error yang aman dan bisa switch antara _String_ atau _&str_
