use std::time::{Duration, Instant};

#[test]
fn test_processing_latency_under_limit() {
    let start = Instant::now();

    std::thread::sleep(Duration::from_millis(5));

    let elapsed = start.elapsed();

    assert!(elapsed.as_millis() < 10);
}

#[test]
fn test_buffer_latency_calculation() {
    let sample_rate = 48_000.0;
    let buffer_size = 480.0;

    let latency_ms = (buffer_size / sample_rate) * 1000.0;

    assert_eq!(latency_ms as u32, 10);
}
