<!--
SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

# co

Theme and style definitions of the `coop_widgets`.

### struct `Brushes`

`Brushes` provides all brushes referenced by `Theme`.

### Properties

* **`background`** (**brush**): Defines the window background.
* **`background_alt`** (**brush**): Defines an alternative background e.g. use by `SideBar`.
* **`foreground`** (**brush**): Defines the foreground e.g. text color that is used on background.
* **`foreground_disabled`** (**brush**): Defines the foreground if the element is disabled.
* **`surface`** (**brush**): Defines the default background of widgets.
* **`surface_disabled`** (**brush**): Defines the default background of for disabled widgets.
* **`on_surface`** (**brush**): Defines the foreground of elements e.g. text that are shown on a `surface` background.
* **`primary`** (**brush**): Defines the primary (key brush) of the theme.
* **`on_primary`** (**brush**): Defines the foreground of elements e.g. text that are shown on a `primary` background.
* **`border`** (**brush**): Defines the brush of border elements.
* **`border_disabled`** (**brush**): Defines the brush of disabled border elements.
* **`state`** (**brush**): Defines the brush of state elements.
* **`shadow`** (**brush**): Defines the brush of drop shadows.
* **`error`** (**brush**): Defines the error brush.

### struct `Duration`

`Duration` provides all animation duration of `Theme`.

### Properties

* **`slow`** (**duration**): The animation runs slow.
* **`medium`** (**duration**): The animation runs medium fast.
* **`fast`** (**duration**): The animation runs fast.

### struct `Icons`

`Icons` provides all icon sizes in px.

### Properties

* **`size_small`** (**length**): Defines the small icon size.
* **`size_medium`** (**length**): Defines the medium icon size.
* **`size_large`** (**length**): Defines the large icon size.

### struct `Radius`

`Radius` defines the border radius.

### Properties

* **`extra_extra_small`** (**length**): Defines the extra extra small border radius.
* **`extra_small`** (**length**): Defines the extra small border radius.
* **`small`** (**length**): Defines the small border radius.
* **`medium`** (**length**): Defines the medium border radius.
* **`large`** (**length**): Defines the large border radius.

### struct `Sizes`

`Sizes` defines the widget and components sizes (width / height).

### Properties

* **`extra_extra_small`** (**length**):Defines the extra extra small size.
* **`extra_small`** (**length**): Defines the extra small size.
* **`small`** (**length**): Defines the small size.
* **`medium`** (**length**): Defines the medium size.
* **`large`** (**length**): Defines the large size.

### struct `Spaces`

`Spaces` defines spaccings and paddings.

### Properties

* **`extra_small`** (**length**): Defines extra small space.
* **`small`** (**length**): Defines the small space.
* **`medium`** (**length**): Defines the medium space.
* **`large`** (**length**): Defines the large space.

### struct `States`

`States` defines opacity and brush.darker / brush.lighter.

### Properties

* **`hover`** (**float**): Defines hover float.
* **`pressed`** (**float**): Defines pressed float.
* **`container_disabled`** (**float**): Defines container disabled float.
* **`content_disabled`** (**float**):  Defines content disabled float.

### struct `Font`

`Font` defines the typography settings of a text.

### Properties

* **`size`** (**length**): Defines the text size.
* **`weight`** (**int**): Defines the font weight.

### struct `Typo`

`Typo` defines the typography settings.

### Properties

* **`small_label`** (**Font**): Defines typo of small labels.
* **`medium_label`** (**Font**): Defines typo of medium labels.
* **`large_label`** (**Font**): Defines typo of large labels.
* **`small_title`** (**Font**): Defines typo of small titles.
* **`medium_title`** (**Font**): Defines typo of medium titles.
* **`large_title`** (**Font**): Defines typo of large titles.

## struct `Theme`

`Theme` is used to define a theme for the  `coop_widgets`.

### Properties

* **`brushes`** (**Brushes**): Brush definitions.
* **`durations`** (**Duration**): Animation durations.
* **`icons`** (**Icons**): Icon sizes.
* **`states`** (**States**): State values e.g. for using .darker().
* **`sizes`** (**Sizes**): Sizes of components and widgets.
* **`radius`** (**Radius**): Border radius.
* **`spaces`** (**Spaces**): Paddings and spaces.
* **`typo`** (**Typo**): Typography settings.

## global `Theme`

`Theme` is used to access to the style resources of the `Theme` theme (dark and light).

### Properties

* **in-out `dark`** (**bool**): Toggles between light and dark theme.
* **in `accent_color`** (**brush**): Defines the accent (primary) brush.
* **in `on_accent_color`** (**brush**): Defines the on accent (primary) brush.
* **`embedded_helper`** (**bool**): If set to `true` helpers for swr will be activated.
* **`theme`** (**Theme**): Defines the current theme of the `coop_widgets`.