use chrono::{DateTime, Duration, Utc};


pub type Angle = i64;


pub fn sun_angle(date: DateTime<Utc>, latitude: f64, longitude: f64, elevation: f64) -> Angle {

    let (sunrise, sunset) = sun_times::sun_times(date.date() - Duration::days(1), latitude, longitude, elevation);
    println!("Sunrise: {}, Sunset: {}", sunrise, sunset);

    let angle = if date < sunrise {
        println!("case 1");
        get_angle(date, sunset - Duration::days(1), sunrise) - 90

    } else if date > sunset {
        println!("case 2");
        (270 + get_angle(date, sunset, sunrise + Duration::days(1))) % 360

    } else {
        println!("case 3");
        90 + get_angle(date, sunrise, sunset)
    };

    println!("Angle {}", angle);

    angle
}


fn get_angle(now: DateTime<Utc>, min_date: DateTime<Utc>, max_date: DateTime<Utc>) -> Angle {

    let delta_day: f64 = (max_date - min_date).num_seconds() as f64;

    let offset: f64 = (now - min_date).num_seconds() as f64;

    let a = offset / delta_day;

    (a * 180.0).max(0.0).min(180.0) as i64
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use chrono::TimeZone;

    const NYC_LAT: f64 = 40.7128;
    const NYC_LON: f64 = 74.0060;

    const PARIS_LAT: f64 = 48.864716;
    const PARIS_LON: f64 = 2.349014;


    #[test]
    fn test_get_angle() {

        let d = Utc.ymd(2022, 1, 1).and_hms(12, 0, 0);
        let a = super::sun_angle(d, PARIS_LAT, PARIS_LON, 1.0);
        assert_eq!(a, 181);

        let d = Utc.ymd(2022, 1, 1).and_hms(0, 0, 1);
        let a = super::sun_angle(d, PARIS_LAT, PARIS_LON, 1.0);
        assert_eq!(a, 1);

        let d = Utc.ymd(2022, 1, 1).and_hms(23, 59, 59);
        let a = super::sun_angle(d, PARIS_LAT, PARIS_LON, 1.0);
        assert_eq!(a, 1);

        let d = Utc.ymd(2022, 1, 1).and_hms(23, 0, 0);
        let a = super::sun_angle(d, PARIS_LAT, PARIS_LON, 1.0);
        assert_eq!(a, 349);

        let d = Utc.ymd(2022, 11, 1).and_hms(13, 0, 0);
        let a = super::sun_angle(d, PARIS_LAT, PARIS_LON, 1.0);
        assert_eq!(a, 205);

        let d = Utc.ymd(2022, 11, 1).and_hms(23, 0, 0);
        let a = super::sun_angle(d, PARIS_LAT, PARIS_LON, 1.0);
        assert_eq!(a, 352);

        let d = Utc.ymd(2022, 8, 1).and_hms(13, 0, 0);
        let a = super::sun_angle(d, PARIS_LAT, PARIS_LON, 1.0);
        assert_eq!(a, 192);

        let d = Utc.ymd(2022, 8, 1).and_hms(2, 0, 0);
        let a = super::sun_angle(d, PARIS_LAT, PARIS_LON, 1.0);
        assert_eq!(a, 41);

        let d = Utc.ymd(2022, 8, 1).and_hms(13, 0, 0);
        let a = super::sun_angle(d, NYC_LAT, NYC_LON, 1.0);
        assert_eq!(a, 252);

        let d = Utc.ymd(2022, 8, 1).and_hms(2, 0, 0);
        let a = super::sun_angle(d, NYC_LAT, NYC_LON, 1.0);
        assert_eq!(a, 114);

    }
}