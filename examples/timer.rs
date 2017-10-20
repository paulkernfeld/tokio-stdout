//! This example prints out to stdout on a timer
extern crate futures;
extern crate tokio_fmt_encoder;
extern crate tokio_io;
extern crate tokio_stdout;
extern crate tokio_timer;

use futures::{Future, Sink, Stream};
use futures::sync::mpsc::SendError;
use std::time::Duration;
use tokio_fmt_encoder::DebugEncoder;
use tokio_stdout::spawn_encoder_sink_unbounded;
use tokio_timer::{Timer, TimerError};

#[derive(Debug)]
enum Error {
    Timer(TimerError),
    Stdout(SendError<String>),
}

fn main() {
    let encoder: DebugEncoder<String> = Default::default();

    Timer::default()
        .interval(Duration::from_secs(1))
        .map(|()| String::from("hello"))
        .map_err(Error::Timer)
        .forward(spawn_encoder_sink_unbounded(encoder).sink_map_err(Error::Stdout))
        .wait()
        .unwrap();
}
