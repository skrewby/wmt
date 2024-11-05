# WMT

A very simple workspace viewer tool for hyprland. I normally launch it from a keybind which allows me to quickly view my workspaces and jump to them.
![image](https://github.com/user-attachments/assets/91c6a327-7375-4716-87b7-62bdbabd2993)


## Installation
- Clone the repository
- Run `cargo build -r`
- Move the `wmt` binary in `target/release` to a directory that is on your `$PATH`
- Optional: add a keybind on hyprland.conf that launches the program, example:
```
bind = $mainMod, W, exec, [float; focus 1; stayfocused; center; size 800 400] (kitty -e wmt)
```

## Usage
- Enter the ID number to automatically jump to that workspace
- Manually navigate the table with either the arrow keys or vi keys and press Enter to jump to that workspace
