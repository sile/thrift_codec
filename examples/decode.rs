extern crate clap;
extern crate thrift_codec;
#[macro_use]
extern crate trackable;

use std::fs::File;
use clap::{App, Arg};
use thrift_codec::decode::CompactDecode;
use trackable::error::Failure;

fn main() {
    let matches = App::new("decode")
        .arg(Arg::with_name("INPUT_FILE").index(1).required(true))
        .get_matches();
    let input_file = matches.value_of("INPUT_FILE").unwrap();
    let mut input = track_try_unwrap!(File::open(input_file).map_err(Failure::from_error));
    let m = track_try_unwrap!(thrift_codec::message::Message::decode(&mut input));
    println!("{:?}", m);
}
