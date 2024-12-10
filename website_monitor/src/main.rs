mod monitor;

use std::time::Duration;
use monitor::{monitor_websites, WebsiteStatus};

fn main() {
    let websites = vec![
        "https://www.google.com",
        "https://www.github.com",
        "https://www.rust-lang.org",
        "https://thisurldoesnotexist.com",
        "http://example1234.net/",
        "https://www.youtube.com/",
        "https://www.amazon.com/",
        "https://www.tiktok.com/",
        "https://www.imdb.com/",
        "https://www.nytimes.com/",
        "https://x.com/",
        "https://www.dailymotion.com/us",
        "https://www.yahoo.com/?guccounter=1",
    ];

    let timeout = Duration::from_secs(5);
    let results = monitor_websites(websites, timeout);

    for status in results.iter() {
        println!("{:?}", status);
    }    
}

fn print_status(status: &WebsiteStatus) {
    match &status.status {
        Ok(code) => println!(
            "URL: {}, Status: {}, Response Time: {:?}, Timestamp: {}",
            status.url, code, status.response_time, status.timestamp
        ),
        Err(err) => println!(
            "URL: {}, Error: {}, Response Time: {:?}, Timestamp: {}",
            status.url, err, status.response_time, status.timestamp
        ),
    }
}
