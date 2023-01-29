<!--
SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

# Components

Components are use in context of complexer constructs like `Widgets`.

## `CoopWindow`

`CoopWindow` is a default window that uses the background of the `Theme` theme.

```slint
export CoopWindow := Window
```
### Properties

* **`dark`** (**bool**): Set to `true` to use the dark theme.
* **`accent_color`** (**brush**): Defines the accent color of the theme.
* **`on_accent_color`** (**brush**): Defines the on accent color of the theme.

## Example 

```slint
import { CoopWindow } from "_imports/coop_widgets.slint"

MyWindow := CoopWindow {
    title: "My Window";
}
```

## `FocusTouchArea`

`FocusTouchArea` is a touch area with a focus border.

```slint
export FocusTouchArea := TouchArea
```

### Properties

* **in `focus_border_radius`** (**length**): Defines the border radius of the focus border.
* **out `focus_pressed`** (**bool**): If `true` the touch area is pressed or the touch area has focus and  `enter` is pressed.
* **out `has_focus`** (**bool**): If `true` the element has focus.

### Example

```slint
import { FocusTouchArea } from "_imports/coop_widgets.slint"

MyWidget := Rectangle {
   FocusTouchArea {}
}
```

## `BaseLabel`

`BaseLabel` represents the base for `Theme` labels. Set font settings by font property.

```slint
export BaseLabel := Text
```

### Example

```slint
import { BaseLabel } from "_imports/coop_widgets.slint"

MyLabel := BaseLabel {}
```

## `SmallLabel`

`SmallLabel` is a label with settings `Theme.typo.small_label`.

```slint
export SmallLabel := BaseLabel
```

### Example

```slint
import { SmallLabel } from "_imports/coop_widgets.slint"

Example := Rectangle {
    SmallLabel {
        text: "Hello";
    }
}
```

## `MediumLabel`

`MediumLabel` is a label with settings `Theme.typo.medium_label`.

```slint
export MediumLabel := BaseLabel
```

### Example

```slint
import { MediumLabel } from "_imports/coop_widgets.slint"

Example := Rectangle {
    MediumLabel {
        text: "Hello";
    }
}
```

## `LargeLabel`

`LargeLabel` is a label with settings `Theme.typo.large_label`.

```slint
export LargeLabel := BaseLabel
```

### Example

```slint
import { LargeLabel } from "_imports/coop_widgets.slint"

Example := Rectangle {
    LargeLabel {
        text: "Hello";
    }
}
```

## `LargeLabel`

`ErrorLabel` is a `ErrorLabel` displayed with error text color.

```slint
export LargeLabel := SmallLabel
```

### Example

```slint
import { ErrorLabel } from "_imports/coop_widgets.slint"

Example := Rectangle {
    ErrorLabel {
        text: "Hello";
    }
}
```

## `SmallTitle`

`SmallTitle` is a Title with settings `Theme.typo.small_title`.

```slint
export SmallTitle := BaseTitle
```

### Example

```slint
import { SmallTitle } from "_imports/coop_widgets.slint"

Example := Rectangle {
    SmallTitle {
        text: "Hello";
    }
}
```

## `MediumTitle`

`MediumTitle` is a Title with settings `Theme.typo.medium_title`.

```slint
export MediumTitle := BaseTitle
```

### Example

```slint
import { MediumTitle } from "_imports/coop_widgets.slint"

Example := Rectangle {
    MediumTitle {
        text: "Hello";
    }
}
```

## `LargeTitle`

`LargeTitle` is a Title with settings `Theme.typo.large_title`.

```slint
export MediumTitle := BaseTitle
```

### Example

```slint
import { LargeTitle } from "_imports/coop_widgets.slint"

Example := Rectangle {
    MediumTitle {
        text: "Hello";
    }
}
```

## `PopupBorder`

`PopupBorder` can used to draw border and background of a `Popup`.

```slint
export PopupBorder := Rectangle
```

### Example

```slint
import { MediumTitle } from "_imports/coop_widgets.slint"

Example := Rectangle {
    PopupBorder {
        width: 100px;
        height: 20px;
    }
}
```

## `Popup`

`Popup` is used to show a popup overlay.

```slint
export Popup := PopupWindow
```

### Example

```slint
import { Button, Popup } from "_imports/coop_widgets.slint"

Example := Rectangle {
    layout := VerticalLayout {  
        alignment: center;
        Button {
            text: "open popup";
            clicked => { popup.show() }
        }
    }
   
    popup := Popup {
        y: layout.height + layout.y;
        x: layout.x;
        width: 60px;
        height: 20px;

        Text {
            text: "popup content";
        }
    }
}
```

## `Icon`

`Icon` is a text component that uses the `ForkAwesome` font.

```slint
export Icon := Text
```

### Example

```slint
import { Icon, Theme } from "_imports/coop_widgets.slint"

Example := Rectangle {
    Icon {
        text: Icons.fa_var_address_card;
    }
}
```

## `CoopWindow`

`CoopWindow` provides the possibility to adjust the theming of the `coop_widgets`.

### Properties

* **in `coop_override`** (**Theme**): Can be used to override style definitions of the `coop_widgets` theme.