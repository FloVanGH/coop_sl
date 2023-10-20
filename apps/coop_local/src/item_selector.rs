// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use slint::*;
use std::collections::BTreeSet;

pub trait Selectable {
    fn selected(&self) -> bool;
    fn set_selected(&mut self, selected: bool);
}

pub struct ItemSelector<T: Selectable> {
    model: ModelRc<T>,
    selected_items: BTreeSet<usize>,
    selection_start: usize,
    selection_end: usize,
    shift_index: usize,
}

impl<T: Selectable> ItemSelector<T> {
    pub fn new(model: ModelRc<T>) -> Self {
        Self {
            model,
            selected_items: BTreeSet::default(),
            selection_start: 0,
            selection_end: 0,
            shift_index: 0,
        }
    }

    pub fn is_selected(&self, row: &usize) -> bool {
        self.selected_items.contains(row)
    }

    pub fn toggle_selection(&mut self, row: usize) {
        if self.is_out_of_bounds(row) {
            return;
        }

        if !self.is_selected(&row) {
            self.select(row);
            self.selection_start = row;
            self.selection_end = row;
            return;
        }

        self.unselect(row);

        if let Some(last) = self.selected_items.last() {
            self.selection_start = *last;
        } else {
            self.selection_start = 0;
        }
    }

    pub fn select_next(&mut self) -> usize {
        let mut next = if self.selected_items.is_empty() {
            0
        } else {
            self.selection_end + 1
        };

        if self.is_out_of_bounds(next) {
            next = self.selection_end;
        }

        self.clear_selection();
        self.select(next);
        self.selection_start = next;
        self.selection_end = next;

        next
    }

    pub fn select_previous(&mut self) -> usize {
        let previous = if self.selected_items.is_empty() || self.selection_end == 0 {
            0
        } else {
            self.selection_end - 1
        };

        self.clear_selection();
        self.select(previous);
        self.selection_start = previous;
        self.selection_end = previous;

        previous
    }

    pub fn shift_selection(&mut self, row: usize) {
        if self.is_out_of_bounds(row) {
            return;
        }

        let selection_start = self.selection_start;

        self.clear_selection();

        self.selection_start = selection_start;
        if selection_start == row {
            self.select(row);
            return;
        }

        if self.selection_start < row {
            for r in self.selection_start..(row + 1) {
                self.select(r);
            }
        } else {
            for r in row..(self.selection_start + 1) {
                self.select(r);
            }
        }

        self.selection_end = row;
    }

    pub fn shift_selection_next(&mut self) -> usize {
        if self.is_empty() {
            self.toggle_selection(0);
            return 0;
        }

        if self.selection_end < self.selection_start {
            self.unselect(self.selection_end);
            self.selection_end += 1;
        } else {
            let next = self.selection_end + 1;

            if !self.is_out_of_bounds(next) {
                self.select(next);
                self.selection_end = next;
            }
        }

        self.selection_end
    }

    pub fn shift_selection_previous(&mut self) -> usize {
        if self.is_empty() {
            if self.model.row_count() > 0 {
                self.toggle_selection(self.model.row_count() - 1);
                return self.selection_end;
            }

            return 0;
        }

        if self.selection_end > self.selection_start {
            self.unselect(self.selection_end);
            self.selection_end -= 1;
        } else if self.selection_end > 0 {
            let previous = self.selection_end - 1;
            self.select(previous);
            self.selection_end = previous;
        }

        self.selection_end
    }

    pub fn select_all(&mut self) {
        if self.model.row_count() == 0 {
            return;
        }

        let row_count = self.model.row_count();

        for row in 0..row_count {
            self.select(row);
        }

        self.selection_start = 0;
        if let Some(last) = self.selected_items.last() {
            self.selection_end = *last;
        }
    }

    pub fn clear_selection(&mut self) {
        if self.model.row_count() == 0 {
            return;
        }

        for row in self.selected_items.iter() {
            let item = self.model.row_data(*row);
            if let Some(mut item) = item {
                item.set_selected(false);
                self.model.set_row_data(*row, item);
            }
        }

        self.selection_start = 0;
        self.selection_end = 0;
        self.shift_index = 0;
        self.selected_items.clear();
    }

    pub fn selected_rows(&self) -> &BTreeSet<usize> {
        &self.selected_items
    }

    pub fn len(&self) -> usize {
        self.selected_items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.selected_items.is_empty()
    }

    pub fn as_vec(&self) -> Vec<i32> {
        self.selected_items
            .iter()
            .map(|row| *row as i32)
            .collect::<Vec<i32>>()
    }

    fn select(&mut self, row: usize) {
        let item = self.model.row_data(row);
        if let Some(mut item) = item {
            item.set_selected(true);
            self.model.set_row_data(row, item);
            self.selected_items.insert(row);
        }
    }

    fn unselect(&mut self, row: usize) {
        let item = self.model.row_data(row);
        if let Some(mut item) = item {
            item.set_selected(false);
            self.model.set_row_data(row, item);
        }

        self.selected_items.remove(&row);
    }

    fn is_out_of_bounds(&self, row: usize) -> bool {
        row >= self.model.row_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Copy, Clone, Debug, Default, PartialEq)]
    struct MockSelectable {
        selected: bool,
    }

    impl Selectable for MockSelectable {
        fn selected(&self) -> bool {
            self.selected
        }

        fn set_selected(&mut self, selected: bool) {
            self.selected = selected;
        }
    }

    #[test]
    fn test_toggle_selection() {
        let mut selector =
            ItemSelector::new(VecModel::from_slice(&[MockSelectable::default(); 10]));

        assert!(!selector.is_selected(&0));
        assert!(selector.is_empty());

        selector.toggle_selection(0);
        assert_eq!(selector.len(), 1);
        assert_eq!(
            selector.model.row_data(0),
            Some(MockSelectable { selected: true })
        );
        assert_eq!(selector.selection_start, 0);

        selector.toggle_selection(0);
        assert!(selector.is_empty());

        selector.toggle_selection(1);
        selector.toggle_selection(3);
        selector.toggle_selection(5);
        assert_eq!(selector.len(), 3);
        assert_eq!(selector.selection_start, 5);
        let mut selected_items = BTreeSet::new();
        selected_items.insert(1);
        selected_items.insert(3);
        selected_items.insert(5);
        assert_eq!(selector.selected_items, selected_items);

        selector.toggle_selection(3);
        assert_eq!(selector.len(), 2);
        assert_eq!(selector.selection_start, 5);
        let mut selected_items = BTreeSet::new();
        selected_items.insert(1);
        selected_items.insert(5);
        assert_eq!(selector.selected_items, selected_items);

        selector.toggle_selection(5);
        assert_eq!(selector.len(), 1);
        assert_eq!(selector.selection_start, 1);
        let mut selected_items = BTreeSet::new();
        selected_items.insert(1);
        assert_eq!(selector.selected_items, selected_items);

        selector.toggle_selection(10);
        assert_eq!(selector.len(), 1);
        assert_eq!(selector.selection_start, 1);
        assert_eq!(selector.selected_items, selected_items);

        let mut selector: ItemSelector<MockSelectable> =
            ItemSelector::new(VecModel::from_slice(&[]));

        selector.toggle_selection(0);
        assert_eq!(selector.len(), 0);
        assert_eq!(selector.selection_start, 0);
        assert_eq!(selector.selected_items, BTreeSet::default());
    }

    #[test]
    fn test_select_all() {
        let mut selector =
            ItemSelector::new(VecModel::from_slice(&[MockSelectable::default(); 10]));
        selector.select_all();
        assert_eq!(selector.len(), 10);

        for r in 0..selector.model.row_count() {
            assert_eq!(
                selector.model.row_data(r),
                Some(MockSelectable { selected: true })
            );
        }

        selector.clear_selection();
        assert_eq!(selector.len(), 0);

        for r in 0..selector.model.row_count() {
            assert_eq!(
                selector.model.row_data(r),
                Some(MockSelectable { selected: false })
            );
        }
    }

    #[test]
    fn test_select_next() {
        let mut selector =
            ItemSelector::new(VecModel::from_slice(&[MockSelectable::default(); 10]));

        assert_eq!(selector.select_next(), 0);
        assert_eq!(selector.len(), 1);
        assert_eq!(
            selector.model.row_data(0),
            Some(MockSelectable { selected: true })
        );
        assert_eq!(selector.selection_start, 0);

        assert_eq!(selector.select_next(), 1);
        assert_eq!(selector.len(), 1);
        assert_eq!(
            selector.model.row_data(0),
            Some(MockSelectable { selected: false })
        );
        assert_eq!(
            selector.model.row_data(1),
            Some(MockSelectable { selected: true })
        );
        assert_eq!(selector.selection_start, 1);

        assert_eq!(selector.select_next(), 2);
        assert_eq!(selector.len(), 1);
        assert_eq!(
            selector.model.row_data(1),
            Some(MockSelectable { selected: false })
        );
        assert_eq!(
            selector.model.row_data(2),
            Some(MockSelectable { selected: true })
        );
        assert_eq!(selector.selection_start, 2);

        selector.toggle_selection(4);
        assert_eq!(selector.select_next(), 5);
        assert_eq!(selector.len(), 1);
        assert_eq!(
            selector.model.row_data(4),
            Some(MockSelectable { selected: false })
        );
        assert_eq!(
            selector.model.row_data(5),
            Some(MockSelectable { selected: true })
        );
        assert_eq!(selector.selection_start, 5);

        selector.toggle_selection(9);
        assert_eq!(selector.select_next(), 9);
        assert_eq!(selector.len(), 1);
        assert_eq!(
            selector.model.row_data(9),
            Some(MockSelectable { selected: true })
        );
        assert_eq!(selector.selection_start, 9);

        selector.select_all();
        assert_eq!(selector.select_next(), 9);
        assert_eq!(selector.len(), 1);
        assert_eq!(
            selector.model.row_data(9),
            Some(MockSelectable { selected: true })
        );
        assert_eq!(selector.selection_start, 9);
        assert_eq!(selector.selection_end, 9);

        selector.toggle_selection(4);
        selector.shift_selection(2);
        assert_eq!(selector.select_next(), 3);
        assert_eq!(
            selector.model.row_data(3),
            Some(MockSelectable { selected: true })
        );

        selector.toggle_selection(2);
        selector.shift_selection(4);
        assert_eq!(selector.select_next(), 5);
        assert_eq!(
            selector.model.row_data(5),
            Some(MockSelectable { selected: true })
        );
    }

    #[test]
    fn test_select_previous() {
        let mut selector =
            ItemSelector::new(VecModel::from_slice(&[MockSelectable::default(); 10]));

        selector.toggle_selection(selector.model.row_count() - 1);
        assert_eq!(selector.select_previous(), 8);
        assert_eq!(selector.len(), 1);
        assert_eq!(
            selector.model.row_data(8),
            Some(MockSelectable { selected: true })
        );
        assert_eq!(selector.selection_start, 8);

        assert_eq!(selector.select_previous(), 7);
        assert_eq!(selector.len(), 1);
        assert_eq!(
            selector.model.row_data(8),
            Some(MockSelectable { selected: false })
        );
        assert_eq!(
            selector.model.row_data(7),
            Some(MockSelectable { selected: true })
        );
        assert_eq!(selector.selection_start, 7);

        selector.toggle_selection(4);
        assert_eq!(selector.select_previous(), 3);
        assert_eq!(selector.len(), 1);
        assert_eq!(
            selector.model.row_data(4),
            Some(MockSelectable { selected: false })
        );
        assert_eq!(
            selector.model.row_data(3),
            Some(MockSelectable { selected: true })
        );
        assert_eq!(selector.selection_start, 3);

        selector.toggle_selection(0);
        assert_eq!(selector.select_previous(), 0);
        assert_eq!(selector.len(), 1);
        assert_eq!(
            selector.model.row_data(0),
            Some(MockSelectable { selected: true })
        );
        assert_eq!(selector.selection_start, 0);

        selector.select_all();
        assert_eq!(selector.select_previous(), 8);
        assert_eq!(selector.len(), 1);
        assert_eq!(
            selector.model.row_data(8),
            Some(MockSelectable { selected: true })
        );
        assert_eq!(selector.selection_start, 8);
        assert_eq!(selector.selection_end, 8);

        selector.toggle_selection(4);
        selector.shift_selection(2);
        assert_eq!(selector.select_previous(), 1);
        assert_eq!(
            selector.model.row_data(1),
            Some(MockSelectable { selected: true })
        );

        selector.toggle_selection(2);
        selector.shift_selection(4);
        assert_eq!(selector.select_previous(), 3);
        assert_eq!(
            selector.model.row_data(3),
            Some(MockSelectable { selected: true })
        );
    }

    #[test]
    fn test_shift_selection() {
        let mut selector =
            ItemSelector::new(VecModel::from_slice(&[MockSelectable::default(); 10]));

        selector.shift_selection(10);
        assert_eq!(selector.len(), 0);
        assert_eq!(selector.selection_start, 0);

        selector.toggle_selection(3);
        selector.shift_selection(6);
        assert_eq!(selector.len(), 4);
        let mut selected_items = BTreeSet::new();
        selected_items.insert(3);
        selected_items.insert(4);
        selected_items.insert(5);
        selected_items.insert(6);
        assert_eq!(selector.selected_items, selected_items);

        selector.toggle_selection(8);
        selector.shift_selection(6);
        assert_eq!(selector.len(), 3);
        let mut selected_items = BTreeSet::new();
        selected_items.insert(6);
        selected_items.insert(7);
        selected_items.insert(8);
        assert_eq!(selector.selected_items, selected_items);
    }

    #[test]
    fn test_shift_selection_next() {
        let mut selector =
            ItemSelector::new(VecModel::from_slice(&[MockSelectable::default(); 10]));

        assert_eq!(selector.shift_selection_next(), 0);
        assert_eq!(selector.len(), 1);
        assert_eq!(
            selector.model.row_data(0),
            Some(MockSelectable { selected: true })
        );
        assert_eq!(selector.selection_start, 0);

        selector.toggle_selection(4);
        selector.shift_selection(2);
        assert_eq!(selector.shift_selection_next(), 3);
        assert_eq!(selector.selection_start, 4);
        assert_eq!(selector.selection_end, 3);
        assert_eq!(selector.len(), 2);
        let mut selected_items = BTreeSet::new();
        selected_items.insert(3);
        selected_items.insert(4);
        assert_eq!(selector.selected_items, selected_items);

        assert_eq!(selector.shift_selection_next(), 4);
        assert_eq!(selector.selection_start, 4);
        assert_eq!(selector.selection_end, 4);
        assert_eq!(selector.len(), 1);

        assert_eq!(selector.shift_selection_next(), 5);
        assert_eq!(selector.selection_start, 4);
        assert_eq!(selector.selection_end, 5);
        assert_eq!(selector.len(), 2);
        let mut selected_items = BTreeSet::new();
        selected_items.insert(4);
        selected_items.insert(5);
        assert_eq!(selector.selected_items, selected_items);

        assert_eq!(selector.shift_selection_next(), 6);
        assert_eq!(selector.selection_start, 4);
        assert_eq!(selector.selection_end, 6);
        assert_eq!(selector.len(), 3);
        let mut selected_items = BTreeSet::new();
        selected_items.insert(4);
        selected_items.insert(5);
        selected_items.insert(6);
        assert_eq!(selector.selected_items, selected_items);

        selector.select_all();
        assert_eq!(selector.shift_selection_next(), 9);
        assert_eq!(selector.selection_start, 0);
        assert_eq!(selector.selection_end, 9);
        assert_eq!(selector.len(), 10);
    }

    #[test]
    fn test_shift_selection_previous() {
        let mut selector =
            ItemSelector::new(VecModel::from_slice(&[MockSelectable::default(); 10]));

        assert_eq!(
            selector.shift_selection_previous(),
            selector.model.row_count() - 1
        );
        assert_eq!(selector.len(), 1);
        assert_eq!(
            selector.model.row_data(selector.model.row_count() - 1),
            Some(MockSelectable { selected: true })
        );
        assert_eq!(selector.selection_start, selector.model.row_count() - 1);
        assert_eq!(selector.selection_end, selector.model.row_count() - 1);

        selector.toggle_selection(2);
        selector.shift_selection(4);
        assert_eq!(selector.shift_selection_previous(), 3);
        assert_eq!(selector.selection_start, 2);
        assert_eq!(selector.selection_end, 3);
        assert_eq!(selector.len(), 2);
        let mut selected_items = BTreeSet::new();
        selected_items.insert(2);
        selected_items.insert(3);
        assert_eq!(selector.selected_items, selected_items);

        assert_eq!(selector.shift_selection_previous(), 2);
        assert_eq!(selector.selection_start, 2);
        assert_eq!(selector.selection_end, 2);
        assert_eq!(selector.len(), 1);

        assert_eq!(selector.shift_selection_previous(), 1);
        assert_eq!(selector.selection_start, 2);
        assert_eq!(selector.selection_end, 1);
        assert_eq!(selector.len(), 2);

        selector.select_all();
        assert_eq!(selector.shift_selection_previous(), 8);
        assert_eq!(selector.selection_start, 0);
        assert_eq!(selector.selection_end, 8);
        assert_eq!(selector.len(), 9);
    }
}
