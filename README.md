# Remapper for the Bluetooth (Eraser) Button of the Surface Pen

Work in progress.

The Surface Pen bluetooth-button can distinguish between three actions, which are reported as key events:

- Single-click (<kbd>⌘ Meta</kbd>+<kbd>F20</kbd>).
- Double-click (<kbd>⌘ Meta</kbd>+<kbd>F19</kbd>).
- Press and hold (<kbd>⌘ Meta</kbd>+<kbd>F18</kbd>).

This utility allows you to re-map these actions via evedev/uinput so that they can be used e.g. in presentation tools that do not allow custom keyboard shortcuts or have trouble handling the meta key.
For more hardware-details, have a look at the [official Windows specification page][windows-spec].

[windows-spec]: https://docs.microsoft.com/en-us/windows-hardware/design/component-guidelines/windows-pen-designs#bluetooth-button-implimentation

### Install

Clone this repository and run `chmod +x install.sh && ./install.sh` in it, then follow the instructions.

### Uninstall

Run `chmod +x uninstall.sh && ./uninstall.sh` in this repository.
