<!--
SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

## `ResponsiveContainer`

Checks if the given `reference-width` is less the given breakpoint and then sets `shrink` to `true`.

### Properties

-   **`reference-width`**: (_in_ _length_): Defines the reference width for the break point check. Commonly the width of the containers parent.
-   **`break-point`**: (_in_ _length_): Defines the break point, that is used to evaluated `shrink` (default 444px).
-   **`shrink`**: (_out_ _bool_): If set to `true` it indicates that one of the children should shrink, commonly the `SideBar`.
-   **`expand-button-width`**: (_out_ _length_): Describes the width of the expand button.
-   **`expand-shrunken`**: (_in-out_ _bool_): If set to `true` and `shrink` is `true` the shrunken element should be expanded as overlay.

### Example

```slint
import { ResponsiveContainer, SideBar } from "@coop/lib.slint";

export component Example inherits Window {
    width: 200px;
    height: 25px;

    i-container := ResponsiveContainer {
        reference-width: parent.width;

        if (!self.shrink) : HorizontalLayout {
            SideBar {}

            Rectangle {
                Text {
                    text: "content";
                }
            }
        }

        if (self.shrink) : Rectangle {
            Rectangle {
                Text {
                    text: "content";
                }
            }

            SideBar {
                x:  i-container.expand-shrunken ? -self.width : 0;
            }
        }
    }
}
```