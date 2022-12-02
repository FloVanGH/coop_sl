// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

/// Represents a result from the server.
pub type ServerResult<T> = Result<T, ServerError>;

#[derive(Clone, Debug)]
pub enum ServerError {
    CannotOpenClient(String),
    CannotSendEvent(String),
    CannotClose(String),
}

impl From<Result<String, ServerError>> for ServerError {
    fn from(value: Result<String, ServerError>) -> Self {
        value.expect_err("Cannot convert an Ok(T) to error")
    }
}
