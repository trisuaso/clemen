use clemen::layouts::{LayoutProperties, LayoutType, element::Element};
use std::time::SystemTime;

fn main() {
    let mut root = Element::new(
        (200.0.into(), 100.0.into()),
        (0.0.into(), 0.0.into()),
        LayoutType::Block,
    );
    root.sublayout.properties = LayoutProperties {
        offset: 0.0,
        ..Default::default()
    };

    let start = SystemTime::now();
    for _ in 0..10 {
        root.sublayout.add(Element::new(
            (100.0.into(), 100.0.into()),
            (0.0.into(), 0.0.into()),
            LayoutType::Block,
        ));
    }

    println!(
        "finished calculating, took: {}μs",
        start.elapsed().unwrap().as_micros()
    );

    std::fs::write("out.html", root.html()).unwrap();
}
