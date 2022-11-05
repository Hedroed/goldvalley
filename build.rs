// build.rs

use const_gen::*;
use std::{env, fs, path::Path};
use std::str::FromStr;
use yaml_rust::YamlLoader;

use builder::colors::{Color, load_colors};


#[derive(CompileConst)]
pub struct SunPath {
    d: Vec<PathSegment>,
    opacity: Option<f64>,
    fill: Color,
}


#[derive(CompileConst)]
pub struct LandPath {
    d: Vec<PathSegment>,
    id: String,
    transform: Transform
}


#[derive(CompileConst)]
pub struct Transform {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
}

impl From<usvg::Transform> for Transform {
    fn from(transform: usvg::Transform) -> Self {
        Self {
            a: transform.a,
            b: transform.b,
            c: transform.c,
            d: transform.d,
            e: transform.e,
            f: transform.f,
        }
    }
}


#[derive(CompileConst)]
pub enum PathSegment {
    MoveTo {
        x: f64,
        y: f64,
    },
    LineTo {
        x: f64,
        y: f64,
    },
    CurveTo {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        x: f64,
        y: f64,
    },
    ClosePath,
}

impl From<&usvg::PathSegment> for PathSegment {
    fn from(segment: &usvg::PathSegment) -> Self {
        match segment {
            usvg::PathSegment::MoveTo { x, y } => PathSegment::MoveTo { x: *x, y: *y },
            usvg::PathSegment::LineTo { x, y } => PathSegment::LineTo { x: *x, y: *y },
            usvg::PathSegment::CurveTo { x1, y1, x2, y2, x, y } => PathSegment::CurveTo { x1: *x1, y1: *y1, x2: *x2, y2: *y2, x: *x, y: *y },
            usvg::PathSegment::ClosePath => PathSegment::ClosePath,
        }
    }
}


fn circle_to_path(cx: f64, cy: f64, r: f64) -> [PathSegment; 6] {
    let h = r / 2.0;

    [
        PathSegment::MoveTo { x: cx + r, y: cy },
        PathSegment::CurveTo { x1: cx + r, y1: cy + h, x2: cx + h, y2: cy + r, x: cx,     y: cy + r},
        PathSegment::CurveTo { x1: cx - h, y1: cy + r, x2: cx - r, y2: cy + h, x: cx - r, y: cy},
        PathSegment::CurveTo { x1: cx - r, y1: cy - h, x2: cx - h, y2: cy - r, x: cx,     y: cy - r},
        PathSegment::CurveTo { x1: cx + h, y1: cy - r, x2: cx + r, y2: cy - h, x: cx + r, y: cy},
        PathSegment::ClosePath,
    ]
}



fn main() {
    // Use the OUT_DIR environment variable to get an
    // appropriate path.
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("const_gen.rs");

    // stars
    let mut star_vec: Vec<[PathSegment; 6]> = Vec::new();

    let text = std::fs::read_to_string("src/data/stars.svg").unwrap();
    let doc = roxmltree::Document::parse(&text).unwrap();

    let root = doc.root();
    let svg_node = root.first_child().unwrap();

    for child in svg_node.children().filter(roxmltree::Node::is_element) {
        let attrs = child.attributes();
        let x = f64::from_str(attrs[0].value()).unwrap();
        let y = f64::from_str(attrs[1].value()).unwrap();
        let r = f64::from_str(attrs[2].value()).unwrap();

        let zoom = 16.0 / 6.0;
        let star = circle_to_path(
            (x * zoom).round(),
            (y * zoom - 400.0).round(),
            (r * zoom).round(),
        );
        star_vec.push(star);
    }


    // landscape
    let mut landscape_vec: Vec<LandPath> = Vec::new();

    let opt = usvg::Options::default();
    let svg_data = std::fs::read("src/data/landscape.svg").unwrap();
    let rtree = usvg::Tree::from_data(&svg_data, &opt.to_ref()).unwrap();

    let root = rtree.root();
    for node in root.children() {
        let node_intern = node.borrow();
        if let usvg::NodeKind::Path(path) = &*node_intern {
            let id = path.id.clone();
            let transform = path.transform;
            let path_data = &path.data.0;
            let d: Vec<PathSegment> = path_data.iter().map(PathSegment::from).collect();

            landscape_vec.push(LandPath {
                d,
                id,
                transform: transform.into(),
            });
        }
    }

    // sun
    let mut sun_vec: Vec<SunPath> = Vec::new();

    let opt = usvg::Options::default();
    let svg_data = std::fs::read("src/data/sun.svg").unwrap();
    let rtree = usvg::Tree::from_data(&svg_data, &opt.to_ref()).unwrap();

    let root = rtree.root();
    for node in root.children() {
        let node_intern = node.borrow();

        match &*node_intern {
            usvg::NodeKind::Path(path) => {
                let path_data = &path.data.0;
                let d: Vec<PathSegment> = path_data.iter().map(PathSegment::from).collect();

                let fill = if let usvg::Paint::Color(c) = path.fill.as_ref().unwrap().paint {
                    Color {
                        r: c.red,
                        g: c.green,
                        b: c.blue,
                    }
                } else {
                    Color {
                        r: 0,
                        g: 0,
                        b: 0,
                    }
                };

                sun_vec.push(SunPath {
                    d,
                    fill,
                    opacity: None,
                });
            },
            usvg::NodeKind::Group(group) => {
                let opacity = group.opacity.value();

                let n = node.children().next().unwrap();
                let n_intern = n.borrow();

                if let usvg::NodeKind::Path(path) = &*n_intern {
                    let path_data = &path.data.0;
                    let d: Vec<PathSegment> = path_data.iter().map(PathSegment::from).collect();

                    let fill = if let usvg::Paint::Color(c) = path.fill.as_ref().unwrap().paint {
                        Color {
                            r: c.red,
                            g: c.green,
                            b: c.blue,
                        }
                    } else {
                        Color {
                            r: 0,
                            g: 0,
                            b: 0,
                        }
                    };

                    sun_vec.push(SunPath {
                        d,
                        fill,
                        opacity: Some(opacity),
                    });
                }
            }
            _ => {}
        }
    }


    // colors

    let data = std::fs::read_to_string("src/data/colors.yaml").unwrap();
    let docs = YamlLoader::load_from_str(&data).unwrap();
    let gradient_names = &docs[0];

    let sky_zenith: Vec<Color> = load_colors(&gradient_names["sky_zenith"]);
    let sky_mid: Vec<Color> = load_colors(&gradient_names["sky_mid"]);
    let sky_horizon: Vec<Color> = load_colors(&gradient_names["sky_horizon"]);

    let layer1: Vec<Color> = load_colors(&gradient_names["layer1"]);
    let layer2: Vec<Color> = load_colors(&gradient_names["layer2"]);
    let layer3: Vec<Color> = load_colors(&gradient_names["layer3"]);
    let layer4: Vec<Color> = load_colors(&gradient_names["layer4"]);
    let layer5: Vec<Color> = load_colors(&gradient_names["layer5"]);
    let layer6: Vec<Color> = load_colors(&gradient_names["layer6"]);
    let layer7: Vec<Color> = load_colors(&gradient_names["layer7"]);
    let layer8: Vec<Color> = load_colors(&gradient_names["layer8"]);
    let layer9: Vec<Color> = load_colors(&gradient_names["layer9"]);
    let layer10: Vec<Color> = load_colors(&gradient_names["layer10"]);
    let layer11: Vec<Color> = load_colors(&gradient_names["layer11"]);

    let const_declarations = vec! {
        const_definition!(#[derive(Debug)] pub LandPath),
        const_definition!(#[derive(Debug)] pub SunPath),
        const_definition!(#[derive(Debug)] pub Transform),
        const_definition!(#[derive(Debug)] pub PathSegment),
        const_definition!(#[derive(Debug)] pub Color),

        const_declaration!(STARS = star_vec),
        const_declaration!(LANDSCAPE = landscape_vec),
        const_declaration!(SUN = sun_vec),

        const_declaration!(SKY_ZENITH = sky_zenith),
        const_declaration!(SKY_MID = sky_mid),
        const_declaration!(SKY_HORIZON = sky_horizon),

        const_declaration!(COLORS_LAYER1 = layer1),
        const_declaration!(COLORS_LAYER2 = layer2),
        const_declaration!(COLORS_LAYER3 = layer3),
        const_declaration!(COLORS_LAYER4 = layer4),
        const_declaration!(COLORS_LAYER5 = layer5),
        const_declaration!(COLORS_LAYER6 = layer6),
        const_declaration!(COLORS_LAYER7 = layer7),
        const_declaration!(COLORS_LAYER8 = layer8),
        const_declaration!(COLORS_LAYER9 = layer9),
        const_declaration!(COLORS_LAYER10 = layer10),
        const_declaration!(COLORS_LAYER11 = layer11),
    }.join("\n");

    // Note: The `const_definition!` and `const_declaration!`
    // macros above are just simple wrappers for CompileConst
    // trait methods of the same name. Using those methods
    // would entail the following sytax:
    // TestStruct::const_definition("#[derive(Debug)]")
    // test_struct.const_declaration("TEST_STRUCT")
    // These may be preferable in cases where const names
    // or type attributes have been procedurally generated
    // somehow and need to be treated as strings.

    // If the "phf" feature is enabled, this crate will also
    // support converting HashMap and HashSet types into
    // compile-time constant phf map and set types respectively.

    // Lastly, output to the destination file.
    fs::write(&dest_path, const_declarations).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/data/colors.yaml");
    println!("cargo:rerun-if-changed=src/data/landscape.svg");
    println!("cargo:rerun-if-changed=src/data/stars.svg");
}
