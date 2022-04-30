pub enum Dimension {
    Dense { size: usize },
}

impl Dimension {
    /// Returns the size of the dimension. More precisely, dimension size represents
    /// the number of indices in the dimension.
    pub fn size(&self) -> usize {
        match self {
            Dimension::Dense { size } => *size,
        }
    }

    /// Returns positions of nonzero coordinates in this dimension.
    /// For dense dimensions, positions coincide with indices.
    pub fn positions(&self) -> impl Iterator<Item = usize> {
        match self {
            Dimension::Dense { size } => (0..*size)
        }
    }

    /// Returns indices of nonzero coordinates in this dimension.
    pub fn indices(&self) -> impl Iterator<Item = usize> {
        match self {
            Dimension::Dense { size } => (0..*size)
        }
    }

    /// Returns the iterator over the values of the data when this dimensions is the last one
    /// in the chain.
    pub fn values<'a, T>(&self, start: usize, data: &'a [T]) -> impl Iterator<Item = &'a T> {
        match self {
            Dimension::Dense { size } => data[start..start + *size].iter()
        }
    }
}
