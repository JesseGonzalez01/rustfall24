use monitor::{monitor_website, WebsiteStatus};
use std::time::Duration;

#[test]
fn test_valid_website() {
    let url = "https://www.google.com";
    let timeout = Duration::from_secs(5);
    let status = monitor_website(url, timeout);

    assert!(status.status.is_ok());
    if let Ok(code) = status.status {
        assert_eq!(code, 200);
    }
}

#[test]
fn test_invalid_website() {
    let url = "https://thisurldoesnotexist.com";
    let timeout = Duration::from_secs(5);
    let status = monitor_website(url, timeout);

    assert!(status.status.is_err());
}
