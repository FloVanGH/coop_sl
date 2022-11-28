<!--
SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

# Widgets

This module contains the base set of `coop_widgets`.

## `ButtonBase`

```slint
export ButtonBase := FocusTouchArea
```

`ButtonBase` is used as base for buttons that can be clicked.
f
### Properties

* **in `icon`** (**string**): Used to set an optional icon on the button. Uses material icons check the `mi` global.
* **in `text`** (**string**): Used to set the display text of the button.
* **in `text_color`** (**brush**): Defines the text color.
* **in `icon_color`** (**brush**): Defines the icon color.
* **in `border_color`** (**brush**): Defines the border color of the widget.
* **in `border_width`** (**length**): Defines the border width of the widget.
* **in `background`** (**brush**): Defines background of the widget.
* **in `icon_size`** (**length**):  Defines the size of the icon.
* **in `container_scale`** (**float**):  Used to scale the inner background container e.g. to add an effect by button pressed.
* **in `border_radius`** (**length**):  Defines the border radius of the button background.

### Example

```slint
import { ButtonBase } from "_imports/coop_widgets.slint";

export MyButton := Example :=  Rectangle {
    in property <string> text: "text";
    in property <string> icon;

    i_base := ButtonBase {
        background: black;
        foreground: white;
        text: root.text;
        icon: root.icon;
    }
}
```

## `Button`

```slint
export Button := Rectangle
```

`Button` represents the default button with filled surface.

### Properties

* **in `text`** (**string**): Used to set the display text of the button.
* **in `icon`** (**string**): Used to set an optional icon on the button. Uses material icons check the `mi` global.
* **in `primary`** (**bool**): If set to  `true` the button will filled with coop.theme.brushes.primary`.
* **in `enabled`** (**bool**): If set to `false` the widget is disabled.

### Callbacks

* **`clicked()`**: Will be called when the widget is clicked (pressed and then released).

### Example

```slint
import { CenterLayout, Button } from "_imports/coop_widgets.slint";

Example := CenterLayout {
    width: 600px;
    height: 400px;

    Button {
        text: "Click me";
        clicked => { debug("Clicked"); }
    }
}
```

## `OutlineButton`

```slint
export OutlineButton := Rectangle
```

`OutlineButton` represents a default button with an outline border.

### Properties

* **in `text`** (**string**): Used to set the display text of the button.
* **in `icon`** (**string**): Used to set an optional icon on the button. Uses material icons check the `mi` global.
* **in `primary`** (**bool**): If set to  `true` the button will filled with coop.theme.brushes.primary`.
* **in `enabled`** (**bool**): If set to `false` the widget is disabled.

### Callbacks

* **`clicked()`**: Will be called when the widget is clicked (pressed and then released).

### Example

```slint
import { CenterLayout, Button } from "_imports/coop_widgets.slint";

Example := CenterLayout {
    width: 600px;
    height: 400px;

    OutlineButton {
        text: "Click me";
        clicked => { debug("Clicked"); }
    }
}
```

## `RoundButton`

```slint
export RoundButton := Rectangle
```

`RoundButton` represents a round fixed sized button with ether an icon or a single character text.

### Properties

* **in `text`** (**string**): Used to set the display text of the button.
* **in `icon`** (**string**): Used to set an optional icon on the button. Uses material icons check the `mi` global.
* **in `enabled`** (**bool**): If set to `false` the widget is disabled.

### Callbacks

* **`clicked()`**: Will be called when the widget is clicked (pressed and then released).

### Example

```slint
import { CenterLayout, Button, mi } from "_imports/coop_widgets.slint";

Example := CenterLayout {
    width: 600px;
    height: 400px;

    RoundButton {
        icon: mi.settings;
        clicked => { debug("Clicked"); }
    }
}
```

## `ButtonOutline`

```slint
export ButtonOutline := Rectangle
```

`RoundOutlineButton` represents a round fixed sized button with ether an icon or a single character text and an outline border.

### Properties

* **in `text`** (**string**): Used to set the display text of the button.
* **in `icon`** (**string**): Used to set an optional icon on the button. Uses material icons check the `mi` global.
* **in `primary`** (**bool**): If set to  `true` the button will filled with coop.theme.brushes.primary`.
* **in `enabled`** (**bool**): If set to `false` the widget is disabled.

### Callbacks

* **`clicked()`**: Will be called when the widget is clicked (pressed and then released).

### Example

```slint
import { CenterLayout, Button, mi } from "_imports/coop_widgets.slint";

Example := CenterLayout {
    width: 600px;
    height: 400px;

    ButtonOutline {
        text: mi.settings;
        clicked => { debug("Clicked"); }
    }
}
```

## `ComboBox`

`ComboBox` enables users to select a value from a list inside of a `Popup`.

```slint
export ComboBox := Rectangle
```

### Properties

* **in `enabled`** (**bool**): If set to `false` the widget is disabled.
* **in `model`** (**[ItemModel]**): Defines the list model of the combo box.
* **in `placeholder`** (**string**): Defines a text that is displayed if no item is selected.
* **out `current_item`** (**ItemModel**): Represents the curren selected item. If there is no selection the current item is empty.
* **in-out`current_index`** (**int**):  Defines the index of the current selected item.

### Callbacks

* **`current_changed(int)`**: Is called after current index is changed.

### Example

```slint
import { CenterLayout, ComboBox } from "_imports/coop_widgets.slint";

Example := CenterLayout {
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

```slint
export SelectableBase := FocusTouchArea
```

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
import { SelectableBase } from "_imports/coop_widgets.slint";

CheckBox := SelectableBase {
    i_container := Example :=  Rectangle {
        width: 20px;
        height: 20px;
        border_width: 1px;
        border_color: "black";
    }

    states [
        selected when root.selected: {
            i_container.background: green;
        }
    ]
}
```

## `CheckBox`

`CheckBox` represents a selectable check box with a text.

```slint
export CheckBox := Rectangle
```

### Properties

* **in `selected`** (**bool**): If set to `true` the widget is marked selected.
* **in-out `text`** (**string**): Defines the label text of the `CheckBox`.
* **in`enabled`** (**bool**): If set to `false` the widget is disabled.

### Callbacks

* **`selected_changed(bool)`**: Is called after `selected` has changed. The parameter represents the current parameter of `selected`.

### Example

```slint
import { CheckBox } from "_imports/coop_widgets.slint";

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

```slint
export Switch := Rectangle
```

### Properties

* **in-out `selected`** (**bool**): If set to `true` the widget is marked selected.
* **in `on_icon`** (**string**): Describes the icon of the on state.
* **in `off_icon`** (**string**): Describes the icon of the off state.
* **in `enabled`** (**bool**): If set to `false` the widget is disabled.

### Callbacks

* **`selected_changed(bool)`**: Is called after `selected` has changed. The parameter represents the current parameter of `selected`.

### Example

```slint
import { Switch } from "_imports/coop_widgets.slint";

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
import { Slider } from "_imports/coop_widgets.slint";

Example := Example :=  Rectangle {
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

## struct `ItemModel`

`ItemModel` represents a list item used by `List` and `ListView`.

### Fields

* **`leading_icon`** (**string**): Defines the icon displayed left of the text.
* **`text`** (**string**): Defines the text of the item.
* **`trailing_icon`** (**string**): Defines the icon displayed right of the text.

## struct `ParentItemModel`

`ParentItemModel` represents a list item with list item children used by `List` and `ListView`.

### Fields

* **`text`** (**string**): Defines the text of the item.
* **`items`** (**[ItemModel]**): Defines the list of children items.

## `List`

`List` is a non scrollable list with a default item delegate.

```slint
export List := VerticalLayout
```

### Properties

* **in `model`** (**[ParentItemModel]**: Defines the list of models.
* **in-out `current_item`** (**{ parent: int, item: int }**): Defines the current selected item.

### Callbacks

* **`current_item_changed()`**: Will be called after current is changed.

### Example

```slint
import { CenterLayout, List } from "_imports/coop_widgets.slint";

Example := CenterLayout {
    width: 600px;
    height: 400px;

    List {
        property <ParentItemModel> items: {
            text: "Parent 1",
            items: [
                { leading_icon: mi.add, text: "Item", trailing_icon: ""},
                { leading_icon: "", text: "Item", trailing_icon: ""},
                { leading_icon: "", text: "Item", trailing_icon: ""},
                { leading_icon: "", text: "Item", trailing_icon: ""},
            ]
        };
        model: [
            items
        ];
     }
}
```

## `ListView`

`ListView` is a scrollable variant of `List`.

```slint
export List := Rectangle
```

### Properties

* **in `model`** (**[ParentItemModel]**: Defines the list of models.
* **in-out `current_item`** (**{ parent: int, item: int }**): Defines the current selected item.

### Callbacks

* **`current_item_changed()`**: Will be called after current is changed.

### Example

```slint
import { CenterLayout, ListView } from "_imports/coop_widgets.slint";

Example := CenterLayout {
    width: 600px;
    height: 400px;

    ListView {
        property <ParentItemModel> items: {
            text: "Parent 1",
            items: [
                { leading_icon: mi.add, text: "Item", trailing_icon: ""},
                { leading_icon: "", text: "Item", trailing_icon: ""},
                { leading_icon: "", text: "Item", trailing_icon: ""},
                { leading_icon: "", text: "Item", trailing_icon: ""},
            ]
        };
        model: [
            items
        ];
     }
}
```

## `ProgressBar`

`ProgressBar` informs about current progress of ongoing process.

```slint
export ProgressBar := Rectangle
```

### Properties

* **in `progress`** (**float**): Represents the current progress. Must be a progress between 0.0 and 1.0.
* **in `indeterminate`** (**bool**): If set to `true` the progress bar shows generic progress.

### Example

```slint
import { ProgressBar } from "_imports/coop_widgets.slint";

Example :=  Rectangle {
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

## `ScrollView`

`ScrollView` allows to scroll content that is larger then the given size.

```slint
export ScrollView := Rectangle
```

### Properties

* **in `viewport_width`** (**length**: The width of the content view port.
* **in `viewport_height`** (**length**: The height of the content view port.
* **in `viewport_x`** (**length**: The current view port content x position.
* **in `viewport_y`** (**length**: The current view port content y position.
selected.
* **`enabled`** (**bool**): If set to `false` the widget is disabled.

### Example

```slint
import { ScrollView } from "_imports/coop_widgets.slint";

Example :=  Rectangle {
    width: 600px;
    height: 400px;

    ScrollView {
        width: 100%;
        height: 100%;

        VerticalLayout {
            spacing: 4px;

            for i in [0, 1, 2, 3, 4, 5, 6, 7] : Rectangle {
                height: 200px;
                background: black;
            }
        }
     }
}
```

## `SelectableBase`

`SelectableBase` is used as base for widgets that can be clicked and toggle between states `unselected` and `selected`.

```slint
export SelectableBase := FocusTouchArea
```

### Properties

* **in-out `selected`** (**bool**): If set to `true` the widget is marked as selected.

### Callbacks

* **`selected_changed(bool)`**: Is called after `selected` has changed. The parameter represents the current parameter of `selected`.

### Example

```slint
import { TextLine } from "_imports/coop_widgets.slint";

MySelectable := Rectangle {
    in-out property <bool> selected <=> i_base.selected;
    i_base := SelectableBase {}
}
```

## `Slider`

`Slider` allow to make selections from a range of values.

```slint
export Slider := Rectangle
```

### Properties

* **out `has_focus`** (**bool**): If `true` the widget has keyboard focus.
* **in `indeterminate`** (**bool**): If set to `false` the widget is disabled.
* **in-out `value`** (**float**): Represents the current value of the slider. Must be a value between 0.0 and 1.0.

### Callbacks

* **`value_changed(float)`**: Is called after `value` has changed. The parameter represents the current value.

### Example

```slint
import { Slider } from "_imports/coop_widgets.slint";

Example :=  Rectangle {
    width: 600px;
    height: 400px;

    VerticalLayout {  
        alignment: center;

        Slider {}   
    }
}
```

## `TextLine`

`TextLine` is a single line text input widget.

```slint
export TextLine := Rectangle
```

### Properties

* **in `icon`** (**string**): Defines the icon that is displayed in front of the text.
* **in `placeholder`** (**string**): Defines a text that is displayed if text is empty.
* **in `has_clear`** (**bool**): If set to true a clear all button is displayed.
* **in `read_only`** (**bool**): If set to `true` the text cannot be selected or changed by keyboard input.
* **in `clickable`** (**bool**): If set to `true` the widget can call the  `clicked` callback.
* **in `input_type`** (**InputType**): The text line ca be used as text or password input.
* **in `has_error`** (**bool**): If set to `true` the text line will display an error border.
* **in `enabled`** (**bool**): If set to false the widget is disabled.
* **in-out `text`** (**string**): The text that is displayed and can changed by text input.


### Callbacks

* **`edited(string)`**:  Is called after text is changed.
* **`clicked()`**: Will be called when the widget is clicked (pressed and then released).

### Example

```slint
import { TextLine } from "_imports/coop_widgets.slint";

Example :=  Rectangle {
    width: 600px;
    height: 400px;

    VerticalLayout {  
        alignment: center;
        padding: 16px;

        TextLine {
            text: "hello";
        }   
    }
}
```