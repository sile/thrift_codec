extern crate clap;
extern crate thrift_codec;
#[macro_use]
extern crate trackable;
extern crate serde;
extern crate serdeconv;

use clap::Parser;
use std::fs::File;
use thrift_codec::message::Message;
use thrift_codec::{BinaryDecode, CompactDecode};
use trackable::error::Failure;

#[derive(Debug, Parser)]
#[clap(name = "decode_message")]
struct Args {
    input_file: std::path::PathBuf,

    #[clap(long)]
    compact: bool,
}

fn main() {
    let args = Args::parse();
    let mut input = track_try_unwrap!(File::open(args.input_file).map_err(Failure::from_error));
    let message = if args.compact {
        track_try_unwrap!(Message::compact_decode(&mut input))
    } else {
        track_try_unwrap!(Message::binary_decode(&mut input))
    };
    println!(
        "{}",
        track_try_unwrap!(serdeconv::to_json_string_pretty(&message))
    );
}
