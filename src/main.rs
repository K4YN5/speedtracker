use std::{
    process::Command,
    time::{Instant, SystemTime},
};

fn main() {
    let start = Instant::now();

    let record = Record::new();

    println!("{:?}", record);

    println!("Elapsed time: {:?}", start.elapsed());
}

// 20672,apfutura,Barcelona,2024-10-15T14:11:31.539921Z,46.92509841918945,61.530,200306069.3653173,183026594.73183116,,79.156.168.226
// Server ID,Server Name,Location,Distance,Ping,Download,Upload,Result URL,IP
// Address
#[derive(Debug)]
struct Record {
    /// Uniqute ID of the record sorted by time
    id: u64,
    /// Time when the record was created, date time in unix timestamp
    created_at: u64,
    /// Server name
    server: String,
    /// Server location
    location: String,
    /// Ping in milliseconds
    ping: f32,
    /// Download speed in bits per second
    download_speed: f32,
    /// Upload speed in bits per second
    upload_speed: f32,
}

impl Record {
    pub fn new() -> Self {
        let fields = Self::record();

        let record = Record {
            id: 0,
            created_at: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            server: fields[0].clone(),
            location: fields[1].clone(),
            ping: fields[2].parse().unwrap(),
            download_speed: fields[3].parse().unwrap(),
            upload_speed: fields[4].parse().unwrap(),
        };

        // Save in db

        record
    }

    fn record() -> [String; 5] {
        let csv_output = String::from_utf8(
            Command::new("speedtest-rs")
                .arg("--csv")
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap();

        let mut splitted: Vec<String> = csv_output.split(",").map(|s| s.to_string()).collect();

        [
            splitted.remove(1),
            splitted.remove(1),
            splitted.remove(3),
            splitted.remove(3),
            splitted.remove(3),
        ]
    }
}
