# Scripts for my [eww](https://github.com/elkowar/eww/) configuration
The intended way to access these scripts is the [flake](flake.nix). It currently only contains the eww-helper binary, a small program with a few subcommands for use in scripts.

I might add more scripts if I write scripts that are complex enough to deserve an addition.

## eww-helper
Currently only supports Hyprland, see the respective help pages (`-h` flag) for usage.

Available subcommands:
- `window-title`: Prints the current window title if it changes. <br>
  Intended for [listening variables](https://elkowar.github.io/eww/configuration.html#adding-dynamic-content).
- `workspaces`: Prints a JSON object with workspace information if there are state changes. <br>
  Intended for [listening variables](https://elkowar.github.io/eww/configuration.html#adding-dynamic-content).


### `workspaces` object structure
A workspace object has the following structure:
```json
{
  "id":<int: workspace id>,
  "name":<string: workspace name>,
  "class":<string: some css classes, see below>,
  "active":<bool: whether the workspace is active>
}
```
The classes are `ws-button ws<id>`, where `<id>` is the workspace ID.

A monitor object has the following structure:
```json
{
  "active":<bool: whether the monitor is active>,
  "workspaces":<list of workspace objects: the workspaces on the given monitor>
}
```
If the monitor is not specified, a list of all workspaces on all monitors will be printed. This is intended for single monitor setups.

If a monitor is specified through `-m`, the corresponding monitor object will be printed. This can be used for setups with an arbitrary amount of monitors.

Note that the JSON object is stripped of all whitespace where possible.
