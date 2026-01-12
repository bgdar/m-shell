
<h1 align="center">
    M shell
    </h1>
> Shell Engine

### tect stack

<p align="center">
  <!-- Ikon skill -->
  <a href="https://skillicons.dev">
    <img src="https://skillicons.dev/icons?i=rust&theme=light&perline=2" />
  </a> 
</p>

- `crossterm` => untuk menghandle input dan parsing key di terminal
- `toml` > untuk management config file
- `dirs` => bisa membantu menyesuikan Path windows, dan UNix , seperti **config_dir** otomatis mendapat : 
| OS      | Lokasi                                              |
| ------- | --------------------------------------------------- |
| Linux   | `~/.config/m-shell/config.toml`                     |
| macOS   | `~/Library/Application Support/m-shell/config.toml` |
| Windows | `C:\Users\user\AppData\Roaming\m-shell\config.toml` |


### File && folder

- `main` => file atau handle utama
- `command`=> untuk daftar commnad command yang akan di gunakan
- `events` => fn utama untuk handle event
- `info` => tangani error , error , info ,

#### Info source code

- **Arc** && **Mutex** : saya guanakna untuk membagi State secara maximal
- **Cow** : untuk menangani Inputan error yang aman dan bisa switch antara _String_ atau _&str_

### `M Shell Commmand`
command bawaan M Shell 
format : 
```bash
m->value_1->value_2->...

```
1. update path 
- `m->path-config->update` : untuk mengupdate path path yang sudah ada di file m-shell.toml 
