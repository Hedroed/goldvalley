use yaml_rust::YamlLoader;

use builder::colors::{Color, load_colors};


fn main() {

    let path = std::env::args().into_iter().nth(1).unwrap();
    let data = std::fs::read_to_string(&path).unwrap();
    let docs = YamlLoader::load_from_str(&data).unwrap();

    let gradient_names = &docs[0];
    println!("debug {:?}", gradient_names);

    let colors: Vec<Color> = load_colors(&gradient_names["layer1"]);

    println!("colors {:?}", colors);
}
