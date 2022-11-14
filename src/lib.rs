//! This library offers the possibility to access the Telegram Bot API from Rust.</br>
//! It is designed to change as little as possible from the well documented <a href="https://core.telegram.org/bots/api">Telegram Bot API</a>.</br>
//! To learn more about the individual types and methods, please visit <a href="https://core.telegram.org/bots/api">Telegram Bot API</a>.</br>
//! The issue tracker is located on <a href="https://github.com/jrmbchtl/telegram-bot-rs">Github</a>
pub mod methods;
pub mod objects;
#[macro_use]
pub mod api_macros;
