<!--
SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

# Building blocks

This module contains stuff like header bar and side bar.

## `SideBar`

`SideBar` is a bar with vertical arranged children.

### Properties

* **in `title`** (**string**): Describes the title of the sidebar.
* **in `parent_width`** (**length**): Defines the parent width as reference fot the break point.
* **in `break_point`** (**length**): If the `paren_width` is less then the break_point the layout of the side bar will switch.
* **in-out `expanded`** (**bool**): If set to `true` the sidebar will draw over the content.
* **in-out `responsive`** (**bool**): If set to `true` the layout flips by reaching the break point.

### Example

```slint
import { SideBar } from "_imports/coop_widgets.slint";

SideBarTest := Rectangle {
    min_width: 900px;
    height: 400px;
    background: Theme.brushes.background;

    GridLayout {  
        Rectangle {  
            col: 1;
            min_width: 200px;
            horizontal_stretch: 1;

            Text {
                text: "content";
            }
        }

        i_side_bar := SideBar { 
            responsive: true;
            col: 0;
            height: 100%;
            title: "My App";

            parent_width: parent.width;
        }   
    }
}
```

## `HeaderBar`

`HeaderBar` is a bar with a title and a content area.

```slint
export HeaderBar := Rectangle
```

### Properties

* **in `title`** (**string**): Defines the title text of the header bar.