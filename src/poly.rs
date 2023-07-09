use num_traits::{One, Signed, Zero};
use std::borrow::Cow;
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Coeff:
    Sized
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Neg<Output = Self>
    + Eq
    + PartialOrd
    + Copy
    + One
    + Zero
    + Signed
{
}

impl<T> Coeff for T where
    T: Sized
        + Add<Output = Self>
        + AddAssign
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Neg<Output = Self>
        + Eq
        + PartialOrd
        + Copy
        + One
        + Zero
        + Signed
{
}

/// Symbol represents a variable of a polynomial
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sym(pub Cow<'static, str>);

/// Derived term is formed by apply function to another polynomial
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Der<T: Coeff> {
    pub func: Cow<'static, str>,
    pub param: Poly<T>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Base<T: Coeff> {
    Sym(Sym),
    Der(Der<T>),
    Poly(Poly<T>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Factor<T: Coeff> {
    pub base: Base<T>,
    pub power: i32,
}

/// monomial
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mono<T: Coeff> {
    pub coeff: T,
    pub factors: Vec<Factor<T>>,
}

/// polynomial
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Poly<T: Coeff> {
    pub terms: Vec<Mono<T>>,
}

impl<T: Coeff> PartialOrd for Base<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Coeff> Ord for Base<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Self::Sym(sym) => match other {
                Self::Sym(sym2) => sym.cmp(sym2),
                _ => std::cmp::Ordering::Less,
            },
            Self::Der(der) => match other {
                Self::Sym(_) => std::cmp::Ordering::Greater,
                Self::Der(der2) => der.func.cmp(&der2.func),
                Self::Poly(_) => std::cmp::Ordering::Less,
            },
            Self::Poly(poly) => match other {
                Self::Poly(poly2) => poly.cmp(poly2),
                _ => std::cmp::Ordering::Greater,
            },
        }
    }
}

impl<T: Coeff> PartialOrd for Factor<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Coeff> Ord for Factor<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.base.cmp(&other.base)
    }
}

impl<T: Coeff> PartialOrd for Mono<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Coeff> Ord for Mono<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.factors.cmp(&other.factors)
    }
}

impl<T: Coeff> PartialOrd for Poly<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Coeff> Ord for Poly<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.terms.cmp(&other.terms)
    }
}

impl<T: Coeff> Base<T> {
    pub fn is_symbol(&self) -> bool {
        match self {
            Self::Sym(_) => true,
            _ => false,
        }
    }
}

impl<T: Coeff> Factor<T> {
    pub fn is_symbol(&self) -> bool {
        self.power == 1 && self.base.is_symbol()
    }
}

impl<T: Coeff> Mono<T> {
    pub fn is_symbol(&self) -> bool {
        self.factors.len() == 1 && self.factors[0].is_symbol()
    }

    pub fn like(&self, other: &Self) -> bool {
        self.factors == other.factors
    }
}

impl<T: Coeff> Mono<T> {
    pub fn merge_factors(&mut self) {
        let mut i = 0;
        while i < self.factors.len() {
            let mut power = self.factors[i].power;
            let mut j = i + 1;
            while j < self.factors.len() {
                if self.factors[j].base == self.factors[i].base {
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
        self.factors.sort_by(|a, b| a.base.cmp(&b.base));
    }
}

impl<T: Coeff> Neg for Mono<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Mono {
            coeff: self.coeff.neg(),
            factors: self.factors,
        }
    }
}

impl<T: Coeff> Mul for Mono<T> {
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

impl<S: Into<Cow<'static, str>>> From<S> for Sym {
    fn from(value: S) -> Self {
        Sym(value.into())
    }
}

impl<T: Coeff, S: Into<Sym>> From<S> for Base<T> {
    fn from(value: S) -> Self {
        Base::Sym(value.into())
    }
}

impl<T: Coeff> From<&'static str> for Poly<T> {
    fn from(value: &'static str) -> Self {
        Self {
            terms: vec![Mono {
                coeff: T::one(),
                factors: vec![Factor {
                    base: value.into(),
                    power: 1,
                }],
            }],
        }
    }
}

impl<T: Coeff> From<String> for Poly<T> {
    fn from(value: String) -> Self {
        Self {
            terms: vec![Mono {
                coeff: T::one(),
                factors: vec![Factor {
                    base: value.into(),
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

impl<T: Coeff> From<Sym> for Poly<T> {
    fn from(value: Sym) -> Self {
        Self {
            terms: vec![Mono {
                coeff: T::one(),
                factors: vec![Factor {
                    base: value.into(),
                    power: 1,
                }],
            }],
        }
    }
}

impl<T: Coeff> From<Mono<T>> for Poly<T> {
    fn from(value: Mono<T>) -> Self {
        Self { terms: vec![value] }
    }
}

impl<T: Coeff> Poly<T> {
    pub fn apply<S: Into<Cow<'static, str>>>(&self, func: S) -> Poly<T> {
        Self {
            terms: vec![Mono {
                coeff: T::one(),
                factors: vec![Factor {
                    base: Base::Der(Der {
                        func: func.into(),
                        param: self.clone(),
                    }),
                    power: 1,
                }],
            }],
        }
    }

    pub fn is_symbol(&self) -> bool {
        self.terms.len() == 1 && self.terms[0].is_symbol()
    }

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

impl<T: Coeff> Neg for Poly<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Poly {
            terms: self.terms.into_iter().map(|term| term.neg()).collect(),
        }
    }
}

impl<T: Coeff> Add for Poly<T> {
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

impl<T: Coeff> Sub for Poly<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + rhs.neg()
    }
}

impl<T: Coeff> Mul for Poly<T> {
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

impl<T: Coeff> Zero for Poly<T> {
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

impl<T: Coeff> AddAssign for Poly<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl<T: Coeff> SubAssign for Poly<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}

impl<T: Coeff> MulAssign for Poly<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl fmt::Display for Sym {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: fmt::Display + Coeff> fmt::Display for Base<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sym(sym) => write!(f, "{}", sym),
            Self::Der(der) => {
                write!(f, "{}", der.func)?;
                if der.param.is_symbol() {
                    write!(f, "{}", der.param)?;
                } else {
                    write!(f, "({})", der.param)?;
                }
                Ok(())
            }
            Self::Poly(poly) => {
                if poly.is_symbol() {
                    write!(f, "{}", poly)?;
                } else {
                    write!(f, "({})", poly)?;
                }
                Ok(())
            }
        }
    }
}

impl<T: fmt::Display + Coeff> fmt::Display for Factor<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.power == 1 {
            write!(f, "{}", self.base)?;
        } else {
            match &self.base {
                Base::Der(der) => {
                    write!(f, "{}^{}", der.func, self.power)?;
                    if der.param.is_symbol() {
                        write!(f, "{}", der.param)?;
                    } else {
                        write!(f, "({})", der.param)?;
                    }
                }
                _ => {
                    write!(f, "{}^{}", self.base, self.power)?;
                }
            }
        }
        Ok(())
    }
}

impl<T: fmt::Display + Coeff> fmt::Display for Poly<T> {
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
            if !coeff.is_one() || term.factors.is_empty() {
                write!(f, "{}", coeff)?;
            }
            for factor in &term.factors {
                write!(f, "{}", factor)?;
            }
        }
        Ok(())
    }
}
