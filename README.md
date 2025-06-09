<div align="center">
<h1>misykat</h1>

<a href="https://github.com/azzamsa/misykat/actions/workflows/ci.yml">
<img src="https://github.com/azzamsa/misykat/actions/workflows/ci.yml/badge.svg">
</a>
<a href="https://crates.io/crates/misykat">
<img src="https://img.shields.io/crates/v/misykat.svg">
</a>
<a href="https://docs.rs/misykat/">
<img src="https://docs.rs/misykat/badge.svg">
</a>

<p></p>

</div>

---

_misykat_ (previously `islam`) is an Islamic library for Rust.
It is a direct port of [PyIslam](https://github.com/abougouffa/pyIslam) with a slight change in the API part.

## Why?

I have always got `panic!` working with [salah](https://github.com/insha/salah).
Previously, I have a good experience with [PyIslam](https://github.com/abougouffa/pyIslam).
In my case, it is very precise and has a simple algorithm. Nowadays, I work a lot with Rust.
So here it is, `misykat` is born!

## Features

- Hijri date
- Prayer times

## Usage

### Getting Prayer Times

```rust
use misykat::salah::{Config, Location, Madhab, Method, PrayerSchedule};

let central_jakarta = Location::new(6.1, 106.49);
let config = Config::new().with(Method::Singapore, Madhab::Shafi);
let prayer_times = PrayerSchedule::new(central_jakarta)?
    .with_config(config)
    .calculate()?;
```

First, you need to specify `Location` with `latitude`, and `longitude` as parameters.
Then choose a calculation method such `Singapore`. Other methods are available [in the docs](https://docs.rs/misykat/latest/misykat/pray/method/enum.Method.html#variants).
There are also `madhab` configurations that you [can choose from](https://docs.rs/misykat/latest/misykat/pray/madhab/enum.Madhab.html#variants).

### Getting Hijri Date

```rust
let date = NaiveDate::from_ymd_opt(2021, 4, 9)
let from_gregorian = HijriDate::from_gregorian(date, 0);
println!(
    "From gregorian: {}-{}-{}",
    from_gregorian.year, from_gregorian.month, from_gregorian.day,
);
```

`from_gregorian` accepts `Date` and `correction value` as parameters.

## More Examples

To learn more, see other [examples](examples/).

## Acknowledgement

The calculation part of this library is a direct port of [PyIslam](https://github.com/abougouffa/pyIslam)
with a slight change in the API part. The API took inspiration from [salah](https://github.com/insha/salah)
