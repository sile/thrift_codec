extern crate clap;
extern crate thrift_codec;
#[macro_use]
extern crate trackable;

use std::fs::File;
use std::time::{SystemTime, UNIX_EPOCH};
use clap::{App, Arg};
use thrift_codec::{BinaryEncode, CompactEncode};
use thrift_codec::message::Message;
use thrift_codec::data::{Struct, Field, List};
use trackable::error::Failure;

fn main() {
    let matches = App::new("encode_message")
        .arg(Arg::with_name("OUTPUT_FILE").index(1).required(true))
        .arg(Arg::with_name("COMPACT").long("compact"))
        .get_matches();
    let message = message();
    let mut output = track_try_unwrap!(
        File::create(matches.value_of("OUTPUT_FILE").unwrap())
            .map_err(Failure::from_error)
    );
    if matches.is_present("COMPACT") {
        track_try_unwrap!(message.compact_encode(&mut output));
    } else {
        track_try_unwrap!(message.binary_encode(&mut output));
    };
}

// see: https://github.com/uber/jaeger-idl/blob/master/thrift/jaeger.thrift
fn message() -> Message {
    let process_tags = List::from(vec![
        str_tag("jaeger.version", "Go-2.9.0"),
        str_tag("hostname", "ubu"),
        str_tag("ip", "10.0.2.15"),
    ]);
    let process = Struct::from(("foo_service".to_owned(), process_tags));

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64 * 1_000_000;
    let span = Struct::new(vec![
            Field::new(1, now), // trace id low
            Field::new(2, 0i64), // trace id high
            Field::new(3, 789i64),
            Field::new(4, 0i64), // no parent
            Field::new(5, "main".to_owned()),
            Field::new(7, 1i32), // flags
            Field::new(8, now), // start time
            Field::new(9, 123_456i64), // duration
        ]);
    let spans = List::from(vec![span]);
    let batch = Struct::from((process, spans));
    Message::oneway("emitBatch", 1, Struct::from((batch,)))
}

fn str_tag(key: &str, val: &str) -> Struct {
    Struct::from((key.to_owned(), 0, val.to_owned()))
}
