# GW2timers

A simple way to iterate and collect Guild Wars 2 map meta event times

# Example

Getting the next 10 upcoming events in Auric Basin

```rust
fn main() {
    let next_10_auricbasin_events =
        MetaIter::new(MetaKey::AuricBasin, Utc::now().time())
            .take(10)
            .collect::<Vec<EventInstance>>();
}
```

# Usage

Create an iterator using `MetaIter` and providing it a `MetaKey` and the time you want to start the iterator at.

```rust
let desert_highlands_events = MetaIter::new(MetaKey::DesertHighlands, NaiveTime::from_hms(5, 20, 0));
```

Then use it like any other iterator.

It's important to `take` or otherwise put a limit on the iterator since the iterator will never return `None`.

# License

MIT
