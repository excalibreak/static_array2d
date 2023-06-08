//! static_array2d provides an interface for 2d slices.
//!
//! It differs from `array2d` and `array2ds` by using the stack
//! instead of the heap through rust's constant generics.

#![no_std]
/// A statically sized 2-dimensional array
#[derive(Clone)]
pub struct Grid<const W: usize, const H: usize, T> {
    pub data: [[T; W]; H],
}

/// A statically sized 2-dimensional array with equal width and height
pub type SquareGrid<const W: usize, T> = Grid<W, W, T>;

impl<const W: usize, const H: usize, T: Default + Copy> Default for Grid<W, H, T> {
    fn default() -> Self {
        Self {
            data: [[T::default(); W]; H],
        }
    }
}

impl<const W: usize, const H: usize, T> From<[[T; W]; H]> for Grid<W, H, T> {
    fn from(value: [[T; W]; H]) -> Self {
        Self { data: value }
    }
}

impl<const W: usize, const H: usize, T> Grid<W, H, T> {
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.data
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, item)| (x, y, item)))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut T)> {
        self.data.iter_mut().enumerate().flat_map(|(y, row)| {
            row.iter_mut()
                .enumerate()
                .map(move |(x, item)| (x, y, item))
        })
    }

    #[must_use]
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(y)?.get(x)
    }

    #[must_use]
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.data.get_mut(y)?.get_mut(x)
    }

    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// [`get`]: slice::get
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    #[must_use]
    pub unsafe fn get_unchecked(&self, x: usize, y: usize) -> &T {
        self.data.get_unchecked(y).get_unchecked(x)
    }

    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// [`get`]: slice::get
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    #[must_use]
    pub unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut T {
        self.data.get_unchecked_mut(y).get_unchecked_mut(x)
    }
}
