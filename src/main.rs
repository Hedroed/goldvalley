#![allow(clippy::redundant_static_lifetimes)]

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
   #[arg(long, default_value_t=1920)]
   width: u32,

   /// Output size height in pixels
   #[arg(long, default_value_t=1080)]
   height: u32,

   /// Path for the output image
   #[arg(short, long, value_name = "FILE")]
   output: PathBuf,

   /// Force sun angle (overwrite lat, lon, alt)
   #[arg(long)]
   angle: Option<usize>,
}


pub mod render;
pub mod angle;


fn main() {
    let args = Args::parse();

    let angle = if let Some(angle) = args.angle {
        angle
    } else {
        let now = Utc::now();
        angle::sun_angle(now, args.lat, args.lon, args.alt).try_into().unwrap()
    };

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