# sfm - simple file manager

> Simple two-panel file manager written in Rust inspired by vim and Total Commander

![screenshot](./screen.png)

---
*Warning: Current status is "work in progress" if you run in some problem please create issue and I will check problem*
---

## Features:
In order to get icons please install nerd font in your system (on screen Jetbrains Mono Nerd Font Mono)

1. Current features
    * Fies management
        - Add file or directory
        - Remove file or directory
        - Move file or dir between panels
        - Open file in nvim
    * Tab management
        - Open as tab (tabs are indicated on top of panel)
        - Navigate between tabs
        - Close tabs 
2. ToDo
    - Rename file under selection in panel
    - Rename dir under selection in panel
    - Config file
    - Symlink management(create, rename)
    - Panel filtering
    - File system information

## Keyboard config
- `h` - focus left panel
- `l` - focus right panel
- `j` - next item 
- `k` - prev item 
- `ctrl + r` - open rename modal
- `ctrl + l` - move selected item from left to right panel  
- `ctrl + h` - move selected item from right to left panel
- `ctrl + c` - open create modal on focused panel 
- `ctrl + q` - quit program 
- `ctrl + o` - open dir in tab 
- `o` - open dir or file(right now it will open in neovim this will be configurable in future) 
- `n` - next tab 
- `p` - prev tab 
- `backspace` - navigate to dir parent 
- `esc` - close modal 
- `enter` - select modal option 

## Instalation 

Right now you can clone this repo and perform `cargo run` I will come with a more convenient way in the future.

//TODO
