use chrono::Utc;
use chrono::TimeZone;
use chrono::Date;
use chrono::Duration;


const LAT: f64 = 40.7128;
const LON: f64 = 74.0060;
// const LAT: f64 = 48.864716;
// const LON: f64 = 2.349014;


fn print_sun(d: Date<Utc>) {
    println!("Date {}", d);
    let (sunrise, sunset) = sun_times::sun_times(d - Duration::days(1), LAT, LON, 1.0);
    println!("Sunrise: {}, Sunset: {}", sunrise, sunset);
}

fn main() {

    print_sun(Utc.ymd(2022, 1, 1));
    print_sun(Utc.ymd(2022, 6, 11));
    print_sun(Utc.ymd(2022, 11, 5));
    print_sun(Utc.ymd(2022, 12, 31));

}