use monitor::monitor_website;
use std::time::Duration;

#[test]
fn test_timeout_error() {
    let url = "https://www.google.com";
    let timeout = Duration::from_millis(1); // Unreasonably short timeout
    let status = monitor_website(url, timeout);

    assert!(status.status.is_err());
}
