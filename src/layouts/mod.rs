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

#[derive(Clone, Debug)]
pub struct Layout {
    /// The elements contained within the layout.
    pub(crate) inner: Vec<Element>,
    /// The variant of layout this iss.
    pub variant: LayoutType,
    /// The spacing between each element in the block layout.
    pub offset: f64,
    /// The size of the containing element.
    pub size: Vector2,
    /// If the flexible layout should wrap elements instead of shrinking.
    /// This essentially creates a column layout.
    pub col: bool,
}

impl Layout {
    /// Create a new [`Layout`].
    pub fn new(r#type: LayoutType, size: Vector2) -> Self {
        Self {
            inner: Vec::new(),
            variant: r#type,
            offset: 0.0,
            size,
            col: false,
        }
    }

    /// Recalculate element sizes/positions using the correct calculator for the layout type.
    pub fn recalculate(&mut self) -> () {
        match self.variant {
            LayoutType::Flexible => self.recalculate_as_flexible(),
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
