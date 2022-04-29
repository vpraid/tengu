pub enum Dimension {
    Dense { size: usize },
}

impl Dimension {
    pub fn size(&self) -> usize {
        match self {
            Dimension::Dense { size } => *size,
        }
    }
}
