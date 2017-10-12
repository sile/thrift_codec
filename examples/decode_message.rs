extern crate clap;
extern crate thrift_codec;
#[macro_use]
extern crate trackable;
extern crate serde;
extern crate serdeconv;

use std::fs::File;
use clap::{App, Arg};
use thrift_codec::{BinaryDecode, CompactDecode};
use thrift_codec::message::Message;
use trackable::error::Failure;

fn main() {
    let matches = App::new("decode_message")
        .arg(Arg::with_name("INPUT_FILE").index(1).required(true))
        .arg(Arg::with_name("COMPACT").long("compact"))
        .get_matches();
    let input_file = matches.value_of("INPUT_FILE").unwrap();
    let mut input = track_try_unwrap!(File::open(input_file).map_err(Failure::from_error));
    let message = if matches.is_present("COMPACT") {
        track_try_unwrap!(Message::compact_decode(&mut input))
    } else {
        track_try_unwrap!(Message::binary_decode(&mut input))
    };
    println!(
        "{}",
        track_try_unwrap!(serdeconv::to_json_string_pretty(&message))
    );
}
