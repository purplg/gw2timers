# GW2timers

A simple way to iterate and collect Guild Wars 2 map meta event times

## Example

Getting the next 5 upcoming events in Auric Basin

```rust
fn main() {
    let next_5_auricbasin_events =
        MapMetaKind::AuricBasin
            .into_iter()
            .take(5)
            .collect::<Vec<EventInstance>>();
}
```

## Usage

Create an iterator by calling `into_iter()` on a `MapMetaKind` and then you can set time you want to start the iterating at then use it like any other iterator. The iterator will never return `None` and will iterate forever always returning the next event in time.

### Create an iterator starting at a time

```rust
let mut tangled_depths_5am_utc =
    MapMetaKind::TangledDepths
        .into_iter()
        .time(NaiveTime::from_hms(5, 0, 0));
```

### Skip forward through time

```rust
let mut tangled_depths_6am_utc =
    tangled_depths_5am_utc
    .fast_forward(Duration::hours(1));
```

### Get the event active at that time in the iterator

```rust
let tangled_depths_event_at_6am_utc: Option<EventInstance> =
    tangled_depths_6am_utc
    .now();
```

# License

MIT
