use num_traits::{One, Signed, Zero};
use std::borrow::Cow;
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sym(pub Cow<'static, str>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Factor {
    pub symbol: Sym,
    pub power: i32,
}

/// monomial
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mono<T> {
    pub coeff: T,
    pub factors: Vec<Factor>,
}

/// polynomial
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Poly<T> {
    pub terms: Vec<Mono<T>>,
}

impl<T> Mono<T> {
    pub fn like(&self, other: &Self) -> bool {
        self.factors == other.factors
    }

    pub fn merge_factors(&mut self) {
        let mut i = 0;
        while i < self.factors.len() {
            let mut power = self.factors[i].power;
            let mut j = i + 1;
            while j < self.factors.len() {
                if self.factors[j].symbol == self.factors[i].symbol {
                    let factor = self.factors.swap_remove(j);
                    power += factor.power;
                } else {
                    j += 1;
                }
            }
            if power == 0 {
                self.factors.swap_remove(i);
            } else {
                self.factors[i].power = power;
                i += 1;
            }
        }
        self.factors.sort_by(|a, b| a.symbol.cmp(&b.symbol));
    }
}

impl<T: Neg<Output = T>> Neg for Mono<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Mono {
            coeff: self.coeff.neg(),
            factors: self.factors,
        }
    }
}

impl<T: Mul<Output = T> + AddAssign + Copy> Mul for Mono<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let coeff = self.coeff * rhs.coeff;

        let mut factors = Vec::with_capacity(self.factors.len() + rhs.factors.len());
        factors.extend(self.factors);
        factors.extend(rhs.factors);

        let mut result = Mono { coeff, factors };
        result.merge_factors();
        result
    }
}

impl<T: One> From<&'static str> for Poly<T> {
    fn from(value: &'static str) -> Self {
        Self {
            terms: vec![Mono {
                coeff: T::one(),
                factors: vec![Factor {
                    symbol: Sym(value.into()),
                    power: 1,
                }],
            }],
        }
    }
}

impl From<i32> for Poly<i32> {
    fn from(value: i32) -> Self {
        Self {
            terms: vec![Mono {
                coeff: value,
                factors: vec![],
            }],
        }
    }
}

impl<T: One> From<Sym> for Poly<T> {
    fn from(value: Sym) -> Self {
        Self {
            terms: vec![Mono {
                coeff: T::one(),
                factors: vec![Factor {
                    symbol: value,
                    power: 1,
                }],
            }],
        }
    }
}

impl<T> From<Mono<T>> for Poly<T> {
    fn from(value: Mono<T>) -> Self {
        Self { terms: vec![value] }
    }
}

impl<T: Zero + AddAssign + Copy> Poly<T> {
    pub fn merge_terms(&mut self) {
        let mut i = 0;
        while i < self.terms.len() {
            let mut coeff = self.terms[i].coeff;
            let mut j = i + 1;
            while j < self.terms.len() {
                if self.terms[j].like(&self.terms[i]) {
                    let term = self.terms.swap_remove(j);
                    coeff += term.coeff;
                } else {
                    j += 1;
                }
            }
            if coeff.is_zero() {
                self.terms.swap_remove(i);
            } else {
                self.terms[i].coeff = coeff;
                i += 1;
            }
        }
        self.terms.sort_by(|a, b| a.factors.cmp(&b.factors));
    }
}

impl<T: Neg<Output = T>> Neg for Poly<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Poly {
            terms: self.terms.into_iter().map(|term| term.neg()).collect(),
        }
    }
}

impl<T: AddAssign + Zero + Copy> Add for Poly<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut terms = Vec::with_capacity(self.terms.len() + rhs.terms.len());
        terms.extend(self.terms);
        terms.extend(rhs.terms);
        let mut result = Poly { terms };
        result.merge_terms();
        result
    }
}

impl<T: AddAssign + Neg<Output = T> + Zero + Copy> Sub for Poly<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + rhs.neg()
    }
}

impl<T: Mul<Output = T> + AddAssign + Zero + Copy> Mul for Poly<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut terms = Vec::with_capacity(self.terms.len() * rhs.terms.len());
        for term in &self.terms {
            for other in &rhs.terms {
                terms.push(term.clone() * other.clone());
            }
        }
        let mut result = Poly { terms };
        result.merge_terms();
        result
    }
}

impl<T: AddAssign + Zero + Copy> Zero for Poly<T> {
    fn zero() -> Self {
        Self { terms: vec![] }
    }

    fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }

    fn set_zero(&mut self) {
        self.terms.clear()
    }
}

impl<T: AddAssign + Zero + Copy> AddAssign for Poly<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl<T: Neg<Output = T> + AddAssign + Zero + Copy> SubAssign for Poly<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}

impl<T: Mul<Output = T> + AddAssign + Zero + Copy> MulAssign for Poly<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl fmt::Display for Sym {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: fmt::Display + Signed + One + PartialEq> fmt::Display for Poly<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for term in &self.terms {
            if first {
                first = false;
                if term.coeff.is_negative() {
                    write!(f, " - ")?;
                }
            } else {
                if term.coeff.is_positive() {
                    write!(f, " + ")?;
                } else {
                    write!(f, " - ")?;
                }
            }
            let coeff = term.coeff.abs();
            if !coeff.is_one() {
                write!(f, "{}", coeff)?;
            }
            for factor in &term.factors {
                if factor.power == 1 {
                    write!(f, "{}", factor.symbol)?;
                } else {
                    write!(f, "{}^{}", factor.symbol, factor.power)?;
                }
            }
        }
        Ok(())
    }
}
