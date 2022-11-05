use palette::{Alpha, rgb::Rgb, LinSrgb, LinSrgba};
use palette::Blend;

fn main() {

    // let c: LinSrgba = Alpha::<Rgb<palette::encoding::linear::Linear<palette::encoding::srgb::Srgb>, u8>, f32>::new(37,32,70,0.33).into_format::<f32, f32>();
    let c: LinSrgba = Alpha::<Rgb<palette::encoding::linear::Linear<palette::encoding::srgb::Srgb>, u8>, f32>::new(59,66,108,0.78).into_format::<f32, f32>();

    let bg = LinSrgb::<u8>::new(0x0d,0x13,0x3a).into_format();
    let bg = Alpha { color: bg, alpha: 1.0 };

    println!("color before l {:?}", c.into_format::<u8, f32>());

    let over = c.over(bg);
    println!("color over l   {:?}", over.into_format::<u8, f32>().color);

}
