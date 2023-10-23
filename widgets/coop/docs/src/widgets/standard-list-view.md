<!--
SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

## `StandardListView`

Like ListView, but with a default delegate, and a `model` property which is a model of type
[`ListViewItem`](../structs/list-view-item.md).

### Properties

-   **`current-item`** (_in-out_ _int_): The index of the currently active item. -1 mean none is selected, which is the default
-   **`model`** (_in_ _[`[ListViewItem]`](../structs/list-view-item.md)_): The model of items.
-   **`selection-mode`** (_in_ _[`SelectionMode`](../enums/selection-mode.md)_): Defines the selection mode.
-   **`edit-item`** (_in_ _int_): Defines the item that is can be edited.
-   **`has-focus`** (_out_ _bool_): True if the list view has focus.
-   **`enabled`** (_in_ _bool_): True if the list view is enabled.
-   **`drag-drop-enabled`** (_in_ _bool_): If set to `true` drop event is enabled.

### Functions

-   **`set-current-item(index: int)`**: Sets the current item by the specified index and brings it into view.
-   **`bring-into-view(index: int)`**: If the given item is outside of the current visible area the list view will be scrolled to ensure the item is visible.

### Callbacks

-   **`current-item-changed(/* current-item */ int)`**: Emitted when the current item has changed because the user modified it
-   **`item-pointer-event(/* index */ int, /* event */ PointerEvent, /* position */ Point)`**: Emitted on any mouse pointer event similar to `TouchArea`. Arguments are item index associated with the event, the `PointerEvent` itself and the mouse position within the listview.
-   **`item-accepted(/* index */ int, /* text */ string)`**: Emitted when the text of the item is edited and after the enter is pressed.
-   **`key-pressed(/* event */ KeyEvent)`**: Emitted when there is a key press event on the list view.
-   **`drop-event(/* event */ `[DropEvent](../structs/drop-event.md)`)`**: Emitted when a row is dropped on an other row.

### Example

```slint
import { StandardListView } from "@coop/lib.slint";

export component Example inherits Window {
    width: 150px;
    height: 150px;

    StandardListView {
        width: 150px;
        height: 150px;
        model: [ { text: "Blue"}, { text: "Red" }, { text: "Green" },
            { text: "Yellow" }, { text: "Black"}, { text: "White"},
            { text: "Magenta" }, { text: "Cyan" },
        ];
    }
}
```
