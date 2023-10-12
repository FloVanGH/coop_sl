<!--
SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

## `TabView`

TabView is a container for a set of tab items.

### Properties

-   **`current-item`** (_in-out_ _int_): The index of the currently active item.
-   **`current-value`** (_out_ _TabItem_): The current selected tab item.
-   **`model`** (_out_ _[`[TabItem]`](../structs/tab-item.md)_): The model of items.
-   **`enabled`** (_in_ _bool_): True if the list view is enabled.
-   **`tab-bar-position`** (_in_ _[Position](../enums/position.md)_): Defines the position of the tab bar (default top).

### Functions

-   **`set-current-item(current-item: int)`**: Sets the current item by the specified index.

### Callbacks

-   **`current-item-changed(/* current-item */ int)`**: Emitted when the current item has changed because the user modified it.
-   **`close(/* index */ int)`**: Emitted when the the given item is requested to be closed.

### Example

```slint
import { TabView } from "_imports/coop_widgets.slint";

export component Example inherits Window {
    width: 150px;
    height: 150px;

    i-tab-view := TabView {
        width: 150px;
        height: 150px;

        model: [
            { text: "Tab 1" },
            { text: "Tab 2" },
            { text: "Tab 3" },
        ];

        Text {
            text: i-tab-view.current-value.text;
        }
    }
}
```
