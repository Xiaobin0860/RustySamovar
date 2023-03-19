extern crate pretty_env_logger;

extern crate tracing_subscriber;

#[macro_use]
extern crate num_derive;

use std::thread;

use rs_nodeconf::NodeConfig;

mod dbmanager;
mod entitymanager;
mod jsonmanager;
mod luamanager;
mod server;
mod utils;

mod subsystems;

use crate::server::GameServer;
use dbmanager::DatabaseManager;
use entitymanager::EntityManager;
use jsonmanager::JsonManager;
use luamanager::LuaManager;
use subsystems::EntitySubsystem;

fn main() {
    //pretty_env_logger::init();

    //let mut rt_main = tokio::runtime::Runtime::new().unwrap();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let nc = NodeConfig::new();
    let mut gs = GameServer::new(&nc);

    gs.run();
}
