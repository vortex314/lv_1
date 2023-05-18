#![allow(unused_imports)]
#![allow(unused_variables)]
mod redis_receiver;
mod vc;

pub use vc::message::{Message};
pub use redis_receiver::do_redis;
use vc::view::do_view;

extern crate chrono;
extern crate crossbeam;
extern crate log;
extern crate rand;
extern crate redis;

use log::info;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

use pub_sub::PubSub;

use std::thread;
use std::time::{Duration, SystemTime};


fn main() {
    set_logging();
    info!("Starting up");

    let channel = PubSub::new();
    let view_channel = channel.clone();

    thread::spawn(move || {
        info!("Starting view ");
        do_view(view_channel).unwrap();
        info!("Started view");
    });
    let rc = do_redis(channel);
}

fn set_logging() {
    let console = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S.%3f)} {l} {t} - {m}\n",
        )))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(console)))
        .build(Root::builder().appender("console").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
}
