# Basic CLI system monitor written in rust.

## Features
Cpu usage, RAM usage, Disks and network information. Expected to become a full-fledged system monitor.
I'm currently working on dynamic printing on the terminal to have changing values on the tables.

## Instalation
```bash
git clone https://github.com/HenryFigtree/mysys.git
cd mysys
cargo install --path .
```
Will install binary to ~/.cargo/bin (Linux, MacOS) or \Users\<USERNAME>\.cargo\bin (Windows)
Make sure the directory is on PATH


## Usage
```bash
mysys [cpu] [ram] [disks] [network]
```
For example:
```bash
mysys cpu
```
You can print cpu only, ram only, disks info only, network info only or all at once, depending on the args you type.

You can also write:
```bash
mysys --help
```

To show what each argument actually display.
