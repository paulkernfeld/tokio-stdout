# tokio-stdout

Spawn a new thread that writes to stdout using a Tokio encoder.

This is not production-ready:
- Items that are flushed to the sink are not guaranteed to be written to stdout (eek!)
- Errors are not bubbled up correctly
- The user should be able to limit the size of the `BytesMut`
- A more thoughful treatment of performance tradeoffs would be nice

```rust
extern crate futures;
extern crate tokio_fmt_encoder;
extern crate tokio_io;
extern crate tokio_stdout;

use futures::{Future, Stream};
use futures::sync::mpsc::SendError;
use futures::stream::iter_ok;
use tokio_fmt_encoder::DebugEncoder;
use tokio_stdout::spawn_encoder_sink_bounded;

fn main() {
    let encoder: DebugEncoder<usize> = Default::default();

    iter_ok::<_, SendError<_>>((1..10).into_iter())
        .forward(spawn_encoder_sink_bounded(encoder, 1))
        .wait()
        .unwrap();
}
```

License: MIT/Apache-2.0
