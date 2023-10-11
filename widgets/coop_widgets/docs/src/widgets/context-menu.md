<!--
SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

## `ContextMenu`

`ContextMenu` displays a list of items with a default item delegate inside of a `PopupWindow`.

### Properties

-   **`model`** (_in_ _[`[ListViewItem]`](../structs/list-view-item.md)_): The model of items.
-   **`min-menu-width`** (_in_ _length_): Defines the minimum width of the context menu.
-   **`offset-x`** (_in_ _length_): Defines the x offset of the context menu.
-   **`offset-y`** (_in_ _length_): Defines the y offset of the context menu.
-   **`has-focus`**: (_out_ _bool_): Set to true when the context menu currently has the focus.
-   **`current-item`** (_in-out_ _int_): The index of the currently active item. -1 mean none is selected, which is the default.

### Callbacks

-   **`item-clicked(/* row */ int)`**: Emitted when an item was clicked.
-   **`close()`**: Emitted when the context menu is closed.

### Functions

-   **`show()`** Call this function show the context menu.
-   **`close()`** Call this function close the context menu.
-   **`show-and-focus()`** Call this function show and focus the context menu.

### Example

```slint
import { ContextMenu, Button } from "_imports/coop_widgets.slint";

export component Example inherits Window {
    width: 200px;
    height: 25px;

    Button {
        text: "Open context menu";
        clicked => {
            i-context-menu.show();
        }
    }

    i-context-menu := ContextMenu {
        x: 50px;
        y: 50px;
        model: [
            { text: "Item 1" },
            { text: "Item 2" },
            { text: "Item 2" },
        ];
    }
}
```