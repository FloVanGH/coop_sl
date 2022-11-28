<!--
SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

# Building blocks

This module contains stuff like header bar and side bar.

## `SideBar`

`SideBar` is a bar with vertical arranged children.

```slint
export HeaderBar := Rectangle
```

## `HeaderBar`

`HeaderBar` is a bar with a title and a content area.

```slint
export HeaderBar := Rectangle
```

### Properties

* **in `title`** (**string**): Defines the title text of the header bar.