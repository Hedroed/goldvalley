#![allow(clippy::redundant_static_lifetimes)]

use chrono::{DateTime, Utc};
use clap::Parser;
use std::path::PathBuf;

/// Generate beautiful wallpaper based on geographical position and current time
#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Args {
    /// Your coordinate latitude
    #[arg(long, default_value_t = 48.864716)]
    lat: f64,

    /// Your coordinate longitude
    #[arg(long, default_value_t = 2.349014)]
    lon: f64,

    /// Your coordinate altitude (in meters)
    #[arg(long, default_value_t = 100.0)]
    alt: f64,

    /// Output size width in pixels
    #[arg(long, default_value_t = 1920)]
    width: u32,

    /// Output size height in pixels
    #[arg(long, default_value_t = 1080)]
    height: u32,

    /// Path for the output image
    #[arg(short, long, value_name = "FILE")]
    output: PathBuf,

    /// Force sun angle (overwrite lat, lon, alt)
    #[arg(long)]
    angle: Option<usize>,

    /// Force datetime, format ISO8601 or RFC3339
    #[arg(long)]
    #[arg(value_parser = parse_hour)]
    datetime: Option<DateTime<Utc>>,
}

pub mod angle;
pub mod render;
pub mod sunrise;

fn parse_hour(arg: &str) -> Result<DateTime<Utc>, chrono::format::ParseError> {
    let datetime = DateTime::parse_from_rfc3339(arg)?;
    Ok(datetime.into())
}

fn main() {
    let args = Args::parse();

    let angle = if let Some(angle) = args.angle {
        angle
    } else {
        let datetime = args.datetime.unwrap_or_else(Utc::now);
        println!("Datetime: {}", datetime);

        angle::sun_angle(datetime.naive_utc(), args.lat, args.lon, args.alt)
            .try_into()
            .unwrap()
    };

    let image = render::render(angle as usize);

    let sx = -1.0;
    let sy = 1.0;
    let cx = args.width as f32 / 2.0;
    let cy = args.height as f32 / 2.0;
    let transform = tiny_skia::Transform::from_row(sx, 0.0, 0.0, sy, cx - sx * cx, cy - sy * cy);

    let mut pixmap = tiny_skia::Pixmap::new(args.width, args.height).unwrap();
    resvg::render(
        &image,
        usvg::FitTo::Width(args.width),
        transform,
        pixmap.as_mut(),
    )
    .unwrap();
    pixmap.save_png(args.output).unwrap();
}
