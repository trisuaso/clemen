use clemen::layouts::{
    AlignmentX, LayoutProperties, LayoutType, element::Element, flexible::Direction,
};
use std::time::SystemTime;

fn main() {
    let mut root = Element::new(
        (500.0.into(), 100.0.into()),
        (0.0.into(), 0.0.into()),
        LayoutType::Flexible,
    );
    root.sublayout.properties = LayoutProperties {
        offset: 0.0,
        align_x: AlignmentX::Center,
        flex_grow: false,
        ..Default::default()
    };

    let start = SystemTime::now();
    for _ in 0..2 {
        root.sublayout.add(Element::new(
            (100.0.into(), 100.0.into()),
            (0.0.into(), 0.0.into()),
            LayoutType::Block,
        ));
    }

    root.sublayout.resize_flexible(Direction::X);
    // root.sublayout.revert_flexible();

    println!(
        "finished calculating, took: {}μs",
        start.elapsed().unwrap().as_micros()
    );

    std::fs::write("out.html", root.html()).unwrap();
}
