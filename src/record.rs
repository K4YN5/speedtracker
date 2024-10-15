use std::{
    process::Command,
    time::{Instant, SystemTime},
};

use crate::ping::ping;

// Address
#[derive(Debug)]
pub struct Record {
    /// Uniqute ID of the record sorted by time
    id: u64,
    /// Time when the record was created, date time in unix timestamp
    created_at: u64,
    /// Ping in milliseconds
    ping: f32,
    /// Download speed in bits per second
    download_speed: f32,
    /// Upload speed in bits per second
    upload_speed: f32,
}

impl Record {
    pub fn new() -> Self {
        // From the db get the last id
        let id = 0;

        let ping = Self::get_ping();

        let download_speed = Self::download_speed();
        let upload_speed = Self::upload_speed();

        let record = Record {
            id,
            created_at: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ping,
            download_speed,
            upload_speed,
        };

        // Save in db

        record
    }

    /// Return the ping in milliseconds
    fn get_ping() -> f32 {
        let ping = ping(5).unwrap();
        ping.as_micros() as f32 / 1000.0
    }

    /// Download a 500 MB file, should take 10 seconds.
    fn download_speed() -> f32 {
        let start = Instant::now();
        {
            Command::new("wget")
                .args(["http://speedtest.tele2.net/100MB.zip"])
                .output()
                .unwrap();
        }
        let elapsed_micros = (start.elapsed().as_micros() - 500_000) as f64;
        println!("Elapsed: {}", elapsed_micros);
        let bits = (100.0 * 8.0 * 1_000_000.0) as f64; // 100 MB in bits
        (bits / elapsed_micros) as f32 // This gives the result in Mbps
    }

    /// Upload a 500 MB file, should take 10 seconds.
    fn upload_speed() -> f32 {
        let start = Instant::now();
        {
            let output = Command::new("curl")
                .args([
                    "-T",
                    "100MB.zip",
                    "http://speedtest.tele2.net/upload.php",
                    "-o",
                    "/dev/null",
                ])
                .output()
                .unwrap()
                .status;
            println!("{}", output);

            // Clean up
            Command::new("rm").args(["100MB.zip"]).output().unwrap();
        }

        let elapsed_micros = (start.elapsed().as_micros() - 500_000) as f64; // Substract one second for the
        println!("Elapsed: {}", elapsed_micros);
        let bits = 100.0 * 8.0 * 1_000_000.0; // 100 MB in bits
        (bits / elapsed_micros) as f32 // This gives the result in Mbps
    }
}
