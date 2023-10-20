<!--
SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: GPL-3.0-only
-->

# coop_local

A cross platform file browser build with [Slint](https://slint.dev/).

<a href="https://codeberg.org/flovansl/co_sl">
    <img alt="Get it on Codeberg" src="https://get-it-on.codeberg.org/get-it-on-blue-on-white.png" height="60">
</a>
<a href="https://slint.dev">
    <img alt="#MadeWithSlint" src="https://raw.githubusercontent.com/slint-ui/slint/master/logo//MadeWithSlint-logo-light.svg" height="60">
</a>

[![GNU GPLv3](https://img.shields.io/badge/license-GPLv3-green.svg)](../../LICENSES/GPL-3.0-only.txt)


## Disclaimer

**This is a hobby project and still early in development. It is not recommended to use it in production. Lost of files during usage could be possible.**

## About

The project is mainly a hobby project of myself. I have started it to evaluate a good way for an application architecture with `Slint` and `Rust`.
Now I'm on to create a file browser that fits my personal needs and taste. So it will be including extra features like. a game launcher, music player and image viewer.
Also if there are some features implemented now the project is still in early stage. As mentioned it's mainly a personal project but if someone else is interested in `coop_local` you can check further topics and progress on https://codeberg.org/flovansl/co_sl/issues/33.

 <img alt="coop_local" src="https://codeberg.org/flovansl/pages//attachments/9ba12373-c14d-4016-a875-401b9c3df3d0" width="320" />

## Features

* Create bookmarks
* View, copy, rename, remove files and folders
* Create new folder
* View images
* View and edit texts
* Integrated game launcher
* Advanced keyboard navigation
    * Optimized tab navigation
    * Toggle selection of single entries with mouse left button and ctrl/command key
    * Select multiple files at once with mouse left button and shift key
    * Move selected entry with arrow key up and down
    * Add more entries to selection with shift key and arrow up / down key
* Optimized to use with a gamepad

## Input

coop_local can be controlled by mouse, touchpad, touchscreen and a game controller.

## Implementation details

### Architecture

The application architecture of `coop_local` is based on the [MVC pattern](https://en.wikipedia.org/wiki/Model–view–controller). The following structure is used in the whole application:

* `{ViewName}View`: the Slint component that represents the `View` (location `ui/views/{view-name}-view.slint`).
* `{ViewName}Adapter`: a global that defines the interface between the view and the controller (location `ui/views/{view-name}-view.slint`).
* `{ViewName}Controller`: a rust struct that represents the `Controller` (location `src/controller/{view_name}_controller.rs`).
* `{ModelName}Model`: a rust struct that represents the `Model` (location `src/models/{model_name}_model.rs`).
* `{RepositoryName}Repository`: reads and changes data convert it into a model and provide it to the controller (location `src/repositories/{_repository_name}_repository.rs`).

#### Example

* `FilesView`: Slint component that displays the files of the current directory in a `StandardTableView`.
* `FilesAdapter`: Slint global that defines the interface between `FilesView` and `FilesController`.
* `FilesController`: Rust struct that set / update the list of `FileModel` of `FilesView` and reacting to input from the `FilesView`.
* `FileRepository`: Reads, copy, remove and rename files. Creates a list of `FileModels`. Is used by `FilesController`.
* `FileModel`: Represents a single file.

## Dependencies

### Linux

* To run coop_local on Linux with `games` feature `libudev-dev` mus be installed.

## Run

* Run from root `cargo run -p coop_local`

## License

* Source code an binaries of this projects are available under [GNU GPLv3](../../LICENSES/GPL-3.0-only.txt) license.
