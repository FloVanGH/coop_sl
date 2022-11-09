# Widgets

This module contains the base set of `co_widgets`.

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

### Callbacks

* **`selected_changed(bool)`**: Is called after `selected` has changed. The parameter represents the current parameter of `selected`.

### Example

```slint
import { CheckBox } from "_imports/co_widgets.slint";

Example := Rectangle {
    width: 100px;

    VerticalLayout {
        padding: 8px;

        Switch {}
    }
}
```