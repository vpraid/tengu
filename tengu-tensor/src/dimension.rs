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
            Dimension::Dense { size } => {
                let offset = start * size;
                data[offset..offset+size].iter()
            }
        }
    }

    /// Returns the position of the coordinate in this dimension if it is nonzero, None otherwise.
    /// This operation essentially flattens the current dimension. For example, consider a 2 by 3
    /// matrix with both dimensions dense. If we need to get the position of the coordinate (1, 2),
    /// we need 3 elements from row 0 and 2 from row 1 to get the coordinate position, which will
    /// be 5.
    pub fn at(&self, start: usize, index: usize) -> Option<usize> {
        match self {
            Dimension::Dense { size } => Some(start * size + index)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dense_dimension() {
        let dim = Dimension::Dense { size: 3 };
        assert_eq!(dim.size(), 3);
        assert_eq!(dim.positions().collect::<Vec<_>>(), (0..3).collect::<Vec<_>>());
        assert_eq!(dim.indices().collect::<Vec<_>>(), (0..3).collect::<Vec<_>>());
        assert_eq!(dim.values(1, &[0, 1, 2, 3, 4, 5]).copied().collect::<Vec<_>>(), vec![3, 4, 5]);
        assert_eq!(dim.at(1, 2), Some(5));
    }
}