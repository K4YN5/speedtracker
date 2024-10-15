use std::io::{Error, ErrorKind};
use std::net::{TcpStream, ToSocketAddrs};
use std::thread;
use std::time::{Duration, Instant};

pub fn ping(times: u32) -> Result<Duration, Error> {
    let target = "8.8.8.8";
    let ports = vec![443, 80, 53]; // Try HTTPS, HTTP, and DNS ports
    let mut results: Duration = Duration::default();

    for i in 0..times {
        let result = ping_tcp_multi(target, &ports);

        let connect_duration = result?;
        results += connect_duration;

        thread::sleep(Duration::from_secs(1));
    }

    Ok(results / times)
}

fn ping_tcp_multi(target: &str, ports: &[u16]) -> Result<Duration, Error> {
    for &port in ports {
        match ping_tcp(&format!("{}:{}", target, port)) {
            Ok(duration) => return Ok(duration),
            Err(_) => continue,
        }
    }
    Err(Error::new(
        ErrorKind::TimedOut,
        "All connection attempts failed",
    ))
}

fn ping_tcp(target: &str) -> Result<Duration, Error> {
    let addrs = target.to_socket_addrs()?;
    for addr in addrs {
        let start = Instant::now();
        match TcpStream::connect(addr) {
            Ok(_) => return Ok(start.elapsed()),
            Err(_) => continue,
        }
    }
    Err(Error::new(ErrorKind::TimedOut, "Connection failed"))
}

