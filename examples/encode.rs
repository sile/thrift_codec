extern crate thrift_codec;
#[macro_use]
extern crate trackable;

use std::fs::File;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use thrift_codec::compact_encode::CompactEncode;
use thrift_codec::message::*;
use thrift_codec::structure::*;
use thrift_codec::collections::*;
use trackable::error::Failure;

fn main() {
    let process_tags = List::structs(vec![
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
        Field::new(1, now.into()), // trace id low
        Field::new(2, 0i64.into()), // trace id high
        Field::new(3, 789i64.into()),
        Field::new(4, 0i64.into()), // no parent
        Field::new(5, "main".to_owned().into()),
        Field::new(7, 1i32.into()), // flags
        Field::new(8, now.into()), // start time
        Field::new(9, 123456i64.into()), // duration
    ]);
    let spans = List::structs(vec![span]);
    let batch = Struct::from((process, spans));
    let body = Struct::from((batch,));
    let message = Message::oneway("emitBatch", 1, body);

    let mut buf = Vec::new();
    track_try_unwrap!(message.encode(&mut buf));

    track_try_unwrap!(
        File::create("_jaeger.dat")
            .and_then(|mut f| f.write_all(&buf))
            .map_err(Failure::from_error)
    );
}

fn str_tag(key: &str, val: &str) -> Struct {
    Struct::new(vec![
        Field::new(1, key.to_owned().into()),
        Field::new(2, (0 as i32).into()),
        Field::new(3, val.to_owned().into()),
    ])
}
