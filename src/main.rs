use std::time::Instant;

use ping::ping;
use record::Record;

mod ping;
mod record;

fn main() {
    let start = Instant::now();
    let record = Record::new();
    println!("{:?}", record);
    println!("{:?}", start.elapsed());
}
