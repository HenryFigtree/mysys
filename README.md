# Basic CLI system monitor written in rust.

## Features
Cpu usage and RAM usage. Expected to become a full-fledged system monitor.

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
mysys [cpu] [ram]
```
For example:
```bash
mysys cpu ram
```
You can print cpu only, ram only or both, depending on the args you type.

