use crossbeam_channel::unbounded;
use crossbeam_queue::ArrayQueue;
use crossbeam_utils::thread::scope;
use futures::channel::oneshot::channel;
use futures::SinkExt;
use itertools::{enumerate, repeat_n, Itertools};
use rand::Rng;
use serde_closure::internal::std::iter::repeat_with;
use std::sync::Arc;
use std::time::Duration;
use std::{io, thread};

#[derive(Debug, Default)]
struct Model {}

fn main() {
    let mut wtr = csv::Writer::from_path("text.csv").unwrap();
    wtr.write_record(&["num"]);
    let (s, r) = unbounded();
    let (model_sender, model_receiver) = unbounded();
    let mut model_queue = repeat_with(|| Model {}).take(1000).collect_vec();
    let done_queue = Arc::new(ArrayQueue::new(10));

    scope(|scope| {
        let runners = (0..10)
            .map(|i| {
                let s = s.clone();
                let receiver = model_receiver.clone();
                let dq = done_queue.clone();
                scope.spawn(move |_| {
                    for m in receiver {
                        s.send(i);
                    }
                    dq.push(());
                })
            })
            .collect_vec();

        scope.spawn(|_| {
            while let Some(m) = model_queue.pop() {
                model_sender.send(m);
            }
            drop(model_sender);
        });

        scope.spawn(|_| {
            let mut c = 0;
            for (u, i) in enumerate(r) {
                wtr.serialize((i));
                wtr.flush().unwrap();
                if u >= 999 {
                    break;
                }
            }
        });
    })
    .unwrap();
}
