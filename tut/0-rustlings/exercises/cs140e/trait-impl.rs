// FIXME: Make me pass! Diff budget: 25 lines.

// I AM NOT DONE

#[derive(Debug)]
enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16),
}

// What traits does `Duration` need to implement?
impl PartialEq for Duration {
    fn eq(&self, other: &Duration) -> bool {
        let left: u64 = match self {
            &Duration::MilliSeconds(ms) => ms,
            &Duration::Seconds(s) => s as u64 * 1000,
            &Duration::Minutes(min) => min as u64 * 1000 * 60,
        };
        let right: u64 = match self {
            &Duration::MilliSeconds(ms) => ms,
            &Duration::Seconds(s) => s as u64 * 1000,
            &Duration::Minutes(min) => min as u64 * 1000 * 60,
        };
        left == right
    }
}

#[test]
fn traits() {
    assert_eq!(Duration::Seconds(120), Duration::Minutes(2));
    assert_eq!(Duration::Seconds(420), Duration::Minutes(7));
    assert_eq!(Duration::MilliSeconds(420000), Duration::Minutes(7));
    assert_eq!(Duration::MilliSeconds(43000), Duration::Seconds(43));
}
