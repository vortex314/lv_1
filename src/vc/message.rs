extern crate chrono;

use chrono::{DateTime, Local};
use core::fmt::Formatter;
use std::{
    fmt::{Debug, Error},
    ops::ShrAssign,
};

#[derive(Debug,Clone)]
pub enum Message {
    Publish {
        topic: String,
        value: String,
        time: DateTime<Local>,
    },
    Subscribe {
        pattern: String,
    },
    Refresh,
    Clear,
}

pub trait Sink<T> {
    fn on(&mut self, t: & T) -> ();
}

trait Source<T> {
    fn subscribe(&mut self, sink: Box<dyn Sink<T>>) -> ();
    fn emit(&mut self, t: T) -> ();
}


