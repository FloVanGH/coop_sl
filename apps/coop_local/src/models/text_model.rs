// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::fmt::Display;

#[derive(Clone, Default)]
pub struct TextModel {
    text: String,
    text_update: String,
}

impl TextModel {
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();

        Self {
            text: text.clone(),
            text_update: text,
        }
    }

    pub fn has_changes(&self) -> bool {
        !self.text.eq(&self.text_update)
    }

    pub fn text_update(&self) -> &str {
        &self.text_update
    }

    pub fn set_text_update(&mut self, text_update: impl Into<String>) {
        self.text_update = text_update.into();
    }

    pub fn update_text(&mut self) {
        self.text = self.text_update.clone();
    }
}

impl Display for TextModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_has_changes() {
        let mut text = TextModel::new("Hello");
        assert!(!text.has_changes());

        text.set_text_update("Hello World");
        assert!(text.has_changes());
        assert_eq!(text.to_string(), String::from("Hello"));

        text.update_text();
        assert!(!text.has_changes());
        assert_eq!(text.to_string(), String::from("Hello World"));
    }
}
