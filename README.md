# GW2timers

A simple way to iterate and collect Guild Wars 2 map meta event times

# Usage

Getting the next 10 upcoming events in Auric Basin

```rust
fn main() {
    let next_10_auricbasic_event =
        MetaIter::new(MetaKey::AuricBasin, Utc::now().time())
            .take(10)
            .collect::<Vec<EventInstance>>();
}
```

It's important to `take` or otherwise put a limit on the iterator since the iterator will never return `None`.

# License

MIT
