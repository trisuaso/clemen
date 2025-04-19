use clemen::layouts::{LayoutType, element::Element, flexible::Direction};
use std::time::SystemTime;

fn main() {
    let mut root = Element::new(
        (200.0.into(), 100.0.into()),
        (0.0.into(), 0.0.into()),
        LayoutType::Flexible,
    );
    root.sublayout.col = true; // this creates a column layout

    let start = SystemTime::now();
    for _ in 0..100 {
        root.sublayout.add(Element::new(
            (200.0.into(), 50.0.into()),
            (0.0.into(), 0.0.into()),
            LayoutType::Block,
        ));
    }

    // root.sublayout.resize_flexible(Direction::X);
    root.sublayout.resize_flexible(Direction::Y);
    // root.sublayout.revert_flexible();

    println!(
        "finished calculating, took: {}μs",
        start.elapsed().unwrap().as_micros()
    );

    std::fs::write("out.html", root.html()).unwrap();
}
