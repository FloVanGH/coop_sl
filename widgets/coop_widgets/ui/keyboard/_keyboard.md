<!--
SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

# Keyboard

This module contains all components for the on screen keyboard.

## `KeyboardPage`

### Callbacks

* **`open_keyboard(string, string, string, string)`**: Opens the keyboard.
* **`keyboard_closed(string, string)`**: Called after keyboard is closed.

## global `KeyboardAdapter`

`KeyboardAdapter` helps to handle keyboard actions that cannot be done at the moment with slint.

### Callbacks

* **`backspace(string) -> string`**:  Removes the last character of the given string and returns the result.

## struct `KeyModel`

`KeyModel` Defines the model of a key of the `Keyboard`.

### Fields

* **`l`** (**string**): Character lower case.
* **`u`** (**string**): Character upper case.
* **`a_l`** (**string**): Alternative character lower case.
* **`a_u`** (**string**): Alternative character upper case.

## struct `KeyboardLayout`

`KeyboardLayout` is used to define a button layout for the  `Keyboard`.

### Fields

* **`row_one`** (**[KeyModel]**): First row of keys.
* **`row_two`** (**[KeyModel]**): Second row of keys.
* **`row_three`** (**[KeyModel]**):Third row of keys.

## `Keyboard`

`Keyboard` represent an on screen keyboard.

### Properties

* **in-out `placeholder`** (**string**): Placeholder of the inner text line.
* **in-out `icon`** (**string**): Icon of the inner text line.
* **in-out `text`** (**string**): The text that can be manipulated by the `Keyboard`.
* **in `layout`** (**KeyboardLayout**): Defines the button layout of the keyboard.

### Callbacks

* **`close(string, string)`**: The keyboard closes.
* **`open(string, string, string, string)`**: Opens the keyboard with the given key and text.