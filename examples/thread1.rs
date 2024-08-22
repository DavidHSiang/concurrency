use anyhow::{anyhow, Result};
use std::{sync::mpsc, thread, time::Duration};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    id: usize,
    data: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx);

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("Received: {:?}", msg);
        }
        println!("Consumer stopping");
        rand::random::<u64>() % 10
    });
    let secret = consumer
        .join()
        .map_err(|e| anyhow!("Consumer thread panicked: {:?}", e))?;
    println!("Secret: {}", secret);
    Ok(())
}

fn producer(i: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let rand_data = rand::random::<usize>();
        tx.send(Msg::new(i, rand_data))?;
        let rand_sleep = rand::random::<u64>() % 10;
        thread::sleep(Duration::from_secs(rand_sleep));

        // randomly stop the producer
        if rand::random::<u64>() % 10 == 0 {
            println!("Producer {} stopping", i);
            break Ok(());
        }
    }
}

impl Msg {
    fn new(id: usize, data: usize) -> Self {
        Self { id, data }
    }
}
