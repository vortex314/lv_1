extern crate chrono;

use chrono::{DateTime, Local};
use core::fmt::Formatter;
use std::fmt::{Debug, Error};


pub enum Message {
    Publish{
         topic: String,
         value: String,
         time: DateTime<Local>,
    },
    Subscribe{
         pattern: String,
    },
    Refresh,
    Test { i:i32 }
}


