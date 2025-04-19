use clemen::layouts::{
    LayoutType,
    element::{Element, PositionStyle},
    flexible::Direction,
};
use std::time::SystemTime;

fn main() {
    let mut root = Element::new(
        (200.0.into(), 100.0.into()),
        (0.0.into(), 0.0.into()),
        LayoutType::Flexible,
    );

    let start = SystemTime::now();

    // add first element
    root.sublayout.add(Element::new(
        (100.0.into(), 100.0.into()),
        (0.0.into(), 0.0.into()),
        LayoutType::Block,
    ));

    // add an absolutely positioned second element
    let mut element = Element::new(
        (100.0.into(), 100.0.into()),
        (100.0.into(), 100.0.into()),
        LayoutType::Block,
    );

    element.attrs.style = PositionStyle::Absolute;
    root.sublayout.add(element);

    // add an extra element to make sure that regular positioning works
    // this should render where the second element usually is
    root.sublayout.add(Element::new(
        (100.0.into(), 100.0.into()),
        (0.0.into(), 0.0.into()),
        LayoutType::Block,
    ));

    // test resizing
    root.sublayout.resize_flexible(Direction::X);

    // done
    println!(
        "finished calculating, took: {}μs",
        start.elapsed().unwrap().as_micros()
    );

    std::fs::write("out.html", root.html()).unwrap();
}
