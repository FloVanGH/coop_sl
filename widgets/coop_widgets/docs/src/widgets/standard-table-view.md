<!--
SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

## `StandardTableView`

The `StandardTableView` represents a table of data with columns and rows. Cells
are organised in a model where each row is a model of
\[[`StandardListViewItem`](../builtins/structs.md#standardlistviewitem)\].

### Properties

Same as [`ListView`](list-view.md), and in addition:

-   **`current-sort-column`** (_out_ _int_): Indicates the sorted column. -1 mean no column is sorted.
-   **`columns`** (_in-out_ _\[[`TableColumn`](https://slint.dev/releases/1.2.2/docs/slint/src/language/builtins/structs#tablecolumn)\]_): Defines the model of the table columns.
-   **`selection-mode`** (_in_ _[`SelectionMode`](../enums/selection-mode.md)_): Defines the row selection mode.
-   **`rows`** (_\[\[[`[ListViewItem]`](../structs/list-view-item.md)\]\]_): Defines the model of table rows.
-   **`edit-item`** (_in_ _{ row: int, column: int}_): Defines the cell that is can be edited.
-   **`has-focus`** (_out_ _bool_): True if the table view has focus.
-   **`current-row`** (_in-out_ _int_): The index of the currently active row. -1 mean none is selected, which is the default.

### Callbacks

-   **`sort-ascending(/* column-index */ int)`**: Emitted if the model should be sorted by the given column in ascending order.
-   **`sort-descending/* column-index */ (int)`**: Emitted if the model should be sorted by the given column in descending order.
-   **`row-pointer-event((/* row-index */ int,  /* event */ PointerEvent, /* absolute mouse position */ Point)`**: Emitted on any mouse pointer event similar to `TouchArea`. Arguments are row index associated with the event, the `PointerEvent` itself and the mouse position within the tableview.
-   **`current-row-changed((/* current-row */ int)`**: Emitted when the current row has changed because the user modified it.
-   **`key-pressed(/* event */ KeyEvent)`**: Emitted when there is a key press event on the table view.
-   **`item-accepted(/* row */ int, /* column */ int, /* text */ string)`**: Emitted when the text of the item is edited and after the enter is pressed.

### Functions

-   **`set-current-row((/* current-row */ int)`: Sets the current row by index and brings it into view.

### Example

```slint
import { StandardTableView } from "_imports/coop_widgets.slint";

export component Example inherits Window {
    width: 230px;
    height: 200px;

    StandardTableView {
        width: 230px;
        height: 200px;
        columns: [
            { title: "Header 1" },
            { title: "Header 2" },
        ];
        rows: [
            [
                { text: "Item 1" }, { text: "Item 2" },
            ],
            [
                { text: "Item 1" }, { text: "Item 2" },
            ],
            [
                { text: "Item 1" }, { text: "Item 2" },
            ]
        ];
    }
}
```
