<!--
SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

## `SideBar`

`SideBar` represents a container that can be used as parent for navigation elements.

### Properties

-   **`title`**: (_in_ _string_): Defines the text that is displayed on top of the side bar.
-   **`resizable`**: (_in_ _bool_): If set to `true` the side bar can be resized by dragging on the right side (default false).
-   **`collapse-title`**: (_in_ _bool_): If set to `true` title text is hidden and does not take place in the sidebars layout (default false).

### Example

```slint
import { SideBar, StandardListView } from "@coop/lib.slint";

export component Example inherits Window {
    width: 200px;
    height: 25px;

    HorizontalLayout {
        SideBar {
            StandardListView {
                model: [
                    { text: "Navigation 1" },
                    { text: "Navigation 2" },
                    { text: "Navigation 3" },
                ]
            }
        }

        Rectangle {
            Text {
                text: "content"
            }
        }
    }
}
```