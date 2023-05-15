#[allow(unused_labels)]
extern crate redis;

use crate::Message;
use chrono::{DateTime, Local, Utc};
use crossbeam_channel::{bounded, unbounded, Receiver, Sender};
use log::info;
use redis::{Cmd, Commands, ConnectionLike, RedisError, RedisResult};
use std::thread;
use std::time::Duration;

pub fn do_redis(sender: Sender<Message>) -> redis::RedisResult<()> {
    info!("Starting redis receiver... ");
    

    'connection : loop {
        let client = redis::Client::open("redis://192.168.0.102/")?;
        let mut con = client.get_connection()?;
        con.set_read_timeout(Some(Duration::from_millis(100)))?;
        let mut pubsub = con.as_pubsub();
    
        pubsub.psubscribe("src/*")?;
        pubsub.psubscribe("dst/*")?;
        info!("Subscribed to src/*");
        'receive : loop {
            match pubsub.get_message() {
                Ok(msg) => {
                    sender
                        .send(Message::Publish {
                            topic: msg.get_channel_name().to_string(),
                            value: msg.get_payload().unwrap(),
                            time: Local::now(),
                        })
                        .unwrap();
                }
                Err(e) => {
                    if e.is_connection_dropped() {
                        info!("Error: {:?}", e);
                        break 'receive
                    }
                }
            }
        }
    }

    
}
