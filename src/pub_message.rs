extern crate chrono;

use chrono::{DateTime, Local};
use core::fmt::Formatter;
use std::fmt::{Debug, Error};


pub struct PublishMessage {
    pub topic: String,
    pub value: String,
    pub time: DateTime<Local>,
}

 impl Debug for PublishMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "PublishMessage {{ topic: {}, value: {}, time: {} }}",
            self.topic, self.value, self.time
        )
    }
}

