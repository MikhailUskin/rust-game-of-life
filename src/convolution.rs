use std::cmp;
use nalgebra::base::allocator::Allocator;
use nalgebra::base::default_allocator::DefaultAllocator;
use nalgebra::base::dimension::{Const, Dim, DimAdd, DimDiff, DimSub, DimSum};
use nalgebra::storage::Storage;
use nalgebra::{zero, RealField, U1};
use nalgebra::{OMatrix, Matrix};
use nalgebra::Shape;

impl<T: RealField, R1: Dim, C1: Dim, S1: Storage<T, R1, C1>> Matrix<T, R1, C1, S1> {
    /// Returns the convolution of the target vector and a kernel.
    ///
    /// # Arguments
    ///
    /// * `kernel` - A Vector with size > 0
    ///
    /// # Errors
    /// Inputs must satisfy `vector.len() >= kernel.len() > 0`.
    ///
    pub fn convolve_full_wrap<R2, C2, S2>(
        &self,
        kernel: Matrix<T, R2, C2, S2>,
    ) -> OMatrix<T, R1, C1> 
    where
        D1: DimAdd<D2>,
        D2: DimAdd<D1, Output = DimSum<D1, D2>>, DimSum<D1, D2>: DimSub<U1>,
        S2: Storage<T, D2>,
        DefaultAllocator: Allocator<T, DimDiff<DimSum<D1, D2>, U1>>,
    {
        let matrix_shape = self.shape();
        let kernel_shape = kernel.shape();

        if matrix_shape > kernel_shape {
            panic!("convolve_full_wrap expects `self.shape() > kernel.shape()`, received {} and {} respectively.", matrix_shape, kernel_shape);
        }

        let number_of_rows = self
            .data
            .shape()
            .0
            .add(kernel.shape_generic().0)
            .sub(Const::<1>);

        let number_of_columns = self
            .data
            .shape()
            .1
            .add(kernel.shape_generic().1)
            .sub(Const::<1>);

        let mut conv = OMatrix::zeros(number_of_rows, number_of_columns);
        conv
    }
}
