<!--
SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

# Components

Components are use in context of complexer constructs like `Widgets`.

## `CoopWindow`

`CoopWindow` is a default window that uses the background of the `coop` theme.

```slint
export FocusTouchArea := Window
```

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

`BaseLabel` represents the base for `coop` labels. Set font settings by font property.

```slint
export BaseLabel := Text
```

### Example

```slint
import { BaseLabel } from "_imports/coop_widgets.slint"

MyLabel := BaseLabel {}
```

## `LabelSmall`

`LabelSmall` is a label with settings `coop.theme.typo.label_small`.

```slint
export LabelSmall := BaseLabel
```

### Example

```slint
import { LabelSmall } from "_imports/coop_widgets.slint"

Example := Rectangle {
    LabelSmall {
        text: "Hello";
    }
}
```

## `LabelMedium`

`LabelMedium` is a label with settings `coop.theme.typo.label_medium`.

```slint
export LabelMedium := BaseLabel
```

### Example

```slint
import { LabelMedium } from "_imports/coop_widgets.slint"

Example := Rectangle {
    LabelMedium {
        text: "Hello";
    }
}
```

## `LabelLarge`

`LabelLarge` is a label with settings `coop.theme.typo.label_large`.

```slint
export LabelLarge := BaseLabel
```

### Example

```slint
import { LabelLarge } from "_imports/coop_widgets.slint"

Example := Rectangle {
    LabelLarge {
        text: "Hello";
    }
}
```

## `LabelLarge`

`LabelError` is a `LabelError` displayed with error text color.

```slint
export LabelLarge := LabelSmall
```

### Example

```slint
import { LabelError } from "_imports/coop_widgets.slint"

Example := Rectangle {
    LabelError {
        text: "Hello";
    }
}
```

## `TitleSmall`

`TitleSmall` is a Title with settings `coop.theme.typo.title_small`.

```slint
export TitleSmall := BaseTitle
```

### Example

```slint
import { TitleSmall } from "_imports/coop_widgets.slint"

Example := Rectangle {
    TitleSmall {
        text: "Hello";
    }
}
```

## `TitleMedium`

`TitleMedium` is a Title with settings `coop.theme.typo.title_medium`.

```slint
export TitleMedium := BaseTitle
```

### Example

```slint
import { TitleMedium } from "_imports/coop_widgets.slint"

Example := Rectangle {
    TitleMedium {
        text: "Hello";
    }
}
```

## `TitleLarge`

`TitleLarge` is a Title with settings `coop.theme.typo.title_large`.

```slint
export TitleMedium := BaseTitle
```

### Example

```slint
import { TitleLarge } from "_imports/coop_widgets.slint"

Example := Rectangle {
    TitleMedium {
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
import { TitleMedium } from "_imports/coop_widgets.slint"

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

## `ThemeWindow`

`ThemeWindow` provides the possibility to adjust the theming of the `coop_widgets`.

### Properties

* **in `theme_override`** (**Theme**): Can be used to override style definitions of the `coop_widgets` theme.