// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::models::{FileModel, FileType};
use crate::repositories::FilesRepository;

#[cfg(feature = "games")]
use crate::repositories::GamesRepository;

use crate::ui::*;
use chrono::{DateTime, Utc};
use slint::*;
use std::cell::{Cell, RefCell};
use std::collections::BTreeSet;
use std::rc::Rc;
use std::time::SystemTime;

mod context_menu {
    pub const OPEN: &str = "open";
    pub const RENAME: &str = "rename";
    pub const REMOVE: &str = "remove";
    pub const COPY: &str = "add-to-clipboard";
    pub const ADD_BOOKMARK: &str = "add-to-favorites";
    pub const PASTE: &str = "paste";
    pub const NEW_FOLDER: &str = "new-folder";

    #[cfg(feature = "games")]
    pub const CREATE_GAME_FILE: &str = "create-game-file";
    pub const ABOUT: &str = "about";
}

type FileCallback = Rc<RefCell<Box<dyn FnMut(&FileModel) + 'static>>>;

#[derive(Clone)]
pub struct FilesController {
    files_model: Rc<VecModel<FileModel>>,
    files_proxy_model: Rc<RefCell<ModelRc<FileModel>>>,
    view_handle: Weak<MainWindow>,
    repository: FilesRepository,
    #[cfg(feature = "games")]
    games_repository: GamesRepository,
    root_file: Rc<RefCell<FileModel>>,
    root_modified: Rc<Cell<Option<SystemTime>>>,
    selected_items: Rc<RefCell<BTreeSet<usize>>>,
    show_about_callback: Rc<RefCell<Box<dyn FnMut() + 'static>>>,
    add_bookmark_callback: FileCallback,
}

impl FilesController {
    pub fn new(
        root_file: FileModel,
        view_handle: Weak<MainWindow>,
        files_repository: FilesRepository,
        #[cfg(feature = "games")] games_repository: GamesRepository,
    ) -> Self {
        let files_model = Rc::new(VecModel::default());
        let files_proxy_model = Rc::new(RefCell::new(files_model_ext::sort_by_type(
            true,
            files_model_ext::filter_hidden_files(files_model.clone().into()),
        )));

        let root_modified = Rc::new(Cell::new(root_file.modified()));

        let controller = Self {
            root_file: Rc::new(RefCell::new(root_file)),
            files_model,
            files_proxy_model,
            view_handle,
            repository: files_repository,
            #[cfg(feature = "games")]
            games_repository,
            root_modified,
            selected_items: Rc::new(RefCell::new(BTreeSet::default())),
            show_about_callback: Rc::new(RefCell::new(Box::new(|| {}))),
            add_bookmark_callback: Rc::new(RefCell::new(Box::new(|_| {}))),
        };
        controller.update_view_files_model();

        upgrade_adapter(&controller.view_handle, {
            let controller = controller.clone();

            // connect show context menu
            move |adapter| {
                adapter.on_get_context_menu({
                    let controller = controller.clone();
                    move || controller.get_context_menu()
                });

                adapter.on_context_menu_action({
                    let controller = controller.clone();
                    move |spec| controller.execute_context_menu_action(spec.as_str())
                });

                adapter.on_get_item_context_menu({
                    let controller = controller.clone();
                    move |row| controller.get_item_context_menu(row as usize)
                });

                adapter.on_item_context_menu_action({
                    let controller = controller.clone();
                    move |row, spec| {
                        controller.execute_item_context_menu_action(row as usize, spec.as_str())
                    }
                });

                adapter.on_rename_item({
                    let controller = controller.clone();
                    move |row, new_name| controller.rename_item(row as usize, new_name.as_str())
                });

                adapter.on_update_selection({
                    let controller = controller.clone();
                    move |row, control| {
                        controller.update_selection(row as usize, control);
                    }
                });

                adapter.on_select_previous({
                    let controller = controller.clone();
                    move || controller.select_previous() as i32
                });

                adapter.on_select_next({
                    let controller = controller.clone();
                    move || controller.select_next() as i32
                });

                adapter.on_select_all({
                    let controller = controller.clone();
                    move || {
                        controller.select_all();
                    }
                });

                adapter.on_copy({
                    let controller = controller.clone();
                    move || {
                        controller.copy(None);
                    }
                });

                adapter.on_paste({
                    let controller = controller.clone();
                    move || {
                        controller.paste();
                    }
                });

                adapter.on_selected_items({
                    let controller = controller.clone();
                    move || {
                        VecModel::from_slice(
                            &controller
                                .selected_items
                                .borrow()
                                .iter()
                                .map(|row| *row as i32)
                                .collect::<Vec<i32>>(),
                        )
                    }
                });

                adapter.on_sort_ascending({
                    let controller = controller.clone();
                    move |column_index| controller.sort(true, column_index as usize)
                });

                adapter.on_sort_descending({
                    let controller = controller.clone();
                    move |column_index| controller.sort(false, column_index as usize)
                });
            }
        });

        controller.update_title();
        controller.load_files();

        controller
    }

    pub fn update(&self) {
        if self
            .root_modified
            .get()
            .eq(&self.root_file.borrow().modified())
        {
            return;
        }

        let files_model = self.files_model.clone();
        let repository: FilesRepository = self.repository.clone();
        let root_file = self.root_file.borrow().clone();
        self.root_modified.set(root_file.modified());

        let _ = slint::spawn_local(async move {
            if let Ok(mut repo_files) = repository.files(&root_file) {
                if repo_files.is_empty() {
                    files_model.set_vec(vec![]);
                    return;
                }

                for row in (0..files_model.row_count()).rev() {
                    if let Some(file_model) = files_model.row_data(row) {
                        if repo_files.contains(&file_model) {
                            repo_files
                                .remove(repo_files.iter().position(|f| f.eq(&file_model)).unwrap());
                            continue;
                        }

                        // file is no longer in the real directory but still in the ui (remove it)
                        files_model.remove(row);
                    }
                }

                // add new files to proxy
                for file in repo_files {
                    files_model.push(file);
                }
            }
        });
    }

    pub fn show_files(&self, root_file: FileModel) {
        self.clear_selection();
        self.root_modified.set(root_file.modified());
        *self.root_file.borrow_mut() = root_file;
        self.update_title();
        self.load_files();
    }

    pub fn set_back_enabled(&self, back_enabled: bool) {
        upgrade_adapter(&self.view_handle, move |adapter| {
            adapter.set_back_enabled(back_enabled);
        });
    }

    pub fn on_back(&self, func: impl FnMut() + 'static) {
        upgrade_adapter(&self.view_handle, move |adapter| {
            adapter.on_back(func);
        });
    }

    pub fn on_open_internal(&self, mut func: impl FnMut(FileModel) + 'static) {
        let files_proxy_model = self.files_proxy_model.clone();

        upgrade_adapter(&self.view_handle, move |adapter| {
            adapter.on_open_internal(move |row| {
                if let Some(file_model) = files_proxy_model.borrow().row_data(row as usize) {
                    func(file_model);
                }
            });
        });
    }

    pub fn on_show_about(&self, callback: impl FnMut() + 'static) {
        *self.show_about_callback.borrow_mut() = Box::new(callback);
    }

    pub fn on_add_bookmark(&self, callback: impl FnMut(&FileModel) + 'static) {
        *self.add_bookmark_callback.borrow_mut() = Box::new(callback);
    }

    pub fn open(&self, file_model: FileModel) {
        let _ = self.repository.open(file_model);
    }

    fn get_context_menu(&self) -> ModelRc<ListViewItem> {
        let items = VecModel::default();

        if self.root_file.borrow().is_dir()
            && !self.root_file.borrow().readonly().unwrap_or_default()
        {
            items.push(ListViewItem {
                text: "New folder".into(),
                spec: context_menu::NEW_FOLDER.into(),
                ..Default::default()
            });

            #[cfg(feature = "games")]
            if let Ok(is_games_dir) = self
                .games_repository
                .is_games_dir(self.root_file.borrow().path())
            {
                if !is_games_dir {
                    items.push(ListViewItem {
                        text: "Create game file".into(),
                        spec: context_menu::CREATE_GAME_FILE.into(),
                        ..Default::default()
                    });
                }
            }

            if self.repository.can_paste() {
                items.push(ListViewItem {
                    text: "Paste".into(),
                    spec: context_menu::PASTE.into(),
                    ..Default::default()
                });
            }
        }

        items.push(ListViewItem {
            text: "About".into(),
            spec: context_menu::ABOUT.into(),
            ..Default::default()
        });

        Rc::new(items).into()
    }

    fn execute_context_menu_action(&self, spec: &str) {
        match spec {
            context_menu::NEW_FOLDER => self.create_new_folder(),
            #[cfg(feature = "games")]
            context_menu::CREATE_GAME_FILE => self.create_game_file(),
            context_menu::PASTE => self.paste(),
            context_menu::ABOUT => {
                if let Ok(mut callback) = self.show_about_callback.try_borrow_mut() {
                    callback();
                }
            }
            _ => {}
        }
    }

    fn get_item_context_menu(&self, row: usize) -> ModelRc<ListViewItem> {
        let items = VecModel::default();

        if let Some(file_model) = self.files_proxy_model.borrow().row_data(row) {
            items.push(ListViewItem {
                text: "Open".into(),
                spec: context_menu::OPEN.into(),
                ..Default::default()
            });

            if let Ok(readonly) = file_model.readonly() {
                if !readonly {
                    items.push(ListViewItem {
                        text: "Rename".into(),
                        spec: context_menu::RENAME.into(),
                        ..Default::default()
                    });

                    items.push(ListViewItem {
                        text: "Move to bin".into(),
                        spec: context_menu::REMOVE.into(),
                        ..Default::default()
                    });
                }
            }

            items.push(ListViewItem {
                text: "Copy".into(),
                spec: context_menu::COPY.into(),
                ..Default::default()
            });

            if file_model.file_type().eq(&FileType::Dir) {
                items.push(ListViewItem {
                    text: "Add bookmark".into(),
                    spec: context_menu::ADD_BOOKMARK.into(),
                    ..Default::default()
                });
            }
        }

        Rc::new(items).into()
    }

    fn execute_item_context_menu_action(&self, row: usize, spec: &str) {
        match spec {
            context_menu::OPEN => {
                if let Some(file_model) = self.files_proxy_model.borrow().row_data(row) {
                    self.open(file_model);
                }
            }

            context_menu::REMOVE => {
                if !self.selected_items.borrow().contains(&row) {
                    if let Some(file_model) = self.files_proxy_model.borrow().row_data(row) {
                        if self.repository.remove(&file_model) {
                            files_model_ext::remove(&self.files_model, &file_model);
                        }
                    }
                } else {
                    let mut remove_files = vec![];

                    for r in self.selected_items.borrow().iter().rev() {
                        if let Some(file_model) = self.files_proxy_model.borrow().row_data(*r) {
                            remove_files.push(file_model);
                        }
                    }

                    self.repository.remove_all(&remove_files);

                    for file_model in remove_files {
                        files_model_ext::remove(&self.files_model, &file_model);
                    }
                }

                self.clear_selection();
                self.root_modified.set(self.root_file.borrow().modified());
            }
            context_menu::COPY => {
                self.copy(Some(row));
            }
            context_menu::RENAME => {
                upgrade_adapter(&self.view_handle, move |adapter| {
                    // editable is only column 1 (name)
                    adapter.set_edit_file((1, row as i32));
                });
            }
            context_menu::ADD_BOOKMARK => {
                if let Ok(mut callback) = self.add_bookmark_callback.try_borrow_mut() {
                    if let Some(file_model) = self.files_proxy_model.borrow().row_data(row) {
                        callback(&file_model);
                    }
                }
            }
            _ => {}
        }
    }

    fn rename_item(&self, row: usize, new_name: &str) {
        if let Some(file_model) = self.files_proxy_model.borrow().row_data(row) {
            if let Ok(renamed_file) = self.repository.rename(file_model, new_name.into()) {
                self.files_proxy_model
                    .borrow()
                    .set_row_data(row, renamed_file);
                self.root_modified.set(self.root_file.borrow().modified());
            }

            upgrade_adapter(&self.view_handle, |adapter| {
                adapter.set_edit_file((-1, -1));
            });
        }
    }

    fn update_selection(&self, row: usize, control: bool) {
        if !control {
            return;
        }

        if self.selected_items.borrow().contains(&row) {
            self.set_item_selected(row, false);
        } else {
            self.set_item_selected(row, true);
        }
    }

    fn select_previous(&self) -> usize {
        let previous = if let Some(first) = self.selected_items.borrow().last() {
            i32::max(0, *first as i32 - 1) as usize
        } else {
            0
        };

        self.clear_selection();
        self.set_item_selected(previous, true);

        if let Some(view_handle) = self.view_handle.upgrade() {
            view_handle.window().request_redraw();
        }

        previous
    }

    fn select_next(&self) -> usize {
        let next = if let Some(last) = self.selected_items.borrow().last() {
            i32::max(
                0,
                i32::min(
                    self.files_proxy_model.borrow().row_count() as i32 - 1,
                    *last as i32 + 1,
                ),
            ) as usize
        } else {
            0
        };

        self.clear_selection();
        self.set_item_selected(next, true);

        if let Some(view_handle) = self.view_handle.upgrade() {
            view_handle.window().request_redraw();
        }

        next
    }

    fn select_all(&self) {
        for r in 0..self.files_proxy_model.borrow().row_count() {
            self.set_item_selected(r, true);
        }
    }

    fn set_item_selected(&self, row: usize, selected: bool) {
        if let Some(mut file_model) = self.files_proxy_model.borrow().row_data(row) {
            if selected {
                self.selected_items.borrow_mut().insert(row);
            } else {
                self.selected_items.borrow_mut().remove(&row);
            }

            file_model.set_selected(selected);

            self.files_proxy_model
                .borrow()
                .set_row_data(row, file_model);
        }
    }

    fn clear_selection(&self) {
        for row in self.selected_items.borrow().iter() {
            if let Some(mut file_model) = self.files_proxy_model.borrow().row_data(*row) {
                file_model.set_selected(false);
                self.files_proxy_model
                    .borrow()
                    .set_row_data(*row, file_model);
            }
        }

        self.selected_items.borrow_mut().clear();
    }

    fn create_new_folder(&self) {
        let mut count = 1;

        loop {
            let name = std::format!("untitled folder-{count}");

            if self
                .repository
                .contains(&self.root_file.borrow(), name.as_str())
            {
                count += 1;
                continue;
            }

            if let Ok(new_folder) = self
                .repository
                .create_new_folder(&self.root_file.borrow(), name.as_str())
            {
                self.files_model.push(new_folder.clone());
                self.root_modified.set(self.root_file.borrow().modified());

                // FIXME: faster way to get the new added row
                if let Some(row) =
                    files_model_ext::row_of(&self.files_proxy_model.borrow(), &new_folder)
                {
                    upgrade_adapter(&self.view_handle, move |adapter| {
                        // editable is only column 1 (name)
                        adapter.set_edit_file((1, row as i32));
                    });
                }
            }

            break;
        }
    }

    #[cfg(feature = "games")]
    fn create_game_file(&self) {
        if let Ok(new_file) = self
            .repository
            .create_empty_text_file(&self.root_file.borrow(), "coop_game.toml")
        {
            self.files_model.push(new_file.clone());
            self.root_modified.set(self.root_file.borrow().modified());
        }
    }

    fn copy(&self, row: Option<usize>) {
        if let Some(row) = row {
            if !self.selected_items.borrow().contains(&row) {
                if let Some(file_model) = self.files_proxy_model.borrow().row_data(row) {
                    self.repository.copy(&[file_model]);
                    return;
                }
            }
        }

        let mut copy_files = vec![];

        for r in self.selected_items.borrow().iter().rev() {
            if let Some(file_model) = self.files_proxy_model.borrow().row_data(*r) {
                copy_files.push(file_model);
            }
        }

        self.repository.copy(&copy_files);
    }

    fn paste(&self) {
        if !self.repository.can_paste() {
            return;
        }

        if let Ok(new_files) = self.repository.paste(&self.root_file.borrow()) {
            for new_file in new_files {
                self.files_model.push(new_file);
            }

            self.root_modified.set(self.root_file.borrow().modified());
        }
    }

    fn load_files(&self) {
        let files_model = self.files_model.clone();
        let repository: FilesRepository = self.repository.clone();
        let root_file = self.root_file.borrow().clone();
        let view_handle = self.view_handle.clone();

        upgrade_adapter(&view_handle, |adapter| {
            adapter.set_loading(true);
        });

        let _ = slint::spawn_local(async move {
            if let Ok(repo_files) = repository.files(&root_file) {
                files_model.set_vec(repo_files);
            }

            upgrade_adapter(&view_handle, |adapter| {
                adapter.set_loading(false);
            });
        });
    }

    fn update_title(&self) {
        let title = self.root_file.borrow().name().unwrap_or_default().into();
        upgrade_adapter(&self.view_handle, move |adapter| {
            adapter.set_title(title);
        });
    }

    fn sort(&self, ascending: bool, column_index: usize) -> bool {
        let sorted = match column_index {
            1 => {
                *self.files_proxy_model.borrow_mut() = files_model_ext::sort_by_name(
                    ascending,
                    files_model_ext::filter_hidden_files(self.files_model.clone().into()),
                );
                true
            }
            2 => {
                *self.files_proxy_model.borrow_mut() = files_model_ext::sort_by_size(
                    ascending,
                    files_model_ext::filter_hidden_files(self.files_model.clone().into()),
                );
                true
            }
            3 => {
                *self.files_proxy_model.borrow_mut() = files_model_ext::sort_by_date_modified(
                    ascending,
                    files_model_ext::filter_hidden_files(self.files_model.clone().into()),
                );
                true
            }
            _ => false,
        };

        if sorted {
            self.update_view_files_model();
        }

        sorted
    }

    fn update_view_files_model(&self) {
        let files_proxy_model = self.files_proxy_model.borrow().clone();
        let view_handle = self.view_handle.clone();

        upgrade_adapter(&self.view_handle, move |adapter| {
            adapter.set_files(
                Rc::new(files_proxy_model.map(move |f: FileModel| TableRowItem {
                    model: VecModel::from_slice(&[
                        ListViewItem {
                            leading_icon: item_type_to_icon(&view_handle, f.file_type()),
                            highlighted: f.is_dir(),
                            ..Default::default()
                        },
                        ListViewItem {
                            text: f.name().unwrap_or_default().into(),
                            ..Default::default()
                        },
                        ListViewItem {
                            text: {
                                if !f.is_dir() {
                                    if let Some(len) = f.len() {
                                        std::format!("{} kb", (len as f64 * 0.001).round() as u64)
                                            .into()
                                    } else {
                                        SharedString::from("--")
                                    }
                                } else {
                                    SharedString::from("--")
                                }
                            },
                            ..Default::default()
                        },
                        ListViewItem {
                            text: if let Some(modified) = f.modified() {
                                let date_time: DateTime<Utc> = modified.into();
                                date_time.format("%Y-%m-%d %H:%M").to_string().into()
                            } else {
                                "--".into()
                            },
                            ..Default::default()
                        },
                    ]),
                    selected: f.selected(),
                }))
                .into(),
            )
        });
    }
}

fn upgrade_adapter(view_handle: &Weak<MainWindow>, func: impl FnOnce(FilesAdapter) + 'static) {
    if let Some(view) = view_handle.upgrade() {
        func(view.global::<FilesAdapter>());
    }
}

fn item_type_to_icon(view_handle: &Weak<MainWindow>, file_type: FileType) -> Image {
    if let Some(main_window) = view_handle.upgrade() {
        return match file_type {
            FileType::Dir => main_window.global::<Icons>().get_filled_folder(),
            FileType::Text => main_window.global::<Icons>().get_filled_description(),
            FileType::Image => main_window.global::<Icons>().get_filled_image(),
            FileType::Audio => main_window.global::<Icons>().get_filled_audio_file(),
            FileType::Video => main_window.global::<Icons>().get_filled_video_file(),
            FileType::Unknown => main_window.global::<Icons>().get_filled_insert_drive_file(),
        };
    }

    Image::default()
}

mod files_model_ext {
    use super::*;

    pub fn filter_hidden_files(model: ModelRc<FileModel>) -> ModelRc<FileModel> {
        Rc::new(model.filter(|f| !f.hidden())).into()
    }

    pub fn sort_by_type(ascending: bool, model: ModelRc<FileModel>) -> ModelRc<FileModel> {
        Rc::new(model.sort_by(move |l: &FileModel, r: &FileModel| {
            if l.file_type() != r.file_type()
                && (l.file_type() == FileType::Dir || r.file_type() == FileType::Dir)
            {
                if ascending {
                    return l.file_type().cmp(&r.file_type());
                } else {
                    return r.file_type().cmp(&l.file_type());
                }
            }
            l.name()
                .unwrap_or_default()
                .to_lowercase()
                .cmp(&r.name().unwrap_or_default().to_lowercase())
        }))
        .into()
    }

    pub fn sort_by_name(ascending: bool, model: ModelRc<FileModel>) -> ModelRc<FileModel> {
        Rc::new(model.sort_by(move |l: &FileModel, r: &FileModel| {
            if ascending {
                l.name()
                    .unwrap_or_default()
                    .to_lowercase()
                    .cmp(&r.name().unwrap_or_default().to_lowercase())
            } else {
                r.name()
                    .unwrap_or_default()
                    .to_lowercase()
                    .cmp(&l.name().unwrap_or_default().to_lowercase())
            }
        }))
        .into()
    }

    pub fn sort_by_size(ascending: bool, model: ModelRc<FileModel>) -> ModelRc<FileModel> {
        Rc::new(model.sort_by(move |l: &FileModel, r: &FileModel| {
            if ascending {
                l.len().cmp(&r.len())
            } else {
                r.len().cmp(&l.len())
            }
        }))
        .into()
    }

    pub fn sort_by_date_modified(ascending: bool, model: ModelRc<FileModel>) -> ModelRc<FileModel> {
        Rc::new(model.sort_by(move |l: &FileModel, r: &FileModel| {
            if ascending {
                l.modified().cmp(&r.modified())
            } else {
                r.modified().cmp(&l.modified())
            }
        }))
        .into()
    }

    pub fn remove(model: &Rc<VecModel<FileModel>>, value: &FileModel) -> Option<FileModel> {
        for i in 0..model.row_count() {
            if let Some(source_value) = model.row_data(i) {
                if !source_value.eq(value) {
                    continue;
                }
            }

            return Some(model.remove(i));
        }

        None
    }

    pub fn row_of(model: &ModelRc<FileModel>, data: &FileModel) -> Option<usize> {
        // FIXME: find faster way to get row of an item
        for row in 0..model.row_count() {
            if let Some(row_data) = model.row_data(row) {
                if row_data.eq(data) {
                    return Some(row);
                }
            }
        }
        None
    }
}
