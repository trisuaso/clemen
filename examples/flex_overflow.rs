use clemen::layouts::{LayoutType, element::Element};
use std::time::SystemTime;

fn main() {
    let mut root = Element::new((200.0, 100.0), (0.0, 0.0), LayoutType::Flexible);
    root.sublayout.offset = 0.0;
    // root.sublayout.flex_wrap = true;

    let start = SystemTime::now();
    for _ in 0..100 {
        root.sublayout
            .add(Element::new((100.0, 100.0), (0.0, 0.0), LayoutType::Block));
    }

    root.sublayout.resize_flexible();
    // root.sublayout.revert_flexible();

    println!(
        "finished calculating, took: {}μs",
        start.elapsed().unwrap().as_micros()
    );

    std::fs::write("out.html", root.html()).unwrap();
}
