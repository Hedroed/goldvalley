#![allow(dead_code)]

use chrono::Duration;
use std::path::PathBuf;
use chrono::Utc;
use clap::Parser;

/// Generate beautiful wallpaper based on geographical position and current time
#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Args {
   /// Your coordinate latitude
   #[arg(long, default_value_t=48.864716)]
   lat: f64,

   /// Your coordinate longitude
   #[arg(long, default_value_t=2.349014)]
   lon: f64,

   /// Your coordinate altitude (in meters)
   #[arg(long, default_value_t=100.0)]
   alt: f64,

   /// Output size width in pixels
   #[arg(short, long, default_value_t=1920)]
   width: u32,

   /// Output size height in pixels
   #[arg(short, long, default_value_t=1080)]
   height: u32,

   /// Path for the output image
   #[arg(short, long, value_name = "FILE")]
   output: PathBuf,
}


pub mod render;

fn main() {
    let args = Args::parse();


    let now = Utc::now();
    println!("Now {}", now.date());
    let (sunrise, sunset) = sun_times::sun_times(now.date() - Duration::days(1), args.lat, args.lon, args.alt);
    println!("Sunrise: {}, Sunset: {}", sunrise, sunset);

    let delta_day = sunset - sunrise;

    let angle = if now < sunrise {
        let max_time = now.timestamp() % (3600*24);
        let cur_time = sunrise.timestamp() % (3600*24);

        cur_time * 90 / max_time

    } else if now > sunset {
        let sunset_day = sunset.timestamp() % (3600*24);

        let max_time = 3600*24 - sunset_day;
        let cur_time = now.timestamp() % (3600*24) - sunset_day;

        270 + (cur_time * 90 / max_time)

    } else {
        let a = (now - sunrise).num_seconds() / delta_day.num_seconds();

        90 + (a * 180)
    };

    println!("Angle {}", angle);

    let image = render::render(angle as usize);

    let mut pixmap = tiny_skia::Pixmap::new(args.width, args.height).unwrap();
    resvg::render(&image, usvg::FitTo::Width(args.width), tiny_skia::Transform::default(), pixmap.as_mut()).unwrap();
    pixmap.save_png(args.output).unwrap();

    // for angle in (0..360).step_by(10) {

    //     let image = render::render(angle);

    //     let mut pixmap = tiny_skia::Pixmap::new(800, 450).unwrap();
    //     println!("pixmap");
    //     resvg::render(&image, usvg::FitTo::Width(800), tiny_skia::Transform::default(), pixmap.as_mut()).unwrap();
    //     println!("rendered");
    //     pixmap.save_png(args.output.join(format!("angle_{}.png", angle))).unwrap();
    //     println!("done");
    // }

}