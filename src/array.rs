use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Eq, PartialEq)]
pub struct Array2d<T> {
    width: usize,
    pub data: Vec<T>,
}

impl<T: fmt::Debug> fmt::Debug for Array2d<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Array2d {{")?;
        for row in self.data.chunks_exact(self.width()) {
            writeln!(f, "{:?}", row)?;
        }
        writeln!(f, "}}")
    }
}

impl<T> Array2d<T> {
    pub fn new(width: usize, data: Vec<T>) -> Result<Self, &'static str> {
        match data.len() % width {
            0 => Ok(Self { width, data }),
            _ => Err("length of data and width provided are not compatible"),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.data.len() / self.width()
    }

    pub fn size(&self) -> usize {
        self.width() * self.height()
    }

    // TODO: change implementation when horizontal seam introduced
    pub fn remove_seam(&mut self, seam: Vec<usize>) -> Result<(), &'static str> {
        if seam.len() != self.height() {
            return Err("seam length should be equal to image height");
        }
        let width = self.width();
        seam.iter()
            .enumerate()
            .map(|(row, &col)| row * width + col - row)
            .for_each(|i| {
                self.data.remove(i);
            });
        self.width -= 1;
        Ok(())
    }
}

impl<T> Index<(usize, usize)> for Array2d<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index; // col, row
        &self.data[x + y * self.width]
    }
}

impl<T> IndexMut<(usize, usize)> for Array2d<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;
        &mut self.data[x + y * self.width]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array2d_new() {
        let _arr = Array2d::new(3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
    }

    #[test]
    #[should_panic]
    fn array2d_new_incompatible() {
        let _arr = Array2d::new(4, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
    }

    #[test]
    fn indexing() {
        let arr = Array2d::new(3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
        assert_eq!(1, arr[(0, 0)]);
        assert_eq!(2, arr[(1, 0)]);
        assert_eq!(3, arr[(2, 0)]);
        assert_eq!(4, arr[(0, 1)]);
        assert_eq!(5, arr[(1, 1)]);
        assert_eq!(6, arr[(2, 1)]);
        assert_eq!(7, arr[(0, 2)]);
        assert_eq!(8, arr[(1, 2)]);
        assert_eq!(9, arr[(2, 2)]);
    }

    #[test]
    fn indexing_mut() {
        let mut arr = Array2d::new(3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
        assert_eq!(1, arr[(0, 0)]);
        arr[(0, 0)] = 2;
        assert_eq!(2, arr[(0, 0)]);
    }

    #[test]
    fn seam_removal() {
        let mut arr = Array2d::new(3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
        assert_eq!(
            Array2d::new(3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap(),
            arr
        );
        let seam = vec![1, 2, 1];
        arr.remove_seam(seam).unwrap();
        assert_eq!(
            Array2d::new(2, vec![1, 3, 4, 5, 7, 9]).unwrap(),
            arr
        );
    }
}
