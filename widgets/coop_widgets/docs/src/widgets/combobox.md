<!--
SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

## `ComboBox`

A button that, when clicked, opens a popup to select a value.

### Properties

-   **`current-index`**: (_in-out_ _int_): The index of the selected value (-1 if no value is selected)
-   **`current-value`**: (_out_ _[`ListViewItem`](../structs/list-view-item.md)_): The currently selected text
-   **`enabled`**: (_in_ _bool_): Defaults to true. When false, the combobox can't be interacted with
-   **`has-focus`**: (_out_ _bool_): Set to true when the combobox has keyboard focus.
-   **`model`** (_in_ _[`[ListViewItem]`](../structs/list-view-item.md)_): The model of items.
-   **`placeholder-text`**: (_in_ _string_): A placeholder text being shown when there is no item selected

### Callbacks

-   **`selected(/* current-index */ int)`**: A value was selected from the combo box. The argument is the currently selected value.

### Example

```slint
import { ComboBox } from "_imports/coop_widgets.slint";

export component Example inherits Window {
    width: 200px;
    height: 130px;

    ComboBox {
        y: 0px;
        width: self.preferred-width;
        height: self.preferred-height;
        model: [
            {
                text: "Item one"
            },
            {
                text: "Item three"
            },
            {
                text: "Item four"
            }
        ];
    }
}
```