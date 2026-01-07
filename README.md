# Basic CLI system monitor written in rust.

## Features
Cpu usage, RAM usage, Disks information. Expected to become a full-fledged system monitor.

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
mysys [cpu] [ram] [disks]
```
For example:
```bash
mysys cpu
```
You can print cpu only, ram only, disks info only or all at once, depending on the args you type.

