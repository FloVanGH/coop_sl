<!--
SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

## `GroupListView`

Like [StandardListView](standard-list-view.md) with parent header items.

### Properties

-   **`current-item`** (_in-out_ _{ parent: int, item: int }_): The index of the currently active item.
-   **`model`** (_in_ _[`[GroupListViewItem]`](../structs/group-list-view-item.md)_): The model of items.
-   **`selection-mode`** (_in_ _[`SelectionMode`](../enums/selection-mode.md)_): Defines the selection mode.
-   **`has-focus`** (_out_ _bool_): True if the list view has focus.
-   **`enabled`** (_in_ _bool_): True if the list view is enabled.

### Functions

-   **`set-current-item(row: int, par-row: int)`**: Sets the current item.

### Callbacks

-   **`current-item-changed(/* index */ { parent: int, item: int })`**: Emitted when the current item has changed because the user modified it
-   **`item-pointer-event(int /* par-index */, int /* item-index */, PointerEvent /* event */, Point /* absolute mouse position */)`**: Emitted on any mouse pointer event similar to `TouchArea`. Arguments are item index associated with the event, the `PointerEvent` itself and the mouse position within the listview.
-   **`key-pressed(/* event */ KeyEvent)`**: Emitted when there is a key press event on the listview.

### Example

```slint
import { GroupListView } from "_imports/coop_widgets.slint";

export component Example inherits Window {
    width: 150px;
    height: 150px;

    GroupListView {
        width: 150px;
        height: 150px;
        model: [
            {
                text: "Header 1",
                items: [
                    { text: "Item 1" },
                    { text: "Item 2" },
                    { text: "Item 3" },
                ]
            },
             {
                text: "Header 2",
                items: [
                    { text: "Item 1" },
                    { text: "Item 2" },
                    { text: "Item 3" },
                ]
            }
        ]
    }
}
```
