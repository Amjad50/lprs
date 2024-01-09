# Lprs

Lprs is a local password manager designed to securely store and manage your passwords.

This is a fork of Lprs (https://github.com/TheAwiteb/lprs) with the aim to be an early demo for `AmjadOS`.
[See later how its being run](#running-in-amjados)

## Installation

To install Lprs, you will need to have the Cargo package manager installed. If you do not have Cargo installed, you can install it by following the instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html). Note the Minimum Supported Rust Version (MSRV) for Lprs is `1.70.0`.

1. Install using [cargo-install](https://doc.rust-lang.org/cargo/commands/cargo-install.html):
```bash
cargo install lprs --locked
```
This will enable the update notifications for Lprs. If you don't want to enable update notifications, you can install Lprs using:
```bash
cargo install lprs --locked --no-default-features
```

2. Run Lprs:
```bash
lprs --help
```

## Uninstallation
```bash
cargo uninstall lprs
```

## Usage

Lprs provides a command-line interface for managing your passwords. The following commands are available:

```
A local CLI password manager

Usage: lprs [OPTIONS] <COMMAND>

Commands:
  add     Add new password
  remove  Remove password
  list    List your password and search
  clean   Clean the password file
  edit    Edit the password content
  gen     Generate password
  export  Export the passwords
  import  Import passwords
  help    Print this message or the help of the given subcommand(s)

Options:
  -p, --passwords-file <PASSWORDS_FILE>
          The passwords json file, default: $HOME/.local/share/lprs/passwords.json
  -h, --help
          Print help
  -V, --version
          Print version
```

### Example
```bash
lprs add -n "Gmail" -u "some@gmail.com" -p $(lprs gen 19 -u -l -s) -s "https://mail.google.com"
```

#### Result
This is the result when search for it
```
$ lprs list -e "mail" -p -s
Master Password: ***************
+-------+-------+----------------+---------------------+-------------------------+
| Index | Name  | Username       | Password            | Service                 |
+================================================================================+
| 31    | Gmail | some@gmail.com | >NC`q$%+Nno<y&<y]VB | https://mail.google.com |
+-------+-------+----------------+---------------------+-------------------------+
```


### Backup

It is important to regularly backup your passwords to prevent data loss. Lprs does not provide an automatic backup feature. To backup your passwords, you can use the `export` command provided by Lprs. This command allows you to export your encrypted passwords to a json file, which you can then manually backup to a secure location.

#### Formats
The format of the exported file can be specified using the `--format` option. The following formats are supported:

-  `lprs`: The default format used by Lprs. This format is encrypted and can be imported back into Lprs using the `import` command. This is the recommended format to use for backups as it is encrypted and can be imported back into Lprs.
- `bit-warden`: The format used by [Bitwarden](https://bitwarden.com/). This format is not encrypted and can be imported into Bitwarden. This format is useful if you want to switch to Bitwarden or another password manager that supports this format.



## Running in AmjadOS

So, there are few changes made to the original code to make it run on AmjadOS. The changes are:

- Removed some dependancies (and related changes to the code):
    - `comfy-table`: because we don't have terminal window, so we just print the table as `Vec<Vec<String>>`
    - `directories`: because we don't have `XDG` directories, so we just use the `/` root directory, which only works for us. Thus, this will crash on windows and linux.
    - `pretty_env_logger`: not sure if it will work, but we don't need it.
    - `scanpw`: We don't have hidden password input, so we just use `stdin` to input the master password.
    - `sha256`: Too complex, replaced with better `lhash` crate.
    - `url`: Not needed.
- And changed `random-pick` which is used inside `passwords` to a modified version that doesn't use `getrandom` crate, from inside `rand`, since we don't have that, so replaced them to use a very basic "not random" `small_rng` from `rand`. Check the modifications in [random-pick/src/lib.rs](random-pick/src/lib.rs) for example (same for random-number).


Ok, so next, is how we can build it. This is built for [Amjad50/OS](https::/github.com/Amjad50/OS) which of course is not available in rust.

Normally when developing a no-std project, we can use `target.json` files, but here this project uses `std` and requires libraries that use `std` as well. So the option here is to add our own custom target into `rust`. 

[Which is what I did here](https://github.com/Amjad50/rust/tree/amjad50_os_new_target) (notice this is not the default branch, so if u clone this make sure to checkout to that branch).

This is a fork of the rust compiler containing the target `x86_64-unknown-amjad_os`, so if you build the compiler. Like this
```sh
$ ./x.py build  -i --stage 1  --target x86_64-unknown-amjad_os
```
Then you can link it to `rustup`
```sh
$ rustup toolchain link amjad_os /path/to/rust/build/host/stage1
```

After that, you can come to this project, and run the build with the correct target and toolchain
```sh
$ cargo +amjad_os build --target x86_64-unknown-amjad_os
```

And with that, you will have an ELF file that you can use in AmjadOS.

You can do that by building AmjadOS, and adding the `lprs` elf to the `filesystem` folder along with `passwords.json` containing your passowrds (since at the time of writing there is no `write` support, only `read`).
Then you can run read related commands of `lprs` and enjoy.



## Contributing

Contributions to Lprs are welcome! If you would like to contribute, please follow the guidelines outlined in the [CONTRIBUTING](CONTRIBUTING.md) file.

## License

Lprs is licensed under the GPL-3.0 License. This means that you are free to use, modify, and distribute the software under the terms of this license. Please refer to the [LICENSE](LICENSE) file for more details.
