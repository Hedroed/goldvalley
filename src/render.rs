use usvg::NodeExt;
use std::rc::Rc;

include!(concat!(env!("OUT_DIR"), "/const_gen.rs"));


struct Interpolation {
    data: Vec<f64>,
}

impl Interpolation {
    fn new(data: Vec<f64>) -> Self {
        Self {
            data
        }
    }

    fn interpolate(&self, index: f64, of: f64) -> f64{
        let l = (self.data.len() - 1) as f64;
        let i = index * l / of;

        let min_value: usize = i.floor() as usize;
        let factor = i.fract();

        let mi = self.data[min_value];

        if factor < 1e-10 {
            mi
        } else {
            let ma = self.data[min_value + 1];
            mi + (ma - mi) * factor
        }
    }
}


fn color(offset: f64, r: u8, g: u8, b: u8) -> usvg::Stop {
    usvg::Stop {
        offset: usvg::NormalizedValue::new(offset),
        color: usvg::Color::new_rgb(r, g, b),
        opacity: usvg::NormalizedValue::new(1.0),
    }
}


fn to_svg_segment(seg: &PathSegment) -> usvg::PathSegment {
    match *seg {
        PathSegment::MoveTo { x, y } => usvg::PathSegment::MoveTo { x, y },
        PathSegment::LineTo { x, y } => usvg::PathSegment::LineTo { x, y },
        PathSegment::CurveTo { x1, y1, x2, y2, x, y } => usvg::PathSegment::CurveTo { x1, y1, x2, y2, x, y },
        PathSegment::ClosePath => usvg::PathSegment::ClosePath,
    }
}

fn to_svg_transform(transform: &Transform) -> usvg::Transform {
    usvg::Transform {
        a: transform.a,
        b: transform.b,
        c: transform.c,
        d: transform.d,
        e: transform.e,
        f: transform.f,
    }
}


fn convert_star(star: &[PathSegment; 6]) -> usvg::Path {
    usvg::Path {
        id: String::default(),
        transform: usvg::Transform::default(),
        visibility: usvg::Visibility::Visible,
        fill: Some(usvg::Fill::from_paint(usvg::Paint::Color(usvg::Color::new_rgb(255, 255, 255)))),
        stroke: None,
        rendering_mode: usvg::ShapeRendering::GeometricPrecision,
        text_bbox: None,
        data: Rc::new(usvg::PathData(
            star.iter()
            .map(to_svg_segment)
            .collect()
        )),
    }
}


fn sun_node(sun: &SunPath, angle: usize) -> usvg::Node {
    let mut path = usvg::PathData(
        sun.d.iter()
        .map(to_svg_segment)
        .collect()
    );

    let angle = angle as f64;

    let pivot_x = 800.0;
    let pivot_y = 530.0;

    let a = (-angle - 180.0).to_radians();
    let a_cos = a.cos();
    let a_sin = a.sin();
    let e = -pivot_x * a_cos + pivot_y * a_sin + pivot_x;
    let f = -pivot_x * a_sin - pivot_y * a_cos + pivot_y;

    path.transform(usvg::Transform {
        a: a_cos,
        b: a_sin,
        c: -a_sin,
        d: a_cos,
        e,
        f,
    });

    usvg::Node::new(usvg::NodeKind::Path(usvg::Path {
        id: String::default(),
        transform: usvg::Transform::default(),
        visibility: usvg::Visibility::Visible,
        fill: Some(usvg::Fill::from_paint(usvg::Paint::Color(usvg::Color::new_rgb(sun.fill.r, sun.fill.g, sun.fill.b)))),
        stroke: None,
        rendering_mode: usvg::ShapeRendering::GeometricPrecision,
        text_bbox: None,
        data: Rc::new(path),
    }))
}

fn convert_sun(sun: &SunPath, angle: usize) -> usvg::Node {

    match sun.opacity {
        Some(opacity) => {
            let mut group = usvg::Node::new(usvg::NodeKind::Group(usvg::Group {
                id: String::default(),
                transform: usvg::Transform::default(),
                opacity: usvg::NormalizedValue::new(opacity),
                clip_path: None,
                mask: None,
                filter: Vec::new(),
                filter_fill: None,
                filter_stroke: None,
                enable_background: None,
            }));

            group.append(sun_node(sun, angle));

            group
        }
        None => sun_node(sun, angle)
    }

    
}


fn convert_landscape(land: &LandPath, angle: usize) -> usvg::Path {
    let id = land.id;
    let mut split = id.split('-');
    let pos = split.next().unwrap();

    let colors: &[Color] = match pos {
        "1" => COLORS_LAYER1,
        "2" => COLORS_LAYER2,
        "3" => COLORS_LAYER3,
        "4" => COLORS_LAYER4,
        "5" => COLORS_LAYER5,
        "6" => COLORS_LAYER6,
        "7" => COLORS_LAYER7,
        "8" => COLORS_LAYER8,
        "9" => COLORS_LAYER9,
        "10" => COLORS_LAYER10,
        "11" => COLORS_LAYER11,
        _ => COLORS_LAYER11,
    };

    let c = &colors[angle];

    let path = usvg::PathData(
        land.d.iter()
        .map(to_svg_segment)
        .collect()
    );

    usvg::Path {
        id: String::default(),
        transform: to_svg_transform(&land.transform),
        visibility: usvg::Visibility::Visible,
        fill: Some(usvg::Fill::from_paint(usvg::Paint::Color(usvg::Color::new_rgb(c.r, c.g, c.b)))),
        stroke: None,
        rendering_mode: usvg::ShapeRendering::GeometricPrecision,
        text_bbox: None,
        data: Rc::new(path),
    }
}


pub fn render(angle: usize) -> usvg::Tree {

    let svg = usvg::Svg {
        size: usvg::Size::new(1600.0, 900.0).unwrap(),
        view_box: usvg::ViewBox {
            rect: usvg::Rect::new(0.0, 0.0, 1600.0, 900.0).unwrap(),
            aspect: usvg::AspectRatio::default(),
        },
    };
    let mut tree = usvg::Tree::create(svg);

    // defs

    let c1 = &SKY_ZENITH[angle];
    let c2 = &SKY_MID[angle];
    let c3 = &SKY_HORIZON[angle];

    let inter1 = Interpolation::new(vec![0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.12,0.21,0.21,0.07,0.07,0.0,0.0,0.0,0.0]);
    let inter2 = Interpolation::new(vec![0.66,0.62,0.52,0.52,0.52,0.52,0.58,0.66,0.69,0.55,0.57,0.77,0.78,0.78,0.74,0.74,0.74,0.74,0.55,0.56,0.58,0.58,0.58,0.58,0.58,0.68]);

    let gradient = usvg::BaseGradient {
        units: usvg::Units::UserSpaceOnUse,
        transform: usvg::Transform::default(),
        spread_method: usvg::SpreadMethod::Pad,
        stops: vec![
            color(inter1.interpolate(angle as f64, 360.0), c1.r, c1.g, c1.b),
            color(inter2.interpolate(angle as f64, 360.0), c2.r, c2.g, c2.b),
            color(1.0, c3.r, c3.g, c3.b),
        ]
    };

    let sky_gradient = usvg::LinearGradient {
        id: "a".to_string(),
        x1: 0.0,
        y1: 0.0,
        x2: 0.0,
        y2: 450.0,
        base: gradient.clone(),
    };
    tree.append_to_defs(usvg::NodeKind::LinearGradient(sky_gradient));

    let reflection_gradient = usvg::LinearGradient {
        id: "b".to_string(),
        x1: 0.0,
        y1: 740.0,
        x2: 0.0,
        y2: 520.0,
        base: gradient,
    };
    tree.append_to_defs(usvg::NodeKind::LinearGradient(reflection_gradient));

    let gradient = usvg::BaseGradient {
        units: usvg::Units::UserSpaceOnUse,
        transform: usvg::Transform::default(),
        spread_method: usvg::SpreadMethod::Pad,
        stops: vec![
            usvg::Stop {
                offset: usvg::NormalizedValue::new(0.65),
                color: usvg::Color::new_rgb(0, 14, 39),
                opacity: usvg::NormalizedValue::new(0.0),
            },
            usvg::Stop {
                offset: usvg::NormalizedValue::new(1.0),
                color: usvg::Color::new_rgb(0, 14, 39),
                opacity: usvg::NormalizedValue::new(0.3),
            }
        ]
    };

    let vignette_gradient = usvg::RadialGradient {
        id: "c".to_string(),
        cx: 800.0,
        cy: 1000.0,
        r: usvg::PositiveNumber::new(1400.0),
        fx: 800.0,
        fy: 1000.0,
        base: gradient,
    };
    tree.append_to_defs(usvg::NodeKind::RadialGradient(vignette_gradient));

    // nodes

    let mut root = tree.root();

    let sky = usvg::Path {
        id: String::default(),
        transform: usvg::Transform::default(),
        visibility: usvg::Visibility::Visible,
        fill: Some(usvg::Fill::from_paint(usvg::Paint::Link("a".to_string()))),
        stroke: None,
        rendering_mode: usvg::ShapeRendering::GeometricPrecision,
        text_bbox: None,
        data: Rc::new(usvg::PathData::from_rect(usvg::Rect::new(0.0, 0.0, 1600.0, 540.0).unwrap())),
    };
    root.append_kind(usvg::NodeKind::Path(sky));

    let reflection = usvg::Path {
        id: String::default(),
        transform: usvg::Transform::default(),
        visibility: usvg::Visibility::Visible,
        fill: Some(usvg::Fill::from_paint(usvg::Paint::Link("b".to_string()))),
        stroke: None,
        rendering_mode: usvg::ShapeRendering::GeometricPrecision,
        text_bbox: None,
        data: Rc::new(usvg::PathData::from_rect(usvg::Rect::new(0.0, 540.0, 1600.0, 360.0).unwrap())),
    };
    root.append_kind(usvg::NodeKind::Path(reflection));

    if angle <= 95 || angle >= 270 {
        for star in STARS {
            let elem = convert_star(star);
            root.append_kind(usvg::NodeKind::Path(elem));
        }
    }

    // sun
    if angle > 95 && angle < 270 {
        for sun in SUN {
            let elem = convert_sun(sun, angle);
            root.append(elem);
        }
    }


    for land in LANDSCAPE {
        let elem = convert_landscape(land, angle);
        root.append_kind(usvg::NodeKind::Path(elem));
    }

    let bottom_color = &COLORS_LAYER11[angle];

    let bottom = usvg::Path {
        id: String::default(),
        transform: usvg::Transform::default(),
        visibility: usvg::Visibility::Visible,
        fill: Some(usvg::Fill::from_paint(usvg::Paint::Color(usvg::Color::new_rgb(bottom_color.r, bottom_color.g, bottom_color.b)))),
        stroke: None,
        rendering_mode: usvg::ShapeRendering::GeometricPrecision,
        text_bbox: None,
        data: Rc::new(usvg::PathData::from_rect(usvg::Rect::new(0.0, 714.0, 1600.0, 186.0).unwrap())),
    };
    root.append_kind(usvg::NodeKind::Path(bottom));

    let vignette = usvg::Path {
        id: String::default(),
        transform: usvg::Transform::default(),
        visibility: usvg::Visibility::Visible,
        fill: Some(usvg::Fill::from_paint(usvg::Paint::Link("c".to_string()))),
        stroke: None,
        rendering_mode: usvg::ShapeRendering::GeometricPrecision,
        text_bbox: None,
        data: Rc::new(usvg::PathData::from_rect(usvg::Rect::new(0.0, 0.0, 1600.0, 900.0).unwrap())),
    };
    root.append_kind(usvg::NodeKind::Path(vignette));

    // let debug = tree.to_string(&usvg::XmlOptions::default());
    // println!("{}", debug);

    tree
}