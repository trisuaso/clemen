use super::{
    Layout,
    element::{Element, Vector2},
};

/// A block layout does **not** resize any element placed into it. The block layout
/// only supports rows, as anything overflowing just goes onto the next row.
impl Layout {
    pub(crate) fn recalculate_as_block(&mut self) {
        let mut previous_element: Option<&mut Element> = None;
        let mut previous_on_new_row: Option<Element> = None;
        let mut tallest_of_row: f64 = 0.0;

        let mut cloned = self.inner.clone();

        for element in cloned.iter_mut() {
            let pre = match previous_element {
                Some(ref e) => e,
                None => {
                    // not really anything to adjust since this is the first element
                    previous_element = Some(element);
                    continue;
                }
            };

            let mut new_pos: Vector2 = (
                pre.position.0 + pre.size.0 + self.offset,
                pre.position.1, // we're going to stay on the same y level (unless we overflow)
            );

            if (new_pos.0 + element.size.0 + self.offset) > self.size.0 {
                // we're overflowing... we need to move down to the next line
                let first_of_row = if let Some(ref e) = previous_on_new_row {
                    // this means that this is not the first time we're going on a new row
                    // this should be a clone of the first element of the current row (that is full)...
                    // this is not a mutable reference, so it isn't to be edited
                    e
                } else {
                    self.inner.get(0).unwrap()
                };

                if tallest_of_row == 0.0 {
                    // this fixes an edge case where the second element
                    // immediately needs a new row, but `tallest_of_row` isn't
                    // set since the previous element didn't overflow, causing
                    // the second element to render over the first
                    //
                    // since `tallest_of_row` hasn't been set yet (since it is 0),
                    // we can assume that the first of the previous row is the tallest
                    // as a fallback
                    tallest_of_row = first_of_row.size.1;
                }

                // we're doing tallest height + first.position.1(y) so that we're
                // under the first element AND under the tallest element
                new_pos = (0.0, tallest_of_row + first_of_row.position.1 + self.offset);
                previous_on_new_row = Some(element.clone());
                tallest_of_row = element.size.1; // this is the first element in the new row, so it is the tallest
            }

            if element.size.1 > tallest_of_row {
                tallest_of_row = element.size.1;
            }

            element.goto(new_pos);
            previous_element = Some(element);
        }

        self.inner = cloned;
    }
}
