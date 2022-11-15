<!--
SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

# Widgets

This module contains the base set of `co_widgets`.

## `ComboBox`

`ComboBox` enables users to select a value from a list inside of a `Popuup`.

### Properties


* **`enabled`** (**bool**): If set to `false` the widget is disabled.
* **`model`** (**[ItemModel]**): Defines the list model of the combo box.
* **`current_index`** (**int**):  Defines the index of the current selected item.
* **`current_item`** (**ItemModel**): Represents the curren selected item. If there is no selection the current item is empty.
* **`placeholder`** (**string**): Defines a text that is diplayed if no item is selected.


### Callbacks

* **`current_changed(int)`**: Is called after current index is changed.

### Example

```slint
import { CenterLayout, ComboBox } from "_imports/co_widgets.slint";

CenterLayout {
    width: 200px;
    height: 400px;

    ComboBox := {
        width: 200px;
        placeholder: "Select an item";
        model: [
            {
                text: "Item 1",
            },
            {
                text: "Item 2",
            },
            {
                text: "Item 3",
            }
        ];

        current_changed => { debug(current_item.text); }
    }
}
```

## `SelectableBase`

`SelectableBase` is used as base for widgets that can be clicked and toggle between states `unselected` and `selected`.

### Properties

* **`selected`** (**bool**): If set to `true` the widget is marked selected.
* **`enabled`** (**bool**): If set to `false` the widget is disabled.
* **`has_hover`** (**bool**): If set to `true` the widget has pointer hover.
* **`has_focus`** (**bool**): If set to `true` the widget has keyboard focus.
* **`pressed`** (**bool**): If set to `true` the widget is pressed by a pointer.

### Callbacks

* **`selected_changed(bool)`**: Is called after `selected` has changed. The parameter represents the current parameter of `selected`.

### Example

```slint
import { SelectableBase } from "_imports/co_widgets.slint";

CheckBox := SelectableBase {
    i_container := Rectangle {
        width: 20px;
        height: 20px;
        border_width: 1px;
        border_color: "black";
    }

    states [
        selected when root.selected : {
            i_container.background: green;
        }
    ]
}
```

## `CheckBox`

`CheckBox` represents a selectable check box with a text.

### Properties

* **`selected`** (**bool**): If set to `true` the widget is marked selected.
* **`text`** (**string**): Defines the label text of the `CheckBox`.
* **`enabled`** (**bool**): If set to `false` the widget is disabled.

### Callbacks

* **`selected_changed(bool)`**: Is called after `selected` has changed. The parameter represents the current parameter of `selected`.

### Example

```slint
import { CheckBox } from "_imports/co_widgets.slint";

Example := Rectangle {
    width: 100px;

    VerticalLayout {
        padding: 8px;

        CheckBox {
            text: "Check me";
        }
    }
}
```

## `Switch`

`Switch` represents a selectable that can be toggled on an off.

### Properties

* **`selected`** (**bool**): If set to `true` the widget is marked selected.
* **`on_icon`** (**string**): Describes the icon of the on state.
* **`off_icon`** (**string**): Describes the icon of the off state.
* **`enabled`** (**bool**): If set to `false` the widget is disabled.

### Callbacks

* **`selected_changed(bool)`**: Is called after `selected` has changed. The parameter represents the current parameter of `selected`.

### Example

```slint
import { Switch } from "_imports/co_widgets.slint";

Example := Rectangle {
    width: 100px;

    VerticalLayout {
        padding: 8px;

        Switch {}
    }
}
```

## `Slider`

`Slider` allow to make selections from a range of values.

### Properties

* **`value`** (**float**): Represents the current value of the slider. Must be a value between 0.0 and 1.0.
* **`has_focus`** (**bool**): If `true` the widget has keyboard focus.
* **`enabled`** (**bool**): If set to `false` the widget is disabled.

### Callbacks

* **`value_changed(float)`**: Is called after `value` has changed. The parameter represents the current value.

### Example

```slint
import { Slider } from "_imports/co_widgets.slint";

Example := Rectangle {
    width: 100px;

    VerticalLayout {
        padding: 8px;
        spacing: 4px;

        Text {
            text: "\{slider.value}";
            horizontal_alignment: center;
        }

        slider := Slider {
            text: "Check me";
        }
    }
}
```

## `ProgressBar`

`ProgressBar` informs about current progress of ongoing process.

### Properties

* **`progress`** (**float**): Represents the current progress. Must be a progress between 0.0 and 1.0.
* **`indeterminate`** (**bool**): If set to `true` the progress bar shows generic progress.

### Example

```slint
import { ProgressBar } from "_imports/co_widgets.slint";

Example := Rectangle {
    width: 100px;

    VerticalLayout {
        padding: 8px;

        ProgressBar {
            progress: 0.5;
        }
        ProgressBar {
            indeterminate: true;
        }
    }
}
```