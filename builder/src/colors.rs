use palette::{Blend, Alpha, LinSrgb, LinSrgba, Gradient};
use yaml_rust::Yaml;
use const_gen::CompileConst;


pub fn convert_alpha(source: LinSrgba) -> LinSrgb {
    let bg = LinSrgb::<u8>::new(0x0d,0x13,0x3a).into_format();
    let bg = Alpha { color: bg, alpha: 1.0 };
    let over = source.over(bg);
    over.color
}


pub enum ColorType {
    Normal(LinSrgb),
    WithAlpha(LinSrgba),
}


pub fn read_color(color_code: &str) -> ColorType {
    let css_color = csscolorparser::parse(color_code).unwrap();

    if css_color.a < 1.0 {
        let color = LinSrgba::<f64>::new(css_color.r,css_color.g,css_color.b,css_color.a).into_format::<f32, f32>();
        ColorType::WithAlpha(color)
    } else {
        let color = LinSrgb::<f64>::new(css_color.r,css_color.g,css_color.b).into_format();
        ColorType::Normal(color)
    }
}

fn load_color(color_code: &str) -> LinSrgb {
    let color_type = read_color(color_code);
    match color_type {
        ColorType::Normal(color) => color,
        ColorType::WithAlpha(color) => convert_alpha(color)
    }
}

#[derive(CompileConst, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<LinSrgb> for Color {
    fn from(color: LinSrgb) -> Self {
        let c = color.into_format();
        Color {
            r: c.red,
            g: c.green,
            b: c.blue,
        }
    }
}

pub fn load_colors(data: &Yaml) -> Vec<Color> {

    let gradient_colors: Vec<LinSrgb> = data.clone()
        .into_iter()
        .map(|d| load_color(&d.into_string().unwrap()))
        .collect();

    let gradient = Gradient::new(gradient_colors);

    gradient.take(360)
    .map(|d| d.into())
    .collect()
}
