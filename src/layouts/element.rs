use super::{Layout, LayoutType};

/// A 2D vector.
pub type Vector2 = (f64, f64);

/// The way and element is positioned.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PositionStyle {
    /// Relative to the other elements within its container.
    Relative,
    /// Not counted as a child of the element; position and size values are absolute.
    Absolute,
}

impl Default for PositionStyle {
    fn default() -> Self {
        Self::Relative
    }
}

/// Specific elements which change the behavior of an [`Element`] in layouts.
#[derive(Clone, Debug)]
pub struct ElementAttributes {
    /// The minimum sizes of the element. The element's width or height cannot
    /// fall below these values
    pub min_size: Option<Vector2>,
    /// The maximum sizes of the element. The element's width or height cannot
    /// fall below these values
    pub max_size: Option<Vector2>,
    /// The type of positioning the element follows.
    pub style: PositionStyle,
}

impl Default for ElementAttributes {
    fn default() -> Self {
        Self {
            min_size: Some((0.5, 0.5)),
            max_size: None,
            style: PositionStyle::default(),
        }
    }
}

/// An element is a single node in a layout.
#[derive(Clone, Debug)]
pub struct Element {
    /// The size of the element.
    pub size: Vector2,
    /// The position of the element.
    pub position: Vector2,
    /// The size of the element. This value does not change in flexible layouts.
    pub real_size: Vector2,
    /// The real position of the element.
    pub real_position: Vector2,
    /// Specific elements which change the behavior of the element in layouts.
    pub attrs: ElementAttributes,
    /// The sub-layout of the element.
    pub sublayout: Layout,
}

impl Element {
    /// Create a new [`Element`].
    pub fn new(size: Vector2, position: Vector2, display: LayoutType) -> Self {
        Self {
            size,
            position,
            real_size: size.clone(),
            real_position: position.clone(),
            attrs: ElementAttributes::default(),
            sublayout: Layout::new(display, size),
        }
    }

    /// Move the element.
    pub fn goto(&mut self, to: Vector2) {
        self.position = to;
    }

    /// Change the size of the element.
    pub fn resize(&mut self, to: Vector2) {
        self.size = to;
        self.sublayout.size = self.size;
    }

    /// Render as HTML for testing.
    pub fn html(&self) -> String {
        let layout_html = self.sublayout.html();
        format!(
            "<layout style=\"border: inset 1px red; position: absolute; left: {}px; top: {}px; width: {}px; height: {}px;\">{layout_html}</layout>",
            self.position.0, self.position.1, self.size.0, self.size.1
        )
    }
}
