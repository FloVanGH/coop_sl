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
