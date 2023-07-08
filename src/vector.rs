use num_traits::Zero;
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub type Vector3<T> = Vector<T, 3>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T, const D: usize> {
    data: [T; D],
}

impl<T: AddAssign, const D: usize> Add for Vector<T, D> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = self.data;
        for (i, value) in rhs.data.into_iter().enumerate() {
            data[i] += value;
        }
        Self { data }
    }
}

impl<T: AddAssign + Clone, const D: usize> Add for &Vector<T, D> {
    type Output = Vector<T, D>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = self.data.clone();
        for (i, value) in rhs.data.iter().enumerate() {
            data[i] += value.clone();
        }
        Vector { data }
    }
}

impl<T: SubAssign, const D: usize> Sub for Vector<T, D> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = self.data;
        for (i, value) in rhs.data.into_iter().enumerate() {
            data[i] -= value;
        }
        Self { data }
    }
}

impl<T: SubAssign + Clone, const D: usize> Sub for &Vector<T, D> {
    type Output = Vector<T, D>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = self.data.clone();
        for (i, value) in rhs.data.iter().enumerate() {
            data[i] -= value.clone();
        }
        Vector { data }
    }
}

impl<T: MulAssign + Clone, const D: usize> Mul<T> for Vector<T, D> {
    type Output = Vector<T, D>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut data = self.data;
        for i in 0..D {
            data[i] *= rhs.clone();
        }
        Vector { data }
    }
}

impl<T: MulAssign + Clone, const D: usize> Mul<T> for &Vector<T, D> {
    type Output = Vector<T, D>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut data = self.data.clone();
        for i in 0..D {
            data[i] *= rhs.clone();
        }
        Vector { data }
    }
}

impl<T: Neg<Output = T> + Clone, const D: usize> Neg for Vector<T, D> {
    type Output = Vector<T, D>;

    fn neg(self) -> Self::Output {
        let mut data = self.data;
        for i in 0..D {
            data[i] = -data[i].clone();
        }
        Vector { data }
    }
}

impl<T: Neg<Output = T> + Zero + Clone, const D: usize> Neg for &Vector<T, D> {
    type Output = Vector<T, D>;

    fn neg(self) -> Self::Output {
        let mut data = self.data.clone();
        for i in 0..D {
            data[i] = -data[i].clone();
        }
        Vector { data }
    }
}

impl<T: Mul<Output = T> + AddAssign + Zero + Clone, const D: usize> Vector<T, D> {
    pub fn dot(&self, other: &Vector<T, D>) -> T {
        let mut sum = T::zero();

        for (a, b) in self.data.iter().zip(other.data.iter()) {
            sum += a.clone() * b.clone();
        }

        sum
    }
}

impl<T: Clone> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector { data: [x, y, z] }
    }
    pub fn x(&self) -> T {
        self.data[0].clone()
    }
    pub fn y(&self) -> T {
        self.data[1].clone()
    }
    pub fn z(&self) -> T {
        self.data[2].clone()
    }
}

impl<T: Mul<Output = T> + Sub<Output = T> + Clone> Vector3<T> {
    pub fn cross(&self, rhs: &Vector3<T>) -> Vector3<T> {
        Self {
            data: [
                self.y() * rhs.z() - rhs.y() * self.z(),
                self.z() * rhs.x() - rhs.z() * self.x(),
                self.x() * rhs.y() - rhs.x() * self.y(),
            ],
        }
    }
}

impl<T: fmt::Display, const D: usize> fmt::Display for Vector<T, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        let mut first = true;
        for value in &self.data {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }
            write!(f, "{}", value)?;
        }
        write!(f, ")")?;
        Ok(())
    }
}
