#[macro_use]
extern crate serde_derive;

extern crate log;
extern crate clap;
extern crate crust;
extern crate maidsafe_utilities;
extern crate rand;
extern crate serde_json;

use std::collections::{BTreeMap, HashMap};
use clap::{App, AppSettings, Arg, SubCommand};
use crust::{Config, ConnectionInfoResult, Uid};
use rand::{Rand, Rng};
use std::cmp;
use std::io;
use std::str::FromStr;
use std::sync::mpsc::{channel, RecvTimeoutError, Sender};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct UniqueId([u8; 20]);
impl Uid for UniqueId {}
impl Rand for UniqueId {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let mut inner = [0; 20];
        rng.fill_bytes(&mut inner);
        UniqueId(inner)
    }
}

type PrivConnectionInfo = crust::PrivConnectionInfo<UniqueId>;
type Service = crust::Service<UniqueId>;

fn generate_random_vec_u8(size: usize) -> Vec<u8> {
    let vec: Vec<u8> = Vec::with_capacity(size);
    vec.iter().map(|_| rand::random::<u8>()).collect()
}

struct Network {
    nodes: HashMap<usize, UniqueId>,
    our_connections: BTreeMap<u32, PrivConnectionInfo>,
    received_msgs: u32,
    received_bytes: usize,
    peer_index: usize,
    connection_info_index: u32,
}

// WIP: Moving example code. It will need a big refactor to be readable

fn main() {
    println!("Hello, world!");
}
