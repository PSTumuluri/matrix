use super::Matrix;

impl Matrix<f64> { 

    /// Constructs a matrix with the specified dimensions filled with 0.
    /// Equivalent to Matrix::default() with an explicit f64 generic type annotation.
    pub fn zeros(dims: &[usize]) -> Self {
        Self::fill(dims, 0.0)
    }
    
    /// Constructs a matrix with the specified dimensions filled with 1.
    pub fn ones(dims: &[usize]) -> Self {
        Self::fill(dims, 1.0)
    } 
}
