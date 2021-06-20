# GW2timers

A simple way to iterate and collect Guild Wars 2 map meta event times

# Example

Getting the next 10 upcoming events in Auric Basin

```rust
fn main() {
    let next_5_auricbasin_events =
        MapMetaKind::AuricBasin
            .into_iter()
            .take(5)
            .collect::<Vec<EventInstance>>();
}
```

# Usage

Create an iterator by calling `into_iter()` on a `MapMetaKind` and then you can set time you want to start the iterating at then use it like any other iterator. The iterator will never return `None` and will iterate forever always returning the next event in time.

## Create an iterator at a starting time

```rust
let mut tangled_depths_5pm_utc =
    MapMetaKind::TangledDepths
        .into_iter()
        .time(NaiveTime::from_hms(5, 0, 0));
```

## Skip forward through time

```rust
let mut tangled_depths_6pm_utc =
    tangled_depths_5pm_utc
    .fast_forward(Duration::hours(1));
```

## Get the event active at that time in the iterator

```rust
let tangled_depths_event_at_6pm_utc: Option<EventInstance> =
    tangled_depths_6pm_utc
    .now();
```

# License

MIT
