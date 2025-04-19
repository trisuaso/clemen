pub mod block;
pub mod element;
pub mod flexible;

use element::{Element, Vector2};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LayoutType {
    /// A layout which attempts to resize elements to fit.
    Flexible,
    /// A layout which preserves element size and skips to the next row on overflow.
    Block,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AlignmentX {
    Left,
    Center,
}

impl Default for AlignmentX {
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AlignmentY {
    Top,
    Center,
}

impl Default for AlignmentY {
    fn default() -> Self {
        Self::Top
    }
}

#[derive(Clone, Debug)]
pub struct LayoutProperties {
    /// The spacing between each element.
    pub offset: f64,
    /// The spacing between each element and the layout bounding box.
    pub padding: f64,
    /// The horizontal alignment of elements in flexible layouts.
    pub align_x: AlignmentX,
    /// The verticle alignment of elements in flexible layouts.
    pub align_y: AlignmentY,
    /// If elements in a flexible container automatically grow to fill the container.
    pub flex_grow: bool,
}

impl Default for LayoutProperties {
    fn default() -> Self {
        Self {
            offset: 0.0,
            padding: 0.0,
            align_x: AlignmentX::default(),
            align_y: AlignmentY::default(),
            flex_grow: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Layout {
    /// The elements contained within the layout.
    pub(crate) inner: Vec<Element>,
    /// The variant of layout this iss.
    pub variant: LayoutType,
    /// The size of the containing element.
    pub size: Vector2,
    /// If the flexible layout should wrap elements instead of shrinking.
    /// This essentially creates a column layout.
    pub col: bool,
    /// Layout configuration.
    pub properties: LayoutProperties,
}

impl Layout {
    /// Create a new [`Layout`].
    pub fn new(r#type: LayoutType, size: Vector2) -> Self {
        Self {
            inner: Vec::new(),
            variant: r#type,
            size,
            col: false,
            properties: LayoutProperties::default(),
        }
    }

    /// Recalculate element sizes/positions using the correct calculator for the layout type.
    pub fn recalculate(&mut self) -> () {
        match self.variant {
            LayoutType::Flexible => self.recalculate_as_flexible((0.0, 0.0)),
            LayoutType::Block => self.recalculate_as_block(),
        }
    }

    /// Add an element to the layout and calculate its position/size.
    ///
    /// Also adjusts the size of every other element in the layout.
    ///
    /// # Arguments
    /// * `element`
    ///
    /// # Returns
    /// New element's index in the layout.
    pub fn add(&mut self, element: Element) -> usize {
        self.inner.push(element);
        self.recalculate();
        self.inner.len()
    }

    /// Remove an element from the layout and calculate the position/size
    /// of remaining elements.
    pub fn remove(&mut self, idx: usize) -> () {
        self.inner.remove(idx);
        self.recalculate();
    }

    /// Convert the entire layout to HTML for testing.
    pub fn html(&self) -> String {
        let mut out: String = String::new();

        for (i, element) in self.inner.iter().enumerate() {
            out.push_str(&format!(
                "<div style=\"position: absolute; left: {}px; top: {}px; width: {}px; height: {}px\" id=\"{i}\"></div>",
                element.position.0,
                element.position.1,
                element.size.0,
                element.size.1
            ));
        }

        format!(
            "<style>
                div:nth-child(even) {{
                    background: red;
                }}

                div:nth-child(odd) {{
                    background: blue;
                }}

                div:nth-child(3n) {{
                    background: purple;
                }}

                div:nth-child(4n) {{
                    background: green;
                }}
            </style>{out}"
        )
    }
}
