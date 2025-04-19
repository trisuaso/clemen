use super::{
    Layout, LayoutType,
    element::{Element, Vector2},
};

/// A flexible layout attempts to shrink elements within it on overflow.
///
/// `amount to shrink each element = overflow amount / number of elements`
impl Layout {
    /// Perform element resize calculations.
    pub fn resize_flexible(&mut self) -> () {
        if self.variant != LayoutType::Flexible {
            panic!("cannot run size calculations outside of a flexible layout");
        }

        // shrink
        let mut overflowing_pixels: f64 = 0.0;
        let mut is_first_overflowing: bool = true;

        for element in self.inner.iter() {
            if element.position.0 + element.size.0 + self.offset > self.size.0 {
                if is_first_overflowing {
                    // the first element to overflow is slightly more complicated
                    // because some of it is likely not overflowing... this means
                    // we need to calculate how much is ACTUALLY outside
                    overflowing_pixels +=
                        (self.size.0 - (element.size.0 + element.position.0)).abs();
                    is_first_overflowing = false;
                } else {
                    // everything else is guaranteed to be 100% outside of the box
                    overflowing_pixels += element.size.0 + self.offset;
                }
            }
        }

        let amount_to_shrink_all_elements = overflowing_pixels / self.inner.len() as f64;

        for (i, element) in self.inner.iter_mut().enumerate() {
            let mut new_size = element.size.clone();
            new_size.0 -= amount_to_shrink_all_elements;
            element.resize(new_size);

            if element.position.0 != 0.0 {
                let mut new_pos = element.position.clone();
                new_pos.0 -= amount_to_shrink_all_elements * if i > 1 { i as f64 } else { 1.0 };
                element.goto(new_pos);
            }
        }

        // grow
        // the good news here is this is the same thing as shrinking, just we
        // calculate how many pixels we can expand to, and then add instead of subtract
        // from each element!
        if overflowing_pixels == 0.0 {
            // nothing is overflowing, that means there is extra room waiting to be used!
            let mut extra_pixels: f64 = 0.0;

            for element in self.inner.iter() {
                extra_pixels += element.size.0;
            }

            extra_pixels = self.size.0 - extra_pixels;
            let amount_to_grow_all_elements = extra_pixels / self.inner.len() as f64;

            for (i, element) in self.inner.iter_mut().enumerate() {
                let mut new_size = element.size.clone();
                new_size.0 += amount_to_grow_all_elements;
                element.resize(new_size);

                if element.position.0 != 0.0 {
                    let mut new_pos = element.position.clone();
                    new_pos.0 += amount_to_grow_all_elements * if i > 1 { i as f64 } else { 1.0 };
                    element.goto(new_pos);
                }
            }
        }
    }

    /// Undo resizing in a flexible layout. Should be done before adding more elements
    /// to the layout. After all elements are added, [`Self::resize_flexible`] should
    /// be run.
    pub fn revert_flexible(&mut self) -> () {
        if self.variant != LayoutType::Flexible {
            panic!("cannot run size calculations outside of a flexible layout");
        }

        for element in self.inner.iter_mut() {
            element.size = element.real_size.clone();
            element.position = element.real_position.clone();
        }
    }

    pub(crate) fn recalculate_as_flexible(&mut self) {
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

            if (new_pos.0 + element.size.0 + self.offset) > self.size.0 && self.flex_wrap {
                // do wrap
                let first_of_row = if let Some(ref e) = previous_on_new_row {
                    // this means that this is not the first time we're going on a new row
                    // this should be a clone of the first element of the current row (that is full)...
                    // this is not a mutable reference, so it isn't to be edited
                    e
                } else {
                    self.inner.get(0).unwrap()
                };

                // we're doing tallest height + first.position.1(y) so that we're
                // under the first element AND under the tallest element
                new_pos = (0.0, tallest_of_row + first_of_row.position.1 + self.offset);
                previous_on_new_row = Some(element.clone());
                tallest_of_row = element.size.1; // this is the first element in the new row, so it is the tallest
            }

            if element.size.1 > tallest_of_row {
                tallest_of_row = element.size.1;
            }

            element.real_position = new_pos.clone();
            element.goto(new_pos);
            previous_element = Some(element);
        }

        // ...
        self.inner = cloned;
    }
}
