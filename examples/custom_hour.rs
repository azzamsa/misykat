use misykat::jiff;
use misykat::salah::{Config, Location, Madhab, Method, PrayerSchedule};

fn custom_hour() -> Result<(), misykat::Error> {
    let central_jakarta = Location::new(6.10, 106.49);
    let config = Config::new().with(Method::Singapore, Madhab::Shafi);
    let now = jiff::civil::date(2023, 8, 30)
        // Current prayer is ishaa (before midnight)
        // .at(20, 28, 00, 0);
        // Current prayer is ishaa (after midnight/early moring, before fajr)
        .at(4, 28, 00, 0);
    let prayer_times = PrayerSchedule::new(central_jakarta)
        .at(now)
        .with_config(config)
        .calculate()?;

    println!("Current time: {}\n", now);

    println!("All Prayers");
    println!("fajr   : {}", prayer_times.fajr);
    println!("sherook: {}", prayer_times.sherook);
    println!("dohr   : {}", prayer_times.dohr);
    println!("asr    : {}", prayer_times.asr);
    println!("maghreb: {}", prayer_times.maghreb);
    println!("ishaa  : {}", prayer_times.ishaa);
    println!("fajr tomorrow: {}", prayer_times.fajr_tomorrow);

    let current_prayer = prayer_times.current();
    let (hour, minute) = prayer_times.time_remaining();
    println!("\nCurrent Prayer");
    println!(
        "{}: ({:02}:{:02} left)",
        current_prayer.name(),
        hour,
        minute
    );

    println!("\nNext Prayer");
    let next_prayer = prayer_times.next();
    let time = prayer_times.time(next_prayer);
    let time = time.strftime("%H:%M").to_string();
    println!("{}: ({})", next_prayer.name(), time);

    Ok(())
}

fn main() {
    if let Err(err) = custom_hour() {
        eprintln!("Error: {:?}", err);
    }
}
