use super::Matrix;

impl Matrix<f64> {

    pub fn add(&self, other: &Self) -> Result<Self, &'static str> {
        if self.dims != other.dims {
            return Err("cannot add two matrices with different shapes");
        }

        Ok(Self {
            data: self.data.iter()
                .zip(other.data.iter())
                .map(|x| x.0 + x.1)
                .collect::<Vec<f64>>(),
            dims: self.dims.clone(),
        })
    }

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
