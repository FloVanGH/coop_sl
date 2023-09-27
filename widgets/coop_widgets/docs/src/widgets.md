<!--
SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

## `Button`

A simple button.

### Properties

-   **`text`** (_in_ _string_): The text written in the button.
-   **`icon`** (_in_ _image_): The image to show in the button. Note that not all styles support drawing icons.
-   **`primary`** (_in_ _bool_): If set to true the button is displayed with the primary accent color (default: false).
-   **`has-focus`**: (_out_ _bool_): Set to true when the button has keyboard focus.
-   **`pressed`**: (_out_ _bool_): Set to true when the button is pressed.
-   **`enabled`**: (_in_ _bool_): Defaults to true. When false, the button cannot be pressed

### Callbacks

-   **`clicked()`**

### Example

```slint
import { Button } from "_imports/coop_widgets.slint";

export component Example inherits Window {
    VerticalBox {
        Button {
            text: "Click Me";
            clicked => { self.text = "Clicked"; }
        }
    }
}
```

## `OutlineButton`

A simple button with an outline border.

### Properties

-   **`text`** (_in_ _string_): The text written in the button.
-   **`icon`** (_in_ _image_): The image to show in the button. Note that not all styles support drawing icons.
-   **`primary`** (_in_ _bool_): If set to true the button is displayed with the primary accent color (default: false).
-   **`has-focus`**: (_out_ _bool_): Set to true when the button has keyboard focus.
-   **`pressed`**: (_out_ _bool_): Set to true when the button is pressed.
-   **`enabled`**: (_in_ _bool_): Defaults to true. When false, the button cannot be pressed

### Callbacks

-   **`clicked()`**

### Example

```slint
import { OutlineButton } from "_imports/coop_widgets.slint";

export component Example inherits Window {
    VerticalBox {
        OutlineButton {
            text: "Click Me";
            clicked => { self.text = "Clicked"; }
        }
    }
}
```

## `RoundButton`

A simple round button that can nether display an icon or a single character.

### Properties

-   **`text`** (_in_ _string_): The text written in the button.
-   **`icon`** (_in_ _image_): The image to show in the button. Note that not all styles support drawing icons.
-   **`primary`** (_in_ _bool_): If set to true the button is displayed with the primary accent color (default: false).
-   **`has-focus`**: (_out_ _bool_): Set to true when the button has keyboard focus.
-   **`pressed`**: (_out_ _bool_): Set to true when the button is pressed.
-   **`enabled`**: (_in_ _bool_): Defaults to true. When false, the button cannot be pressed

### Callbacks

-   **`clicked()`**

### Example

```slint
import { RoundButton } from "_imports/coop_widgets.slint";

export component Example inherits Window {
    VerticalBox {
        RoundButton {
            text: "C";
            clicked => { self.text = "Clicked"; }
        }
    }
}
```

## `RoundOutlineButton`

A simple round button that can nether display an icon or a single character with an outline border.

### Properties

-   **`text`** (_in_ _string_): The text written in the button.
-   **`icon`** (_in_ _image_): The image to show in the button. Note that not all styles support drawing icons.
-   **`primary`** (_in_ _bool_): If set to true the button is displayed with the primary accent color (default: false).
-   **`primary`** (_in_ _bool_): If set to true the button is displayed with the primary accent color (default: false).
-   **`has-focus`**: (_out_ _bool_): Set to true when the button has keyboard focus.
-   **`pressed`**: (_out_ _bool_): Set to true when the button is pressed.
-   **`enabled`**: (_in_ _bool_): Defaults to true. When false, the button cannot be pressed

### Callbacks

-   **`clicked()`**

### Example

```slint
import { RoundOutlineButton } from "_imports/coop_widgets.slint";

export component Example inherits Window {
    VerticalBox {
        RoundOutlineButton {
            text: "C";
            clicked => { self.text = "Clicked"; }
        }
    }
}
```

## `CheckBox`

Use a `CheckBox` to let the user select or deselect values, for example in a list with multiple options. Consider using a `Switch` element instead if the action resembles more something that's turned on or off.

### Properties

-   **`checked`**: (_inout_ _bool_): Whether the checkbox is checked or not (default: false).
-   **`enabled`**: (_in_ _bool_): Defaults to true. When false, the checkbox can't be pressed (default: true)
-   **`has-focus`**: (_out_ _bool_): Set to true when the checkbox has keyboard focus (default: false).
-   **`text`** (_in_ _string_): The text written next to the checkbox.

### Callbacks

-   **`toggled(/* checked */ bool)`**: The checkbox value changed

### Example

```slint
import { CheckBox } from "_imports/coop_widgets.slint";

export component Example inherits Window {
    width: 200px;
    height: 25px;

    CheckBox {
        width: parent.width;
        height: parent.height;
        text: "Hello World";
    }
}
```

## `Switch`

A `Switch` is a representation of a physical switch that allows users to turn things on or off. Consider using a `CheckBox` instead if you want the user to select or deselect values, for example in a list with multiple options.

### Properties

-   **`checked`**: (_inout_ _bool_): Whether the switch is checked or not (default: false).
-   **`enabled`**: (_in_ _bool_): Defaults to true. When false, the switch can't be pressed (default: true).
-   **`has-focus`**: (_out_ _bool_): Set to true when the switch has keyboard focue (default: false).
-   **`text`** (_in_ _string_): The text written next to the switch.

### Callbacks

-   **`toggled(/* checked */ bool)`**: The switch value changed

### Example

```slint
import { Switch } from "_imports/coop_widgets.slint";

export component Example inherits Window {
    width: 200px;
    height: 25px;
    Switch {
        width: parent.width;
        height: parent.height;
        text: "Hello World";
    }
}
```

## `ComboBox`

A button that, when clicked, opens a popup to select a value.

### Properties

-   **`current-index`**: (_in-out_ _int_): The index of the selected value (-1 if no value is selected)
-   **`current-value`**: (_in-out_ _ListViewItem_): The currently selected text
-   **`enabled`**: (_in_ _bool_): Defaults to true. When false, the combobox can't be interacted with
-   **`has-focus`**: (_out_ _bool_): Set to true when the combobox has keyboard focus.
-   **`model`** (_in_ _\[ListViewItem\]_): The list of possible values
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
