use super::*;
use anyhow::{ensure, Result};

#[allow(dead_code)]
pub struct Tensor<T> {
    data: Vec<T>,
    dims: Vec<Dimension>,
}

// Construction interface

impl<T> Tensor<T> {
    pub fn from(data: impl IntoIterator<Item = T>) -> Builder<T> {
        Builder {
            data: data.into_iter().collect(),
            dims: Vec::new(),
        }
    }
}

pub struct Builder<T> {
    data: Vec<T>,
    dims: Vec<Dimension>,
}

impl<T> Builder<T> {
    pub fn dense(mut self, size: usize) -> Self {
        self.dims.push(Dimension::Dense { size });
        self
    }

    pub fn build(self) -> Result<Tensor<T>> {
        let total_size = self.dims.iter().map(|d| d.size()).product::<usize>();
        ensure!(
            self.data.len() == total_size,
            "Calculated tensor size doesn't match the data."
        );
        Ok(Tensor {
            data: self.data,
            dims: self.dims,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dense_vector() {
        let _tensor = Tensor::from(vec![1, 2, 3]).dense(3).build();
    }

    #[test]
    fn dense_matrix() {
        let _tensor = Tensor::from(vec![1, 2, 3, 4, 5, 6]).dense(2).dense(3).build();
    }
}
