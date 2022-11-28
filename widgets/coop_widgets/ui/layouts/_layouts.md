<!--
SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

# Layouts

This module contains additional layout helpers and components.

## `CenterLayout`

`CenterLayout` aligns its child vertical and horizontal center.

### Example

```slint
import { CenterLayout } from "_imports/coop_widgets.slint";

Example := Rectangle {
    width: 400px;
    height: 400px;
    background: red;

    CenterLayout {  
        Rectangle {  
            min_width: 100px;
            min_height: 100px;
            background: blue;
        }
    }
}
```