pub(crate) struct Array2D<T> {
    data: Vec<T>,
    dim0: usize,
    dim1: usize,
}

impl<T: Clone> Array2D<T> {
    pub(crate) fn new(dim0: usize, dim1: usize, default: T) -> Self {
        let size = dim0 * dim1;
        Self {
            data: vec![default; size],
            dim0,
            dim1,
        }
    }

    fn index(&self, i: usize, j: usize) -> usize {
        debug_assert!(i < self.dim0 && j < self.dim1);
        (i * self.dim1) + j
    }

    pub(crate) fn get(&self, i: usize, j: usize) -> &T {
        &self.data[self.index(i, j)]
    }

    pub(crate) fn get_mut(&mut self, i: usize, j: usize) -> &mut T {
        let idx = self.index(i, j);
        &mut self.data[idx]
    }
}

pub(crate) struct Array3D<T> {
    data: Vec<T>,
    dim0: usize,
    dim1: usize,
    dim2: usize,
}

impl<T: Clone> Array3D<T> {
    pub(crate) fn new(dim0: usize, dim1: usize, dim2: usize, default: T) -> Self {
        let size = dim0 * dim1 * dim2;
        Self {
            data: vec![default; size],
            dim0,
            dim1,
            dim2,
        }
    }

    fn index(&self, i: usize, j: usize, k: usize) -> usize {
        debug_assert!(i < self.dim0 && j < self.dim1 && k < self.dim2);
        ((i * self.dim1 + j) * self.dim2) + k
    }

    pub(crate) fn get(&self, i: usize, j: usize, k: usize) -> &T {
        &self.data[self.index(i, j, k)]
    }

    pub(crate) fn get_mut(&mut self, i: usize, j: usize, k: usize) -> &mut T {
        let idx = self.index(i, j, k);
        &mut self.data[idx]
    }
}

pub(crate) struct Array4D<T> {
    data: Vec<T>,
    dim0: usize,
    dim1: usize,
    dim2: usize,
    dim3: usize,
}

impl<T: Clone> Array4D<T> {
    pub(crate) fn new(dim0: usize, dim1: usize, dim2: usize, dim3: usize, default: T) -> Self {
        let size = dim0 * dim1 * dim2 * dim3;
        Self {
            data: vec![default; size],
            dim0,
            dim1,
            dim2,
            dim3,
        }
    }

    fn index(&self, i: usize, j: usize, k: usize, l: usize) -> usize {
        debug_assert!(i < self.dim0 && j < self.dim1 && k < self.dim2 && l < self.dim3);
        (((i * self.dim1 + j) * self.dim2 + k) * self.dim3) + l
    }

    pub(crate) fn get(&self, i: usize, j: usize, k: usize, l: usize) -> &T {
        &self.data[self.index(i, j, k, l)]
    }

    pub(crate) fn get_mut(&mut self, i: usize, j: usize, k: usize, l: usize) -> &mut T {
        let idx = self.index(i, j, k, l);
        &mut self.data[idx]
    }
}
