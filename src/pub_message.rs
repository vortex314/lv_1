extern crate chrono;

use chrono::{DateTime, Local};
use core::fmt::Formatter;
use std::fmt::{Debug, Error};


pub enum Message {
    Publish(PublishMessage),
    Subscribe(SubscribeMessage),
    Refresh,
}
pub struct PublishMessage {
    pub topic: String,
    pub value: String,
    pub time: DateTime<Local>,
}

pub struct SubscribeMessage {
    pub pattern: String,
}

pub struct CommandMessage {
    pub command: String,
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

