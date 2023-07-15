use crate::Vector;
use array_init::array_init;
use num_traits::Zero;
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T, const R: usize, const C: usize> {
    pub data: [[T; R]; C],
}

impl<T: Clone, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn row(&self, index: usize) -> Vector<T, C> {
        let data = array_init(|col| self.data[col][index].clone());
        Vector { data }
    }

    pub fn col(&self, index: usize) -> Vector<T, R> {
        let data = array_init(|row| self.data[index][row].clone());
        Vector { data }
    }
}

impl<T: AddAssign, const R: usize, const C: usize> Add for Matrix<T, R, C> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = self.data;
        for (c, col) in rhs.data.into_iter().enumerate() {
            for (r, value) in col.into_iter().enumerate() {
                data[c][r] += value;
            }
        }
        Self { data }
    }
}

impl<T: AddAssign + Clone, const R: usize, const C: usize> Add for &Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = self.data.clone();
        for (c, col) in rhs.data.iter().enumerate() {
            for (r, value) in col.iter().enumerate() {
                data[c][r] += value.clone();
            }
        }
        Matrix { data }
    }
}

impl<T: SubAssign, const R: usize, const C: usize> Sub for Matrix<T, R, C> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = self.data;
        for (c, col) in rhs.data.into_iter().enumerate() {
            for (r, value) in col.into_iter().enumerate() {
                data[c][r] -= value;
            }
        }
        Self { data }
    }
}

impl<T: SubAssign + Clone, const R: usize, const C: usize> Sub for &Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = self.data.clone();
        for (c, col) in rhs.data.iter().enumerate() {
            for (r, value) in col.iter().enumerate() {
                data[c][r] -= value.clone();
            }
        }
        Matrix { data }
    }
}

impl<T: MulAssign + Clone, const R: usize, const C: usize> Mul<T> for Matrix<T, R, C> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut data = self.data;
        for c in 0..C {
            for r in 0..R {
                data[c][r] *= rhs.clone();
            }
        }
        Self { data }
    }
}

impl<T: MulAssign + Clone, const R: usize, const C: usize> Mul<T> for &Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut data = self.data.clone();
        for c in 0..C {
            for r in 0..R {
                data[c][r] *= rhs.clone();
            }
        }
        Matrix { data }
    }
}

impl<T: Neg<Output = T> + Clone, const R: usize, const C: usize> Neg for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn neg(self) -> Self::Output {
        let mut data = self.data;
        for c in 0..C {
            for r in 0..R {
                data[c][r] = -data[c][r].clone();
            }
        }
        Matrix { data }
    }
}

impl<T: Neg<Output = T> + Clone, const R: usize, const C: usize> Neg for &Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn neg(self) -> Self::Output {
        let mut data = self.data.clone();
        for c in 0..C {
            for r in 0..R {
                data[c][r] = -data[c][r].clone();
            }
        }
        Matrix { data }
    }
}

impl<T: Mul<Output = T> + AddAssign + Zero + Clone, const R: usize, const C: usize>
    Mul<Vector<T, C>> for Matrix<T, R, C>
{
    type Output = Vector<T, R>;

    fn mul(self, rhs: Vector<T, C>) -> Self::Output {
        let data = array_init(|r| self.row(r).dot(&rhs));
        Vector { data }
    }
}

impl<T: Mul<Output = T> + AddAssign + Zero + Clone, const R: usize, const C: usize>
    Mul<&Vector<T, C>> for &Matrix<T, R, C>
{
    type Output = Vector<T, R>;

    fn mul(self, rhs: &Vector<T, C>) -> Self::Output {
        let data = array_init(|r| self.row(r).dot(rhs));
        Vector { data }
    }
}

impl<
        T: Mul<Output = T> + AddAssign + Zero + Clone,
        const R: usize,
        const C: usize,
        const D: usize,
    > Mul<Matrix<T, C, D>> for Matrix<T, R, C>
{
    type Output = Matrix<T, R, D>;

    fn mul(self, rhs: Matrix<T, C, D>) -> Self::Output {
        let data = array_init(|c| array_init(|r| self.row(r).dot(&rhs.col(c))));
        Matrix { data }
    }
}

impl<
        T: Mul<Output = T> + AddAssign + Zero + Clone,
        const R: usize,
        const C: usize,
        const D: usize,
    > Mul<&Matrix<T, C, D>> for &Matrix<T, R, C>
{
    type Output = Matrix<T, R, D>;

    fn mul(self, rhs: &Matrix<T, C, D>) -> Self::Output {
        let data = array_init(|c| array_init(|r| self.row(r).dot(&rhs.col(c))));
        Matrix { data }
    }
}

impl<T: fmt::Display, const R: usize, const C: usize> fmt::Display for Matrix<T, R, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for r in 0..R {
            write!(f, "[")?;
            let mut first = true;
            for c in 0..C {
                if first {
                    first = false;
                } else {
                    write!(f, ", ")?;
                }
                write!(f, "{}", &self.data[c][r])?;
            }
            if r < R - 1 {
                writeln!(f, "],")?;
                write!(f, " ")?;
            } else {
                write!(f, "]")?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

/*
 *
 * Constructors for small matrices and vectors.
 *
 */

macro_rules! transpose_array(
    [$($a: ident),*;] => {
        [$([$a]),*]
    };
    [$($a: ident),*; $($b: ident),*;] => {
        [$([$a, $b]),*]
    };
    [$($a: ident),*; $($b: ident),*; $($c: ident),*;] => {
        [$([$a, $b, $c]),*]
    };
    [$($a: ident),*; $($b: ident),*; $($c: ident),*; $($d: ident),*;] => {
        [$([$a, $b, $c, $d]),*]
    };
    [$($a: ident),*; $($b: ident),*; $($c: ident),*; $($d: ident),*; $($e: ident),*;] => {
        [$([$a, $b, $c, $d, $e]),*]
    };
    [$($a: ident),*; $($b: ident),*; $($c: ident),*; $($d: ident),*; $($e: ident),*; $($f: ident),*;] => {
        [$([$a, $b, $c, $d, $e, $f]),*]
    };
);

macro_rules! componentwise_constructors_impl(
    ($($R: expr, $C: expr, [$($($args: ident),*);*] $(;)*)*) => {$(
        impl<T> Matrix<T, $R, $C> {
            /// Initializes this matrix from its components.
            #[inline]
            #[allow(clippy::too_many_arguments)]
            pub const fn new($($($args: T),*),*) -> Self {
                Self {
                    data: transpose_array![
                        $(
                            $($args),*
                        ;)*
                    ]
                }
            }
        }
    )*}
);

componentwise_constructors_impl!(
    /*
     * Square matrices 1 .. 6.
     */
    2, 2, [m11, m12;
           m21, m22];
    3, 3, [m11, m12, m13;
          m21, m22, m23;
          m31, m32, m33];
    4, 4, [m11, m12, m13, m14;
          m21, m22, m23, m24;
          m31, m32, m33, m34;
          m41, m42, m43, m44];
    5, 5, [m11, m12, m13, m14, m15;
          m21, m22, m23, m24, m25;
          m31, m32, m33, m34, m35;
          m41, m42, m43, m44, m45;
          m51, m52, m53, m54, m55];
    6, 6, [m11, m12, m13, m14, m15, m16;
          m21, m22, m23, m24, m25, m26;
          m31, m32, m33, m34, m35, m36;
          m41, m42, m43, m44, m45, m46;
          m51, m52, m53, m54, m55, m56;
          m61, m62, m63, m64, m65, m66];

    /*
     * Rectangular matrices with 2 rows.
     */
    2, 3, [m11, m12, m13;
          m21, m22, m23];
    2, 4, [m11, m12, m13, m14;
          m21, m22, m23, m24];
    2, 5, [m11, m12, m13, m14, m15;
          m21, m22, m23, m24, m25];
    2, 6, [m11, m12, m13, m14, m15, m16;
          m21, m22, m23, m24, m25, m26];

    /*
     * Rectangular matrices with 3 rows.
     */
    3, 2, [m11, m12;
          m21, m22;
          m31, m32];
    3, 4, [m11, m12, m13, m14;
          m21, m22, m23, m24;
          m31, m32, m33, m34];
    3, 5, [m11, m12, m13, m14, m15;
          m21, m22, m23, m24, m25;
          m31, m32, m33, m34, m35];
    3, 6, [m11, m12, m13, m14, m15, m16;
          m21, m22, m23, m24, m25, m26;
          m31, m32, m33, m34, m35, m36];

    /*
     * Rectangular matrices with 4 rows.
     */
    4, 2, [m11, m12;
          m21, m22;
          m31, m32;
          m41, m42];
    4, 3, [m11, m12, m13;
          m21, m22, m23;
          m31, m32, m33;
          m41, m42, m43];
    4, 5, [m11, m12, m13, m14, m15;
          m21, m22, m23, m24, m25;
          m31, m32, m33, m34, m35;
          m41, m42, m43, m44, m45];
    4, 6, [m11, m12, m13, m14, m15, m16;
          m21, m22, m23, m24, m25, m26;
          m31, m32, m33, m34, m35, m36;
          m41, m42, m43, m44, m45, m46];

    /*
     * Rectangular matrices with 5 rows.
     */
    5, 2, [m11, m12;
          m21, m22;
          m31, m32;
          m41, m42;
          m51, m52];
    5, 3, [m11, m12, m13;
          m21, m22, m23;
          m31, m32, m33;
          m41, m42, m43;
          m51, m52, m53];
    5, 4, [m11, m12, m13, m14;
          m21, m22, m23, m24;
          m31, m32, m33, m34;
          m41, m42, m43, m44;
          m51, m52, m53, m54];
    5, 6, [m11, m12, m13, m14, m15, m16;
          m21, m22, m23, m24, m25, m26;
          m31, m32, m33, m34, m35, m36;
          m41, m42, m43, m44, m45, m46;
          m51, m52, m53, m54, m55, m56];

    /*
     * Rectangular matrices with 6 rows.
     */
    6, 2, [m11, m12;
          m21, m22;
          m31, m32;
          m41, m42;
          m51, m52;
          m61, m62];
    6, 3, [m11, m12, m13;
          m21, m22, m23;
          m31, m32, m33;
          m41, m42, m43;
          m51, m52, m53;
          m61, m62, m63];
    6, 4, [m11, m12, m13, m14;
          m21, m22, m23, m24;
          m31, m32, m33, m34;
          m41, m42, m43, m44;
          m51, m52, m53, m54;
          m61, m62, m63, m64];
    6, 5, [m11, m12, m13, m14, m15;
          m21, m22, m23, m24, m25;
          m31, m32, m33, m34, m35;
          m41, m42, m43, m44, m45;
          m51, m52, m53, m54, m55;
          m61, m62, m63, m64, m65];

    /*
     * Row vectors 1 .. 6.
     */
    1, 1, [x];
    1, 2, [x, y];
    1, 3, [x, y, z];
    1, 4, [x, y, z, w];
    1, 5, [x, y, z, w, a];
    1, 6, [x, y, z, w, a, b];

    /*
     * Column vectors 1 .. 6.
     */
    2, 1, [x; y];
    3, 1, [x; y; z];
    4, 1, [x; y; z; w];
    5, 1, [x; y; z; w; a];
    6, 1, [x; y; z; w; a; b];
);
