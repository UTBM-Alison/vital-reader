use vital_reader::reader::SessionStats;

#[test]
fn test_stats_new() {
    let stats = SessionStats::new();
    assert_eq!(stats.total_bytes(), 0);
}

#[test]
fn test_stats_default() {
    let stats = SessionStats::default();
    assert_eq!(stats.total_bytes(), 0);
}

#[test]
fn test_add_single_byte() {
    let mut stats = SessionStats::new();
    stats.add_bytes(1);
    assert_eq!(stats.total_bytes(), 1);
}

#[test]
fn test_add_multiple() {
    let mut stats = SessionStats::new();
    stats.add_bytes(100);
    stats.add_bytes(200);
    stats.add_bytes(300);
    assert_eq!(stats.total_bytes(), 600);
}

#[test]
fn test_add_zero() {
    let mut stats = SessionStats::new();
    stats.add_bytes(0);
    assert_eq!(stats.total_bytes(), 0);
}

#[test]
fn test_add_large() {
    let mut stats = SessionStats::new();
    stats.add_bytes(1_000_000);
    assert_eq!(stats.total_bytes(), 1_000_000);
}

#[test]
fn test_elapsed() {
    let stats = SessionStats::new();
    let elapsed = stats.elapsed();
    assert!(elapsed.as_millis() < 1000);
}

#[test]
fn test_average_rate_with_data() {
    let mut stats = SessionStats::new();
    stats.add_bytes(1000);
    std::thread::sleep(std::time::Duration::from_millis(10));
    let rate = stats.average_rate();
    assert!(rate > 0.0);
}

#[test]
fn test_average_rate_no_data() {
    let stats = SessionStats::new();
    assert_eq!(stats.average_rate(), 0.0);
}

#[test]
fn test_reset() {
    let mut stats = SessionStats::new();
    stats.add_bytes(1000);
    stats.reset();
    assert_eq!(stats.total_bytes(), 0);
}

#[test]
fn test_multiple_resets() {
    let mut stats = SessionStats::new();
    stats.add_bytes(100);
    stats.reset();
    stats.add_bytes(200);
    assert_eq!(stats.total_bytes(), 200);
    stats.reset();
    assert_eq!(stats.total_bytes(), 0);
}

#[test]
fn test_accumulation() {
    let mut stats = SessionStats::new();
    for _ in 0..100 {
        stats.add_bytes(10);
    }
    assert_eq!(stats.total_bytes(), 1000);
}

#[test]
fn test_overflow_safety() {
    let mut stats = SessionStats::new();
    stats.add_bytes(usize::MAX);
    assert!(stats.total_bytes() > 0);
}

#[test]
fn test_stats_average_rate_immediate() {
    use vital_reader::reader::SessionStats;
    
    let stats = SessionStats::new();
    // Immediately after creation, rate should be 0 or very high
    let rate = stats.average_rate();
    assert!(rate >= 0.0);
}

#[test]
fn test_stats_elapsed_increases() {
    use vital_reader::reader::SessionStats;
    use std::time::Duration;
    use std::thread;
    
    let stats = SessionStats::new();
    let elapsed1 = stats.elapsed();
    thread::sleep(Duration::from_millis(10));
    let elapsed2 = stats.elapsed();
    assert!(elapsed2 > elapsed1);
}
