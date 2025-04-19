use super::{
    AlignmentX, Layout, LayoutType,
    element::{Element, PositionStyle, Vector2},
};

#[derive(PartialEq, Eq)]
pub enum Direction {
    X,
    Y,
}

/// A flexible layout attempts to shrink elements within it on overflow.
///
/// `amount to shrink each element = overflow amount / number of elements`
impl Layout {
    /// Perform element resize calculations.
    pub fn resize_flexible(&mut self, direction: Direction) -> () {
        if self.variant != LayoutType::Flexible {
            panic!("cannot run size calculations outside of a flexible layout");
        }

        let boundary = if direction == Direction::X {
            self.size.0
        } else {
            self.size.1
        };

        // align x
        if self.properties.align_x == AlignmentX::Center {
            let mut spacing_from_center: Vec<f64> = Vec::new(); // the offset of each element from each other's positions BEFORE moving
            let center_value = if direction == Direction::X {
                self.size.0 / 2.0
            } else {
                self.size.1 / 2.0
            };

            let center_idx = self.inner.len() / 2;

            // get the spacing of every other element from the center element
            let center = self.inner.get(center_idx);
            if let Some(center) = center {
                let center_pos_value = if direction == Direction::X {
                    center.position.0
                } else {
                    center.position.1
                };

                for (i, element) in self.inner.iter().enumerate() {
                    let position_value = if direction == Direction::X {
                        element.position.0
                    } else {
                        element.position.1
                    };

                    spacing_from_center.insert(i, center_pos_value - position_value);
                }
            }

            // get center element
            let len = self.inner.len();
            let center = self.inner.get_mut(center_idx);
            if let Some(center) = center {
                // move center element to the center
                let size_value = if direction == Direction::X {
                    center.size.0
                } else {
                    center.size.1
                };

                let mut center_new_pos = center.position.clone();

                if len % 2 == 0 {
                    // even number of elements
                    if direction == Direction::X {
                        center_new_pos.0 = center_value - size_value;
                    } else {
                        center_new_pos.1 = center_value - size_value;
                    }
                } else {
                    // odd number of elements
                    if direction == Direction::X {
                        center_new_pos.0 = center_value - (size_value / 2.0);
                    } else {
                        center_new_pos.1 = center_value - (size_value / 2.0);
                    }
                }

                center.goto(center_new_pos);

                // adjust all other elements
                for (i, element) in self.inner.iter_mut().enumerate() {
                    // get element center offset idx
                    let center_offset = spacing_from_center.get(i).unwrap();
                    let mut new_pos = element.position.clone();

                    if direction == Direction::X {
                        new_pos.0 = center_new_pos.0 + center_offset;
                    } else {
                        new_pos.1 = center_new_pos.1 + center_offset;
                    }

                    element.goto(new_pos);
                }
            }
        }

        // shrink
        let mut overflowing_pixels: f64 = 0.0;
        let mut is_first_overflowing: bool = true;

        for element in self.inner.iter() {
            if element.attrs.style == PositionStyle::Absolute {
                continue;
            }

            let size_value = if direction == Direction::X {
                element.size.0
            } else {
                element.size.1
            };

            let position_value = if direction == Direction::X {
                element.position.0
            } else {
                element.position.1
            };

            if position_value + size_value + self.properties.offset > boundary {
                if is_first_overflowing {
                    // the first element to overflow is slightly more complicated
                    // because some of it is likely not overflowing... this means
                    // we need to calculate how much is ACTUALLY outside
                    overflowing_pixels += (boundary - (size_value + position_value)).abs();
                    is_first_overflowing = false;
                } else {
                    // everything else is guaranteed to be 100% outside of the box
                    overflowing_pixels += size_value + self.properties.offset;
                }
            }
        }

        let amount_to_shrink_all_elements = overflowing_pixels / self.inner.len() as f64;

        for (i, element) in self.inner.iter_mut().enumerate() {
            if element.attrs.style == PositionStyle::Absolute {
                continue;
            }

            let mut new_size = element.size.clone();

            if direction == Direction::X {
                new_size.0 -= amount_to_shrink_all_elements;
            } else {
                new_size.1 -= amount_to_shrink_all_elements;
            };

            element.resize(new_size);

            let position_value = if direction == Direction::X {
                element.position.0
            } else {
                element.position.1
            };

            if position_value != 0.0 {
                let mut new_pos = element.position.clone();
                let x = amount_to_shrink_all_elements * if i > 1 { i as f64 } else { 1.0 };

                if direction == Direction::X {
                    new_pos.0 -= x;
                } else {
                    new_pos.1 -= x;
                };

                element.goto(new_pos);
            }
        }

        // grow
        // the good news here is this is the same thing as shrinking, just we
        // calculate how many pixels we can expand to, and then add instead of subtract
        // from each element!
        if self.properties.flex_grow && overflowing_pixels == 0.0 {
            // nothing is overflowing, that means there is extra room waiting to be used!
            let mut extra_pixels: f64 = 0.0;

            for element in self.inner.iter() {
                if element.attrs.style == PositionStyle::Absolute {
                    continue;
                }

                let size_value = if direction == Direction::X {
                    element.size.0
                } else {
                    element.size.1
                };

                extra_pixels += size_value;
            }

            extra_pixels = boundary - extra_pixels;
            let amount_to_grow_all_elements = extra_pixels / self.inner.len() as f64;

            for (i, element) in self.inner.iter_mut().enumerate() {
                if element.attrs.style == PositionStyle::Absolute {
                    continue;
                }

                let mut new_size = element.size.clone();

                if direction == Direction::X {
                    new_size.0 += amount_to_grow_all_elements;
                } else {
                    new_size.1 += amount_to_grow_all_elements;
                };

                element.resize(new_size);

                if element.position.0 != 0.0 {
                    let mut new_pos = element.position.clone();
                    let x = amount_to_grow_all_elements * if i > 1 { i as f64 } else { 1.0 };

                    if direction == Direction::X {
                        new_pos.0 += x;
                    } else {
                        new_pos.1 += x;
                    };

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

    pub(crate) fn recalculate_as_flexible(&mut self, basis: Vector2) {
        let mut previous_element: Option<&mut Element> = None;
        let mut previous_on_new_row: Option<Element> = None;
        let mut tallest_of_row: f64 = 0.0;

        let mut cloned = self.inner.clone();

        for element in cloned.iter_mut() {
            if element.attrs.style == PositionStyle::Absolute {
                // should not attempt to move absolute element
                continue;
            }

            let pre = match previous_element {
                Some(ref e) => e,
                None => {
                    // not really anything to adjust since this is the first element
                    previous_element = Some(element);
                    continue;
                }
            };

            let mut new_pos: Vector2 = (
                basis.0 + pre.position.0 + pre.size.0 + self.properties.offset,
                basis.1 + pre.position.1, // we're going to stay on the same y level (unless we overflow)
            );

            if (new_pos.0 + element.size.0 + self.properties.offset) > self.size.0 && self.col {
                // do wrap
                let first_of_row = if let Some(ref e) = previous_on_new_row {
                    // this means that this is not the first time we're going on a new row
                    // this should be a clone of the first element of the current row (that is full)...
                    // this is not a mutable reference, so it isn't to be edited
                    e
                } else {
                    self.inner.get(0).unwrap()
                };

                if tallest_of_row == 0.0 {
                    tallest_of_row = first_of_row.size.1;
                }

                // we're doing tallest height + first.position.1(y) so that we're
                // under the first element AND under the tallest element
                new_pos = (
                    0.0,
                    tallest_of_row + first_of_row.position.1 + self.properties.offset,
                );
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
