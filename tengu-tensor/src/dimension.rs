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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dense_dimension() {
        let dim = Dimension::Dense { size: 10 };
        assert_eq!(dim.size(), 10);
        assert_eq!(dim.positions().collect::<Vec<_>>(), (0..10).collect::<Vec<_>>());
        assert_eq!(dim.indices().collect::<Vec<_>>(), (0..10).collect::<Vec<_>>());
        assert_eq!(dim.values(0, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]).copied().collect::<Vec<_>>(),
                   (0..10).collect::<Vec<_>>());
    }
}