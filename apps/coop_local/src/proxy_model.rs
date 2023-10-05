// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use slint::*;
use std::rc::Rc;

pub struct ProxyModel<T: Clone + Default + 'static> {
    source: Rc<VecModel<T>>,
    wrapper: ModelRc<T>,
}

impl<T: Clone + Default + PartialEq> Default for ProxyModel<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + Default + PartialEq> ProxyModel<T> {
    pub fn new() -> Self {
        let source = Rc::new(VecModel::default());

        ProxyModel {
            source: source.clone(),
            wrapper: source.into(),
        }
    }

    pub fn push_to_source(&self, value: T) {
        self.source.push(value);
    }

    pub fn remove_from_source(&self, value: T) -> Option<T> {
        for i in 0..self.source.row_count() {
            if let Some(source_value) = self.source.row_data(i) {
                if !source_value.eq(&value) {
                    continue;
                }
            }

            return Some(self.source.remove(i));
        }

        None
    }

    pub fn set_vec_to_source(&self, new: impl Into<Vec<T>>) {
        self.source.set_vec(new);
    }

    pub fn row_data_from_source(&self, row: usize) -> Option<T> {
        self.source.row_data(row)
    }

    pub fn row_count_from_source(&self) -> usize {
        self.source.row_count()
    }

    pub fn as_sort_by<F>(self, sort_function: F) -> Self
    where
        F: FnMut(&T, &T) -> core::cmp::Ordering + 'static,
    {
        Self {
            source: self.source,
            wrapper: Rc::new(self.wrapper.clone().sort_by(sort_function)).into(),
        }
    }

    pub fn row_of(&self, data: &T) -> Option<usize> {
        // FIXME: find faster way to get row of an item
        for row in 0..self.row_count() {
            if let Some(row_data) = self.row_data(row) {
                if row_data.eq(data) {
                    return Some(row);
                }
            }
        }
        None
    }

    pub fn as_filter_by<F>(self, filter_function: F) -> Self
    where
        F: Fn(&T) -> bool + 'static,
    {
        Self {
            source: self.source,
            wrapper: Rc::new(self.wrapper.clone().filter(filter_function)).into(),
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        for row in 0..self.row_count() {
            if let Some(row_data) = self.row_data(row) {
                if row_data.eq(value) {
                    return true;
                }
            }
        }

        false
    }

    pub fn clear(&self) {
        self.source.set_vec(vec![]);
    }
}

impl<T: Clone + Default> Model for ProxyModel<T> {
    type Data = T;

    fn row_count(&self) -> usize {
        self.wrapper.row_count()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        self.wrapper.row_data(row)
    }

    fn model_tracker(&self) -> &dyn ModelTracker {
        self.wrapper.model_tracker()
    }

    fn set_row_data(&self, row: usize, data: Self::Data) {
        self.wrapper.set_row_data(row, data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::FileModel;

    #[test]
    fn test_proxy_model() {
        let proxy_model = ProxyModel::new();
        proxy_model.set_vec_to_source(vec![
            FileModel::new("/root/1.png"),
            FileModel::new("/root/2"),
            FileModel::new("/root/3"),
            FileModel::new("/root/4"),
        ]);

        assert_eq!(proxy_model.row_count_from_source(), 4);
        assert_eq!(proxy_model.row_count(), 4);

        assert_eq!(proxy_model.row_data(0), Some(FileModel::new("/root/1.png")));
        assert_eq!(proxy_model.row_data(5), None);

        proxy_model.set_row_data(0, FileModel::new("/root/new/1.png"));
        assert_eq!(
            proxy_model.row_data(0),
            Some(FileModel::new("/root/new/1.png"))
        );

        assert_eq!(
            proxy_model.row_data_from_source(1),
            Some(FileModel::new("/root/2"))
        );

        assert_eq!(
            proxy_model.remove_from_source(FileModel::new("/root/2")),
            Some(FileModel::new("/root/2"))
        );
        assert_eq!(
            proxy_model.remove_from_source(FileModel::new("/not/in/files")),
            None
        );
        assert_eq!(proxy_model.row_count_from_source(), 3);
        assert_eq!(proxy_model.row_count(), 3);
        assert_eq!(
            proxy_model.row_data_from_source(1),
            Some(FileModel::new("/root/3"))
        );

        proxy_model.push_to_source(FileModel::new("/root/5"));
        assert_eq!(proxy_model.row_count_from_source(), 4);
        assert_eq!(proxy_model.row_count(), 4);
        assert_eq!(
            proxy_model.row_data_from_source(3),
            Some(FileModel::new("/root/5"))
        );
    }

    #[test]
    fn test_proxy_model_sorted() {
        let proxy_model = ProxyModel::new()
            .as_sort_by(|l: &FileModel, r: &FileModel| l.name().unwrap().cmp(r.name().unwrap()));

        proxy_model.set_vec_to_source(vec![
            FileModel::new("/root/5"),
            FileModel::new("/root/3"),
            FileModel::new("/root/4"),
            FileModel::new("/root/1"),
        ]);

        assert_eq!(proxy_model.row_count_from_source(), 4);
        assert_eq!(proxy_model.row_count(), 4);

        assert_eq!(proxy_model.row_data(0), Some(FileModel::new("/root/1")));
        assert_eq!(
            proxy_model.row_data_from_source(0),
            Some(FileModel::new("/root/5"))
        );

        proxy_model.set_row_data(3, FileModel::new("/root/0"));
        assert_eq!(proxy_model.row_data(0), Some(FileModel::new("/root/0")));
        assert_eq!(
            proxy_model.row_data_from_source(0),
            Some(FileModel::new("/root/0"))
        );

        assert_eq!(
            proxy_model.remove_from_source(FileModel::new("/root/0")),
            Some(FileModel::new("/root/0"))
        );

        assert_eq!(proxy_model.row_count_from_source(), 3);
        assert_eq!(proxy_model.row_count(), 3);

        proxy_model.push_to_source(FileModel::new("/root/9"));

        assert_eq!(proxy_model.row_count_from_source(), 4);
        assert_eq!(proxy_model.row_count(), 4);

        assert_eq!(proxy_model.row_data(3), Some(FileModel::new("/root/9")));
        assert_eq!(
            proxy_model.row_data_from_source(3),
            Some(FileModel::new("/root/9"))
        );
    }

    #[test]
    fn test_proxy_model_filtered() {
        let proxy_model =
            ProxyModel::new().as_filter_by(|f: &FileModel| !f.name().unwrap().contains("filtered"));

        proxy_model.set_vec_to_source(vec![
            FileModel::new("/root/filtered5"),
            FileModel::new("/root/3"),
            FileModel::new("/root/filtered4"),
            FileModel::new("/root/1"),
        ]);

        assert_eq!(proxy_model.row_count_from_source(), 4);
        assert_eq!(proxy_model.row_count(), 2);

        assert_eq!(proxy_model.row_data(0), Some(FileModel::new("/root/3")));
        assert_eq!(
            proxy_model.row_data_from_source(0),
            Some(FileModel::new("/root/filtered5"))
        );

        proxy_model.set_row_data(0, FileModel::new("/root/filtered3"));
        assert_eq!(proxy_model.row_count_from_source(), 4);
        assert_eq!(proxy_model.row_count(), 1);

        assert_eq!(proxy_model.row_data(0), Some(FileModel::new("/root/1")));
        assert_eq!(
            proxy_model.row_data_from_source(0),
            Some(FileModel::new("/root/filtered5"))
        );

        proxy_model.push_to_source(FileModel::new("/root/9"));

        assert_eq!(proxy_model.row_count_from_source(), 5);
        assert_eq!(proxy_model.row_count(), 2);

        assert_eq!(proxy_model.row_data(1), Some(FileModel::new("/root/9")));
        assert_eq!(
            proxy_model.row_data_from_source(4),
            Some(FileModel::new("/root/9"))
        );

        proxy_model.push_to_source(FileModel::new("/root/filtered9"));

        assert_eq!(proxy_model.row_count_from_source(), 6);
        assert_eq!(proxy_model.row_count(), 2);

        assert_eq!(proxy_model.row_data(1), Some(FileModel::new("/root/9")));
        assert_eq!(
            proxy_model.row_data_from_source(5),
            Some(FileModel::new("/root/filtered9"))
        );

        proxy_model.remove_from_source(FileModel::new("/root/filtered9"));

        assert_eq!(proxy_model.row_count_from_source(), 5);
        assert_eq!(proxy_model.row_count(), 2);

        assert_eq!(proxy_model.row_data(1), Some(FileModel::new("/root/9")));
        assert_eq!(
            proxy_model.row_data_from_source(4),
            Some(FileModel::new("/root/9"))
        );

        proxy_model.remove_from_source(FileModel::new("/root/1"));

        assert_eq!(proxy_model.row_count_from_source(), 4);
        assert_eq!(proxy_model.row_count(), 1);
    }
}
