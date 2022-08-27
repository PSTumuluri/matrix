mod f64_impl;

use std::ops::Add;

/// An n-dimensional generic matrix.
pub struct Matrix<T: Clone> {
    /// Holds the data of the matrix in contiguous memory.
    data: Vec<T>,
    
    /// A vector whose elements are the size of each dimension.
    /// The number of dimensions of the matrix is determined by the size of this vector.
    /// This vector informs the indexing mechanism -- that is, it is used to compute the
    /// actual index of an element in the flat data vector from a sequence of 
    /// n-dimensional indices.
    dims: Vec<usize>,
}

impl<T: Clone> Matrix<T> {

    /// Constructs a matrix with the specified dimensions filled with clones of the
    /// specified instance of an object.
    /// The matrix is empty if the number of dimensions provided is 0 or if the size of
    /// any dimension is 0.
    pub fn fill(dims: &[usize], item: T) -> Self {
        let mut dims = Vec::from(dims);
        Self::remove_trailing_size_one_dims(&mut dims);
     
        let size = dims.iter()
            .copied()
            .reduce(|accum, item| accum * item)
            .unwrap_or_else(|| 0);

        let mut data = Vec::with_capacity(size);
        data.resize(size, item);

        Self {
            data,
            dims: if size == 0 { vec![] } else { dims },
        }
    }

    fn remove_trailing_size_one_dims(dims: &mut Vec<usize>) {
        let len = dims.len();
        if len == 0 { return };

        let mut cutoff = 0;
        for _ in dims.iter()
                .rev()
                .take(len - 1)
                .take_while(|item| **item == 1) { 
            cutoff += 1;
        }

        dims.resize(dims.len() - cutoff, 0);
    }
}

impl<T: Clone + Default> Matrix<T> {
    pub fn from_default(dims: &[usize]) -> Self {
        Self::fill(dims, T::default())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn fill_constructs_matrix() {
        let dims = vec![1, 2, 3];
        let mat = Matrix::fill(&dims, 10.0);
        assert_eq!(dims, mat.dims);
        assert_eq!(6, mat.data.len());
    }

    #[test]
    fn fill_removes_trailing_ones() {
        let dims = vec![2, 2, 1, 1];
        let mat = Matrix::fill(&dims, 10.0);
        assert_eq!(vec![2, 2], mat.dims);
        assert_eq!(4, mat.data.len());

        let dims = vec![1, 1, 1];
        let mat = Matrix::fill(&dims, 10.0);
        assert_eq!(vec![1], mat.dims);
        assert_eq!(1, mat.data.len());
    }

    #[test]
    fn fill_with_no_dimensions_constructs_empty_matrix() {
        let dims = vec![];
        let mat = Matrix::fill(&dims, 3.14);
        assert_eq!(dims, mat.dims);
        assert_eq!(0, mat.data.len());
    }

    #[test]
    fn fill_with_0_dimension_constructs_empty_matrix() {
        let dims = vec![1, 2, 3, 4, 5, 6, 0, 8, 9, 10];
        let mat = Matrix::fill(&dims, 1.2);
        assert_eq!(0, mat.dims.len());
    }
}
