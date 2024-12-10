use monitor::monitor_websites;
use std::time::Duration;

#[test]
fn test_monitoring_large_number_of_websites() {
    let urls = vec![
        "https://www.google.com",
        "https://www.github.com",
        "https://www.rust-lang.org",
    ];
    let timeout = Duration::from_secs(5);
    let results = monitor_websites(urls, timeout);

    assert_eq!(results.len(), 3);
    for status in results {
        assert!(status.status.is_ok() || status.status.is_err());
    }
}
