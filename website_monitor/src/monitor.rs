use serde::Deserialize;
use std::time::{Duration, Instant};
use chrono::{DateTime, Utc};
use std::sync::mpsc;
use std::thread;

/// Struct representing the status of a website
#[derive(Debug)]
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: DateTime<Utc>,
}

/// Monitors a single website and returns its status
pub fn monitor_website(url: &str, timeout: Duration) -> WebsiteStatus {
    let client = ureq::AgentBuilder::new()
        .timeout(timeout)
        .build();

    let start_time = Instant::now();
    let response = client.get(url).call();
    let duration = start_time.elapsed();

    let status = match response {
        Ok(resp) => Ok(resp.status()),
        Err(e) => Err(e.to_string()),
    };

    WebsiteStatus {
        url: url.to_string(),
        status,
        response_time: duration,
        timestamp: Utc::now(),
    }
}

/// Monitor multiple websites concurrently
pub fn monitor_websites(urls: Vec<&str>, timeout: Duration) -> Vec<WebsiteStatus> {
    use std::thread;
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    for url in urls {
        let tx = tx.clone();
        let url = url.to_string();

        let handle = thread::spawn(move || {
            let status = monitor_website(&url, timeout);
            tx.send(status).expect("Failed to send status");
        });

        handles.push(handle);
    }

    drop(tx);

    let mut results = vec![];
    for result in rx.iter() {
        println!("{:?}", result);
    }    

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    results
}
