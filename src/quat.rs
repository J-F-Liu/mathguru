use crate::{Matrix, Vector};
use num_traits::Zero;
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// quaternion
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quat<T>(Vector<T, 4>);

impl<T: Clone> Quat<T> {
    pub fn new(q0: T, q1: T, q2: T, q3: T) -> Self {
        Self(Vector::<T, 4>::new(q0, q1, q2, q3))
    }

    pub fn as_vector(&self) -> &Vector<T, 4> {
        &self.0
    }

    pub fn q0(&self) -> T {
        self.0.data[0].clone()
    }

    pub fn q1(&self) -> T {
        self.0.data[1].clone()
    }

    pub fn q2(&self) -> T {
        self.0.data[2].clone()
    }

    pub fn q3(&self) -> T {
        self.0.data[3].clone()
    }
}

impl<T: Clone + Neg<Output = T>> Quat<T> {
    #[rustfmt::skip]
    pub fn left_mul_matrix(&self) -> Matrix<T, 4, 4> {
        Matrix::<T, 4, 4>::new(
            self.q0(), -self.q1(), -self.q2(), -self.q3(),
            self.q1(), self.q0(), -self.q3(), self.q2(),
            self.q2(), self.q3(), self.q0(), -self.q1(),
            self.q3(), -self.q2(), self.q1(), self.q0(),
        )
    }

    #[rustfmt::skip]
    pub fn right_mul_matrix(&self) -> Matrix<T, 4, 4> {
        Matrix::<T, 4, 4>::new(
            self.q0(), -self.q1(), -self.q2(), -self.q3(),
            self.q1(), self.q0(), self.q3(),-self.q2(),
            self.q2(), -self.q3(), self.q0(), self.q1(),
            self.q3(), self.q2(), -self.q1(), self.q0(),
        )
    }
}

impl<T: AddAssign> Add for Quat<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Quat(self.0 + rhs.0)
    }
}

impl<T: AddAssign + Clone> Add for &Quat<T> {
    type Output = Quat<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Quat(&self.0 + &rhs.0)
    }
}

impl<T: SubAssign> Sub for Quat<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Quat(self.0 - rhs.0)
    }
}

impl<T: SubAssign + Clone> Sub for &Quat<T> {
    type Output = Quat<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Quat(&self.0 - &rhs.0)
    }
}

impl<T: MulAssign + Clone> Mul<T> for Quat<T> {
    type Output = Quat<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Quat(self.0 * rhs)
    }
}

impl<T: MulAssign + Clone> Mul<T> for &Quat<T> {
    type Output = Quat<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Quat(&self.0 * rhs)
    }
}

impl<T: Mul<Output = T> + Neg<Output = T> + AddAssign + Zero + Clone> Mul<Self> for Quat<T> {
    type Output = Quat<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Quat(self.left_mul_matrix() * rhs.0)
    }
}

impl<T: Mul<Output = T> + Neg<Output = T> + AddAssign + Zero + Clone> Mul<Self> for &Quat<T> {
    type Output = Quat<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Quat(&self.left_mul_matrix() * &rhs.0)
    }
}

impl<T: Neg<Output = T> + Clone> Neg for Quat<T> {
    type Output = Quat<T>;

    fn neg(self) -> Self::Output {
        Quat(-self.0)
    }
}

impl<T: Neg<Output = T> + Clone> Neg for &Quat<T> {
    type Output = Quat<T>;

    fn neg(self) -> Self::Output {
        Quat(-&self.0)
    }
}

impl<T: fmt::Display> fmt::Display for Quat<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Quat")?;
        write!(f, "{}", self.0)?;
        Ok(())
    }
}
