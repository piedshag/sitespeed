use reqwest;
use clap::Parser;
use std::time::{Duration, Instant};
use std::thread::sleep;

/// Time requests for a site
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Site to time
    #[clap(short, long)]
    site: String,

    /// Delay between requests
    #[clap(short, long, default_value_t = 1)]
    delay: u8,

    /// Iterations
    #[clap(short, long, default_value_t = 10)]
    iterations: u8,
}

fn time_request(site: &str) -> Result<Duration, Box<dyn std::error::Error>> {
    let now = Instant::now();
    let body = reqwest::blocking::get(site)?.text()?;
    let time = now.elapsed();
    Ok(time)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut times = Vec::new();

    for i in  1..10 {    
        let time = time_request(&args.site)?;
        println!("Attempt {} {:?} μs", i, time.as_micros());
        times.push(time.as_micros().clone() as u32);
        sleep(Duration::new(args.delay.into(), 0));    
    }

    let average = times.iter().sum::<u32>() as f32 / times.len() as f32;
    println!("average: {} seconds", average / 1000000.0);

    Ok(())
}
