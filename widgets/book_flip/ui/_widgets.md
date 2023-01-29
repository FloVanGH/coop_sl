<!--
SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

# book_flip widgets

This module contains all the widgets provided by the `book_flip` library.

## `BookPage`

`BookPage`  defines a book page with  `title`, `content` and page `number`.

```slint
export component BookPage inherits Rectangle
```

### Properties

* **in `title`** (**string**): Defines the title of the page.
* **in `title_color`** (**brush**): Defines the color of the title label.
* **in `title_font_size`** (**length**): Defines the font size of the title label.
* **in `title_font_weight`** (**int**): Defines the font weight of the title label.
* **in `title_font_family`** (**string**): Defines the font family of the title label.
* **in `number`** (**int**): Defines the page number.
* **in `number_color`** (**brush**): Defines the color of the page number label.
* **in `number_font_size`** (**length**): Defines the font size of the page number label.
* **in `number_font_weight`** (**int**): Defines the font weight of the page number label.
* **in `number_font_family`** (**string**): Defines the font family of the page number label.
* **in `number_alignment`** (**TextHorizontalAlignment**) Defines the horizontal alignment of the page number label.
* **in `spacing`** (**length**) Defines the spacing between title label, separator, text label and number label.
* **in `separator_color`** (**brush**) Defines the color of the title separator bar.

## `BookPage`

`BookPage`  defines a book page with  `title`, `content` and page `number`.

```slint
export component TextPage inherits BookPage
```

### Properties

* **in `text`** (**string**): Defines the text of the page.
* **in `text_color`** (**brush**): Defines the color of the text label.
* **in `text_font_size`** (**length**): Defines the font size of the text label.
* **in `text_font_weight`** (**int**): Defines the font weight of the text label.
* **in `text_font_family`** (**string**): Defines the font family of the text label.label.

### Example

```slint
PageExample := Rectangle {
    width: 200px;
    height: 600px;

    HorizontalLayout {  
        TextPage {
            title: "Chapter 1";
            number: 1;
            horizontal-stretch: 1;
            text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Gravida in fermentum et sollicitudin ac. Ut placerat orci nulla pellentesque dignissim enim sit amet venenatis. Orci nulla pellentesque dignissim enim sit amet. Tristique senectus et netus et malesuada fames ac. Vitae proin sagittis nisl rhoncus mattis rhoncus urna. Magna fermentum iaculis eu non diam phasellus vestibulum lorem sed. Massa ultricies mi quis hendrerit dolor. Mauris cursus mattis molestie a iaculis at erat. Sed elementum tempus egestas sed sed risus. Enim praesent elementum facilisis leo vel fringilla est ullamcorper. Malesuada fames ac turpis egestas maecenas pharetra convallis posuere morbi. At elementum eu facilisis sed odio morbi. Vitae aliquet nec ullamcorper sit amet risus nullam. Et ligula ullamcorper malesuada proin libero. Amet venenatis urna cursus eget nunc scelerisque viverra mauris. Donec pretium vulputate sapien nec sagittis aliquam malesuada bibendum. Convallis tellus id interdum velit laoreet id donec. Tristique senectus et netus et. Amet commodo nulla facilisi nullam vehicula ipsum a arcu.";
        }

        TextPage {
            text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Gravida in fermentum et sollicitudin ac. Ut placerat orci nulla pellentesque dignissim enim sit amet venenatis. Orci nulla pellentesque dignissim enim sit amet. Tristique senectus et netus et malesuada fames ac. Vitae proin sagittis nisl rhoncus mattis rhoncus urna. Magna fermentum iaculis eu non diam phasellus vestibulum lorem sed. Massa ultricies mi quis hendrerit dolor. Mauris cursus mattis molestie a iaculis at erat. Sed elementum tempus egestas sed sed risus. Enim praesent elementum facilisis leo vel fringilla est ullamcorper. Malesuada fames ac turpis egestas maecenas pharetra convallis posuere morbi. At elementum eu facilisis sed odio morbi. Vitae aliquet nec ullamcorper sit amet risus nullam. Et ligula ullamcorper malesuada proin libero. Amet venenatis urna cursus eget nunc scelerisque viverra mauris. Donec pretium vulputate sapien nec sagittis aliquam malesuada bibendum. Convallis tellus id interdum velit laoreet id donec. Tristique senectus et netus et. Amet commodo nulla facilisi nullam vehicula ipsum a arcu.";
            horizontal-stretch: 1;
            number-alignment: right;
            number: 2;
        }
    }
}
```