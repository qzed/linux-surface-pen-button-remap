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

To install it you need cargo.

Clone the repository and open it. Then run `cargo build --release`  and copy the binary file to /usr/bin/ via `sudo cp ./target/release/surface-pen-button /usr/bin/` . Now you can start it via `sudo surface-pen-button`.

If you want to install the systemd service which automatically starts on system startup and restarts when the Pen isn't connected until it's connected you have to copy the surface-pen-button.service file to /etc/systemd/system/ via `sudo cp surface-pen-button.service /etc/systemd/system/`. Then you have to run `sudo systemctl daemon-reload` and then to enable it at system startup `sudo systemctl enable --now surface-pen-button.service`.
