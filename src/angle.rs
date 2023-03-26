use chrono::{Duration, NaiveDateTime};

use crate::sunrise;

pub type Angle = i64;

pub fn sun_angle(date: NaiveDateTime, latitude: f64, longitude: f64, elevation: f64) -> Angle {
    let (sunrise, sunset) = sunrise::sun_times(date.date(), latitude, longitude, elevation);
    println!("Sunrise: {}, Sunset: {}", sunrise, sunset);

    let angle = if date < sunrise {
        println!("morning");
        get_angle(date, sunset - Duration::days(1), sunrise) - 90
    } else if date > sunset {
        println!("night");
        (270 + get_angle(date, sunset, sunrise + Duration::days(1))) % 360
    } else {
        println!("day");
        90 + get_angle(date, sunrise, sunset)
    };

    println!("Angle {}", angle);

    angle
}

fn get_angle(now: NaiveDateTime, min_date: NaiveDateTime, max_date: NaiveDateTime) -> Angle {
    let delta_day: f64 = (max_date - min_date).num_seconds() as f64;

    let offset: f64 = (now - min_date).num_seconds() as f64;

    let a = offset / delta_day;

    (a * 180.0).max(0.0).min(180.0) as i64
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use chrono::Utc;

    const NYC_LAT: f64 = 40.7128;
    const NYC_LON: f64 = 74.0060;

    const PARIS_LAT: f64 = 48.864716;
    const PARIS_LON: f64 = 2.349014;

    #[test]
    fn test_get_angle() {
        let d = Utc.with_ymd_and_hms(2022, 1, 1, 12, 0, 0).unwrap();
        let a = super::sun_angle(d.naive_utc(), PARIS_LAT, PARIS_LON, 1.0);
        assert_eq!(a, 180);

        let d = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 1).unwrap();
        let a = super::sun_angle(d.naive_utc(), PARIS_LAT, PARIS_LON, 1.0);
        assert_eq!(a, 0);

        let d = Utc.with_ymd_and_hms(2022, 1, 1, 23, 59, 59).unwrap();
        let a = super::sun_angle(d.naive_utc(), PARIS_LAT, PARIS_LON, 1.0);
        assert_eq!(a, 359);

        let d = Utc.with_ymd_and_hms(2022, 1, 1, 23, 0, 0).unwrap();
        let a = super::sun_angle(d.naive_utc(), PARIS_LAT, PARIS_LON, 1.0);
        assert_eq!(a, 348);

        let d = Utc.with_ymd_and_hms(2022, 11, 1, 13, 0, 0).unwrap();
        let a = super::sun_angle(d.naive_utc(), PARIS_LAT, PARIS_LON, 1.0);
        assert_eq!(a, 198);

        let d = Utc.with_ymd_and_hms(2022, 11, 1, 23, 0, 0).unwrap();
        let a = super::sun_angle(d.naive_utc(), PARIS_LAT, PARIS_LON, 1.0);
        assert_eq!(a, 347);

        let d = Utc.with_ymd_and_hms(2022, 8, 1, 13, 0, 0).unwrap();
        let a = super::sun_angle(d.naive_utc(), PARIS_LAT, PARIS_LON, 1.0);
        assert_eq!(a, 191);

        let d = Utc.with_ymd_and_hms(2022, 8, 1, 2, 0, 0).unwrap();
        let a = super::sun_angle(d.naive_utc(), PARIS_LAT, PARIS_LON, 1.0);
        assert_eq!(a, 40);

        let d = Utc.with_ymd_and_hms(2022, 8, 1, 2, 0, 0).unwrap();
        let a = super::sun_angle(d.naive_utc(), NYC_LAT, NYC_LON, 1.0);
        assert_eq!(a, 37);
    }
}
