use crate::config::AppConfig;
use poise::Command;
use std::{error::Error, marker::Send};

pub mod clear;
pub mod ping;
pub mod nasa;

pub fn commands() -> Vec<Command<AppConfig, Box<dyn Error + Send + Sync + 'static>>> {
    vec![ping::ping(), clear::clear(), nasa::apod()]
}
