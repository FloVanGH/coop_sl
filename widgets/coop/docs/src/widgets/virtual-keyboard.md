<!--
SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

## `VirtualKeyboardAdapter`

VirtualKeyboardAdapter represents a global that must be used from code to send keyboard events to the VirtualKeyboard.

### Properties

-   **`default-key-sets`** (_in _\[\[[[`KeyModel`](../structs/key-model.md)]\]\]_): Defines the default model of keys.
-   **`open`**: (_in-out_ _bool_): Set to true to open the keyboard.
-   **`int`**: (_out_ _int_): The index of the current key-set.
-   **`keys`**: (_out_ _\[\[[`KeyModel`](../structs/key-model.md)\]\]_): The current selected keys.

### functions

-   **`switch-keyboard()`**: Switches the current keys.

## `VirtualKeyboard`

VirtualKeyboard represents an on screen keyboard. Must be used with `VirtualKeyboardAdapter`.