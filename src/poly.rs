use num_traits::{One, Signed, Zero};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::{fmt, vec};

pub trait Coeff:
    Sized
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Neg<Output = Self>
    + Hash
    + Eq
    + PartialOrd
    + Copy
    + One
    + Zero
    + Signed
    + fmt::Display
{
}

impl<T> Coeff for T where
    T: Sized
        + Add<Output = Self>
        + AddAssign
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Neg<Output = Self>
        + Hash
        + Eq
        + PartialOrd
        + Copy
        + One
        + Zero
        + Signed
        + fmt::Display
{
}

/// Symbol represents a variable of a polynomial
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sym(pub Cow<'static, str>);

/// Derived term is formed by apply function to another polynomial
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Der<T: Coeff> {
    pub func: Cow<'static, str>,
    pub param: Poly<T>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Base<T: Coeff> {
    Sym(Sym),
    Der(Der<T>),
    Poly(Poly<T>),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Factor<T: Coeff> {
    pub base: Base<T>,
    pub power: i32,
}

/// monomial
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Mono<T: Coeff> {
    pub coeff: T,
    pub factors: Vec<Factor<T>>,
}

/// polynomial
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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
                Self::Der(der2) => der.func.cmp(&der2.func).then(der.param.cmp(&der2.param)),
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
        self.base
            .cmp(&other.base)
            .then(self.power.cmp(&other.power))
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
    pub fn is_polynomial(&self) -> bool {
        match self {
            Self::Poly(_) => true,
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

    pub fn power_of(&self, base: &Base<T>) -> i32 {
        for factor in &self.factors {
            if &factor.base == base {
                return factor.power;
            }
        }
        0
    }

    /// If the monomial contains given factor, return remained part after extract the factor.
    pub fn extract(&self, factor: &Factor<T>) -> Option<Mono<T>> {
        for (index, fact) in self.factors.iter().enumerate() {
            if fact.base == factor.base {
                if fact.power >= factor.power {
                    let mut factors = self.factors.clone();
                    factors[index].power = fact.power - factor.power;
                    if factors[index].power == 0 {
                        factors.remove(index);
                    }
                    return Some(Mono {
                        coeff: self.coeff,
                        factors,
                    });
                } else {
                    break;
                }
            }
        }
        None
    }

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

    pub fn group_by(self, bases: &[Base<T>]) -> (Mono<T>, Vec<Factor<T>>) {
        let (factors, group): (Vec<_>, Vec<_>) = self
            .factors
            .into_iter()
            .partition(|factor| bases.contains(&factor.base));
        (
            Mono {
                coeff: self.coeff,
                factors: group,
            },
            factors,
        )
    }

    pub fn contains_polynomial(&self) -> bool {
        self.factors
            .iter()
            .any(|factor| factor.base.is_polynomial())
    }

    pub fn expand(self) -> Poly<T> {
        let mut polynomials = vec![];
        let mut factors = vec![];
        for factor in self.factors {
            match factor.base {
                // Todo: expand polynomial with power greater than 1
                Base::Poly(poly) => polynomials.push(poly),
                _ => factors.push(factor),
            }
        }
        let mono = Poly {
            terms: vec![Mono {
                coeff: self.coeff,
                factors,
            }],
        };
        if let Some(mut poly) = polynomials.pop() {
            while let Some(poly2) = polynomials.pop() {
                poly = poly * poly2;
            }
            poly.expand();
            poly = poly * mono;
            poly
        } else {
            mono
        }
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

    pub fn group_by(&mut self, bases: Vec<Base<T>>) {
        let terms = std::mem::replace(&mut self.terms, vec![]);
        let items = terms.into_iter().map(|mono| mono.group_by(&bases)).fold(
            BTreeMap::new(),
            |mut acc, (term, factors)| {
                acc.entry(factors).or_insert(vec![]).push(term);
                acc
            },
        );
        self.terms = items
            .into_iter()
            .filter_map(|(mut factors, mut group)| {
                // println!(
                //     "{}",
                //     Poly::from(Mono {
                //         coeff: T::one(),
                //         factors: factors.clone(),
                //     })
                // );
                if group.len() == 1 {
                    let mut mono = group.remove(0);
                    mono.factors.extend(factors);
                    Some(mono)
                } else {
                    let mut poly = Poly { terms: group };
                    poly.merge_terms();
                    if poly.is_zero() {
                        None
                    } else {
                        factors.insert(
                            0,
                            Factor {
                                base: Base::Poly(poly),
                                power: 1,
                            },
                        );
                        Some(Mono {
                            coeff: T::one(),
                            factors,
                        })
                    }
                }
            })
            .collect();
    }

    pub fn collect_by(&self, factors: &[Factor<T>]) -> (Vec<Poly<T>>, Poly<T>) {
        let mut collected_terms = vec![vec![]; factors.len()];
        let mut remained_terms = vec![];
        for term in &self.terms {
            let mut collected = false;
            for (index, factor) in factors.iter().enumerate() {
                if let Some(mono) = term.extract(factor) {
                    collected_terms[index].push(mono);
                    collected = true;
                    break;
                }
            }
            if !collected {
                remained_terms.push(term.clone());
            }
        }
        (
            collected_terms
                .into_iter()
                .map(|terms| Poly { terms })
                .collect(),
            Poly {
                terms: remained_terms,
            },
        )
    }

    pub fn simplify_by_identity(&self, left_side: Poly<T>, right_side: Poly<T>) -> Poly<T> {
        let factors = left_side
            .clone()
            .terms
            .into_iter()
            // assume only one factor in each term, and coeff of the factor is 1.
            .map(|mut term| term.factors.remove(0))
            .collect::<Vec<_>>();
        let right_term = right_side.clone().terms.remove(0);
        let (mut collected_terms, remained_terms) = self.collect_by(&factors);
        let mut simlified_terms = vec![];
        let mut common_count = 0;
        let mut index = 0;
        while index < collected_terms[0].terms.len() {
            // find common term
            let curr = &collected_terms[0].terms[index];
            let positions = collected_terms[1..]
                .iter()
                .filter_map(|poly| poly.terms.iter().position(|term| term == curr))
                .collect::<Vec<_>>();
            if positions.len() == collected_terms.len() - 1 {
                common_count += 1;
                let term = collected_terms[0].terms.remove(index);
                // println!("Common: {}", Poly::from(term.clone()));
                for (pos, poly) in positions
                    .into_iter()
                    .zip(collected_terms.iter_mut().skip(1))
                {
                    poly.terms.remove(pos);
                }
                simlified_terms.push(term * right_term.clone());
            } else {
                index += 1;
            }
        }
        // simplify nested polynomails
        if common_count > 0 {
            for poly in &mut collected_terms {
                *poly = poly.simplify_by_identity(left_side.clone(), right_side.clone());
            }
        }
        let mut final_terms = vec![];
        for (factor, poly) in factors.into_iter().zip(collected_terms.into_iter()) {
            if poly.terms.len() > 0 {
                final_terms.push(Mono {
                    coeff: T::one(),
                    factors: vec![
                        Factor {
                            base: Base::Poly(poly),
                            power: 1,
                        },
                        factor,
                    ],
                })
            }
        }
        final_terms.extend(simlified_terms);
        final_terms.extend(remained_terms.terms);
        Poly { terms: final_terms }
    }

    pub fn expand(&mut self) {
        let capacity = self.terms.len();
        let terms = std::mem::replace(&mut self.terms, Vec::with_capacity(capacity));
        for term in terms {
            if term.contains_polynomial() {
                self.terms.extend(term.expand().terms);
            } else {
                self.terms.push(term);
            }
        }
    }

    pub fn extract_common_factors(&mut self) -> Vec<Factor<T>> {
        if self.is_zero() {
            return vec![];
        }
        let mut common_factors = vec![];
        let mut index = 0;
        while index < self.terms[0].factors.len() {
            // find common factor
            let base = &self.terms[0].factors[index].base;
            let mut min_power = self.terms[0].factors[index].power;
            for term in &self.terms[1..] {
                min_power = min_power.min(term.power_of(base));
                if min_power == 0 {
                    break;
                }
            }
            if min_power > 0 {
                let factor = Factor {
                    base: base.clone(),
                    power: min_power,
                };
                for term in self.terms.iter_mut() {
                    *term = term.extract(&factor).unwrap();
                }
                common_factors.push(factor);
            }
            index += 1;
        }
        common_factors
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

impl<T: Coeff> fmt::Display for Base<T> {
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

impl<T: Coeff> fmt::Display for Factor<T> {
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

impl<T: Coeff> fmt::Display for Poly<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for term in &self.terms {
            if first {
                first = false;
                if term.coeff.is_negative() {
                    write!(f, "- ")?;
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
