# Remapper for the Bluetooth (Eraser) Button of the Surface Pen

Work in progress.

The Surface Pen bluetooth-button can distinguish between three actions, which are reported as key events:

- Single-click (<kbd>⌘ Meta</kbd>+<kbd>F20</kbd>).
- Double-click (<kbd>⌘ Meta</kbd>+<kbd>F19</kbd>).
- Press and hold (<kbd>⌘ Meta</kbd>+<kbd>F18</kbd>).

This utility allows you to re-map these actions via evedev/uinput so that they can be used e.g. in presentation tools that do not allow custom keyboard shortcuts or have trouble handling the meta key.
For more hardware-details, have a look at the [official Windows specification page][windows-spec].

[windows-spec]: https://docs.microsoft.com/en-us/windows-hardware/design/component-guidelines/windows-pen-designs#bluetooth-button-implimentation

### Configure



If you are configuring before installation: Open the file `remap.conf` in the folder `etc` and edit it:

If you are configuring after installation: Open the file `remap.conf` in the folder `/etc/surface-pen-button/` and edit it:
> The content of this file is toml syntax so keys are simple strings and actions are arrays of strings.
1) Change `vendor` if neccessary. You can check the vendor id of your pen by using `sudo evtest` and then selecting `Surface Pen Keyboard`. You'll find you vendor id at `Input device ID` after `vendor`.
2) Change `product` if neccessary. You can check the product id of your pen by using `sudo evtest` and then selecting `Surface Pen Keyboard`. You'll find you product id at `Input device ID` after `product`.
3) Change the key combinations for the actions. You have to put the keys in quotation marks (e. g. "KEY_ESC"). If you want multiple keys, you have to put a comma (, ) between them.
<details> <summary> Available keys: </summary>
  
```
KEY_ESC
KEY_1
KEY_2
KEY_3
KEY_4
KEY_5
KEY_6
KEY_7
KEY_8
KEY_9
KEY_0
KEY_MINUS
KEY_EQUAL
KEY_BACKSPACE
KEY_TAB
KEY_Q
KEY_W
KEY_E
KEY_R
KEY_T
KEY_Y
KEY_U
KEY_I
KEY_O
KEY_P
KEY_LEFTBRACE
KEY_RIGHTBRACE
KEY_ENTER
KEY_LEFTCTRL
KEY_A
KEY_S
KEY_D
KEY_F
KEY_G
KEY_H
KEY_J
KEY_K
KEY_L
KEY_SEMICOLON
KEY_APOSTROPHE
KEY_GRAVE
KEY_LEFTSHIFT
KEY_BACKSLASH
KEY_Z
KEY_X
KEY_C
KEY_V
KEY_B
KEY_N
KEY_M
KEY_COMMA
KEY_DOT
KEY_SLASH
KEY_RIGHTSHIFT
KEY_KPASTERISK
KEY_LEFTALT
KEY_SPACE
KEY_CAPSLOCK
KEY_F1
KEY_F2
KEY_F3
KEY_F4
KEY_F5
KEY_F6
KEY_F7
KEY_F8
KEY_F9
KEY_F10
KEY_NUMLOCK
KEY_SCROLLLOCK
KEY_KP7
KEY_KP8
KEY_KP9
KEY_KPMINUS
KEY_KP4
KEY_KP5
KEY_KP6
KEY_KPPLUS
KEY_KP1
KEY_KP2
KEY_KP3
KEY_KP0
KEY_KPDOT
KEY_102ND
KEY_F11
KEY_F12
KEY_KPENTER
KEY_RIGHTCTRL
KEY_KPSLASH
KEY_SYSRQ
KEY_RIGHTALT
KEY_HOME
KEY_UP
KEY_PAGEUP
KEY_LEFT
KEY_RIGHT
KEY_END
KEY_DOWN
KEY_PAGEDOWN
KEY_INSERT
KEY_DELETE
KEY_POWER
KEY_KPEQUAL
KEY_PAUSE
KEY_LEFTMETA
KEY_RIGHTMETA
KEY_COMPOSE
KEY_F13
KEY_F14
KEY_F15
KEY_F16
KEY_F17
KEY_F18
KEY_F19
KEY_F20
```

</details>

4) If you are configuring after installation: Stop this program via `sudo killall surface-pen-button`. If you have installed the systemd service, the program will start again after 2 seconds.
  

### Install

Run `./install.sh` in this repository, then follow the instructions.

### Uninstall

Run `./uninstall.sh` in this repository.
