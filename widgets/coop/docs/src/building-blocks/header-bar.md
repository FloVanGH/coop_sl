<!--
SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

## `HeaderBar`

`HeaderBar` can be used to add an title and action buttons to the top of a view.

### Properties

-   **`title`**: (_in_ _string_): Defines the text that is displayed on the start of the header bar.
-   **`action-action`**: (_in_ _image_): Defines the image for the primary action.
-   **`action-text`**: (_in_ _string_): Defines the text of the primary action. Is displayed inside of a action button.
-   **`action-enabled`**: (_in_ _bool_): If set to `true` the primary action button is enabled (default: false).
-   **`offset`**: (_in_ _length_): Defines an extra offset that is adding to the left padding.

### Example

```slint
import { HeaderBar, Icons, RoundButton } from "@coop/lib.slint";

export component Example inherits Window {
    width: 200px;
    height: 25px;

    VerticalLayout {
        HeaderBar {
            action-icon: Icons.arrow-back;
            action-enabled: true;
            title: "My Fancy header";

            RoundButton {
                icon: Icons.more-vert;
                horizontal-stretch: 0;
            }

            action => {
                debug("action");
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