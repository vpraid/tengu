use super::*;
use anyhow::{ensure, Result};

#[allow(dead_code)]
pub struct Tensor<T> {
    data: Vec<T>,
    dims: Vec<Storage>,
    size: Vec<usize>,
}

// Construction interface

impl<T> Tensor<T> {
    pub fn from(data: impl IntoIterator<Item = T>) -> Builder<T> {
        Builder {
            data: data.into_iter().collect(),
            dims: Vec::new(),
            size: Vec::new(),
        }
    }

    pub fn vector(data: impl IntoIterator<Item = T>) -> Self {
        let data = data.into_iter().collect::<Vec<_>>();
        let size = data.len();
        Self {
            data,
            dims: vec![Storage::Dense],
            size: vec![size],
        }
    }

    pub fn matrix(data: impl IntoIterator<Item = T>, rows: usize, cols: usize) -> Result<Self> {
        let data = data.into_iter().collect::<Vec<T>>();
        ensure!(
            data.len() == rows * cols,
            "Matrix size doesn't fit the data."
        );
        Ok(Self {
            data,
            dims: vec![Storage::Dense, Storage::Dense],
            size: vec![rows, cols],
        })
    }
}

pub struct Builder<T> {
    data: Vec<T>,
    dims: Vec<Storage>,
    size: Vec<usize>,
}

impl<T> Builder<T> {
    pub fn dimension(mut self, storage: Storage, size: usize) -> Self {
        self.dims.push(storage);
        self.size.push(size);
        self
    }

    pub fn build(self) -> Result<Tensor<T>> {
        let total_size = self.size.iter().product::<usize>();
        ensure!(
            self.data.len() == total_size,
            "Tensor size doesn't fit the data."
        );
        Ok(Tensor {
            data: self.data,
            dims: self.dims,
            size: self.size,
        })
    }
}
