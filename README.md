# Rust-DragonOS
rust used a simple OS

## Features
- Basic boot process
- VGA text buffer
- Serial output
- Panic handling
- basic Test framework

## Usage
Build: `cargo bootimage` 
or 
you can use github release: [demo](https://github.com/cyberworrier8088/Rust-DragonOS/releases/download/demo/bootimage-Rust-DragonOS.bin):

release windows you went to use it with qemu install in your pc or wsl install and run it with qemu

using qemu:
```bash
qemu-system-x86_64 -drive format=raw,file=bootimage-Rust-DragonOS.bin
```


## demo
![demo](assets/demo.png)