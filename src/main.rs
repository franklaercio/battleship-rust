use std::io::Result;

use log::{info, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};

mod game;
mod network;
mod ui;
mod utils;
mod player;

fn main() -> Result<()> {
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();

    info!("Starting server...");

    network::start_server("0.0.0.0:5560")
}
