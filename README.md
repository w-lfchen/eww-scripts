# Scripts for my [eww](https://github.com/elkowar/eww/) configuration
The intended way to access these scripts is the [flake](flake.nix). It currently only contains the eww-helper binary, a small program with a few subcommands for use in scripts.

I might add more scripts if I write scripts that are complex enough to deserve an addition.

## eww-helper
Currently only supports Hyprland, see the respective help pages (`-h` flag) for usage.

Available subcommands:
- `window-title`: Prints the current window title if it changes. <br>
  Intended for [listening variables](https://elkowar.github.io/eww/configuration.html#adding-dynamic-content).
- `workspaces`: Prints a JSON object with the current workspace information if it changes. <br>
  Intended for [listening variables](https://elkowar.github.io/eww/configuration.html#adding-dynamic-content).
