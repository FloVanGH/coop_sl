<!--
SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

## `ContextMenu`

`ContextMenu` displays a list of items with a default item delegate inside of a `PopupBorder`. The suggested use case it to use a
`ContextMenu` inside of a `PopupWindow`.

### Properties

-   **`model`**: (_in_ _[ListViewItem]_): The model with list items.

### Callbacks

-   **`item-clicked(/* row */ bool)`**: An item was clicked

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

    i-context-menu := PopupWindow {
        x: 50px;
        y: 50px;
        ContextMenu {
            model: [
                { text: "Item 1" },
                { text: "Item 2" },
                { text: "Item 2" },
            ];
        }
    }
}
```