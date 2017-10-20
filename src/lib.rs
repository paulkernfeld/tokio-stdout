//! Spawn a new thread that writes to stdout using a Tokio encoder.
//!
//! This is not production-ready:
//! - Items that are flushed to the sink are not guaranteed to be written to stdout (eek!)
//! - Errors are not bubbled up correctly
//! - The user should be able to limit the size of the `BytesMut`
//! - A more thoughful treatment of performance tradeoffs would be nice
//!
//! ```rust
//! extern crate futures;
//! extern crate tokio_fmt_encoder;
//! extern crate tokio_io;
//! extern crate tokio_stdout;
//!
//! use futures::{Future, Stream};
//! use futures::sync::mpsc::SendError;
//! use futures::stream::iter_ok;
//! use tokio_fmt_encoder::DebugEncoder;
//! use tokio_stdout::spawn_encoder_sink_bounded;
//!
//! fn main() {
//!     let encoder: DebugEncoder<usize> = Default::default();
//!
//!     iter_ok::<_, SendError<_>>((1..10).into_iter())
//!         .forward(spawn_encoder_sink_bounded(encoder, 1))
//!         .wait()
//!         .unwrap();
//! }
//! ```
#![deny(warnings)]
#![allow(deprecated)]
extern crate bytes;
extern crate futures;
extern crate tokio_io;

use bytes::BytesMut;
use futures::Stream;
use futures::sync::mpsc::{Sender, UnboundedSender, channel, unbounded};
use std::fmt::Debug;
use std::io::{self, Write};
use std::thread;
use tokio_io::codec::Encoder;


/// Spawn a new thread that encodes each item and writes it to stdout. The channel to send items to
/// the spawned thread is bounded in size.
pub fn spawn_encoder_sink_bounded<I, E, N>(mut encoder: N, buffer_size: usize) -> Sender<I>
where
    I: Send + 'static,
    E: std::convert::From<std::io::Error> + Debug + 'static,
    N: Encoder<Item = I, Error = E> + Send + 'static,
{
    let (channel_sink, channel_stream) = channel::<I>(buffer_size);
    let mut buffer: BytesMut = Default::default();
    thread::spawn(move || {
        let mut stdout = io::stdout();

        for s in channel_stream.wait() {
            encoder.encode(s.unwrap(), &mut buffer).unwrap();
            stdout.write_all(&buffer.take()).unwrap();
        }
    });
    channel_sink
}

/// Spawn a new thread that encodes each item and writes it to stdout. The channel to send items to
/// the spawned thread is unbounded in size.
pub fn spawn_encoder_sink_unbounded<I, E, N>(mut encoder: N) -> UnboundedSender<I>
where
    I: Send + 'static,
    E: std::convert::From<std::io::Error> + Debug + 'static,
    N: Encoder<Item = I, Error = E> + Send + 'static,
{
    let (channel_sink, channel_stream) = unbounded::<I>();
    let mut buffer: BytesMut = Default::default();
    thread::spawn(move || {
        let mut stdout = io::stdout();

        for s in channel_stream.wait() {
            encoder.encode(s.unwrap(), &mut buffer).unwrap();
            stdout.write_all(&buffer.take()).unwrap();
        }
    });
    channel_sink
}
