use clemen::layouts::{LayoutType, element::Element};
use std::time::SystemTime;

fn main() {
    let mut root = Element::new((210.0, 100.0), (0.0, 0.0), LayoutType::Block);
    root.sublayout.offset = 10.0;

    let start = SystemTime::now();
    for _ in 0..10 {
        root.sublayout
            .add(Element::new((100.0, 100.0), (0.0, 0.0), LayoutType::Block));
    }

    println!(
        "finished calculating, took: {}μs",
        start.elapsed().unwrap().as_micros()
    );

    std::fs::write("out.html", root.sublayout.html()).unwrap();
}
