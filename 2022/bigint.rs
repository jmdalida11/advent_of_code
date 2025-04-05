use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Rem};
use std::str::FromStr;

const BASE: u64 = 1_000_000_000;
const BASE_DIGITS: usize = 9;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigInt {
    digits: Vec<u32>,
    is_negative: bool,
}

impl BigInt {
    pub fn zero() -> Self {
        Self {
            digits: vec![0],
            is_negative: false,
        }
    }

    pub fn one() -> Self {
        Self {
            digits: vec![1],
            is_negative: false,
        }
    }

    pub fn from_u64(mut n: u64) -> Self {
        if n == 0 {
            return Self::zero();
        }

        let mut digits = Vec::new();
        while n > 0 {
            digits.push((n % BASE) as u32);
            n /= BASE;
        }

        Self {
            digits,
            is_negative: false,
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        let (s, is_negative) = match s.strip_prefix('-') {
            Some(rest) => (rest, true),
            None => (s, false),
        };

        if s.is_empty() {
            return Err("Empty string".to_string());
        }

        let mut digits = Vec::new();
        let mut start = 0;
        let len = s.len();

        while start < len {
            let end = std::cmp::min(start + BASE_DIGITS, len);
            let chunk = &s[start..end];
            let digit = chunk.parse::<u32>()
                .map_err(|_| format!("Invalid digit: {}", chunk))?;

            digits.push(digit);
            start = end;
        }

        digits.reverse();

        while digits.len() > 1 && digits.last() == Some(&0) {
            digits.pop();
        }

        if digits.is_empty() {
            digits.push(0);
        }

        Ok(Self {
            digits,
            is_negative,
        })
    }

    fn add_unsigned(&self, other: &Self) -> Self {
        let mut result = Vec::new();
        let mut carry = 0u64;
        let max_len = std::cmp::max(self.digits.len(), other.digits.len());

        for i in 0..max_len {
            let a = self.digits.get(i).copied().unwrap_or(0) as u64;
            let b = other.digits.get(i).copied().unwrap_or(0) as u64;
            let sum = a + b + carry;
            result.push((sum % BASE) as u32);
            carry = sum / BASE;
        }

        if carry > 0 {
            result.push(carry as u32);
        }

        Self {
            digits: result,
            is_negative: false,
        }
    }

    fn sub_unsigned(&self, other: &Self) -> Self {
        if self < other {
            let mut result = other.sub_unsigned(self);
            result.is_negative = true;
            return result;
        }

        let mut result = Vec::new();
        let mut borrow = 0i64;

        for i in 0..self.digits.len() {
            let a = self.digits[i] as i64;
            let b = other.digits.get(i).copied().unwrap_or(0) as i64;
            let mut diff = a - b - borrow;
            borrow = 0;

            if diff < 0 {
                diff += BASE as i64;
                borrow = 1;
            }

            result.push(diff as u32);
        }

        while result.len() > 1 && result.last() == Some(&0) {
            result.pop();
        }

        Self {
            digits: result,
            is_negative: false,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        match (self.is_negative, other.is_negative) {
            (false, false) => self.add_unsigned(other),
            (false, true) => self.sub_unsigned(other),
            (true, false) => other.sub_unsigned(self),
            (true, true) => {
                let mut result = self.add_unsigned(other);
                result.is_negative = true;
                result
            }
        }
    }

    pub fn sub(&self, other: &Self) -> Self {
        match (self.is_negative, other.is_negative) {
            (false, false) => self.sub_unsigned(other),
            (false, true) => self.add_unsigned(other),
            (true, false) => {
                let mut result = self.add_unsigned(other);
                result.is_negative = true;
                result
            }
            (true, true) => other.sub_unsigned(self),
        }
    }

    pub fn mul(&self, other: &Self) -> Self {
        let mut result = Self::zero();

        for (i, &digit) in other.digits.iter().enumerate() {
            let mut temp = vec![0; i];
            let mut carry = 0u64;

            for &d in &self.digits {
                let product = d as u64 * digit as u64 + carry;
                temp.push((product % BASE) as u32);
                carry = product / BASE;
            }

            if carry > 0 {
                temp.push(carry as u32);
            }

            result = result.add(Self {
                digits: temp,
                is_negative: false,
            });
        }

        result.is_negative = self.is_negative != other.is_negative;
        result
    }

    pub fn modulo(&self, other: &Self) -> Self {
        let mut result = self.clone();
        let abs_other = Self {
            digits: other.digits.clone(),
            is_negative: false,
        };

        while result >= abs_other {
            result = result.sub(abs_other.clone());
        }

        if self.is_negative {
            abs_other.sub(result)
        } else {
            result
        }
    }

    pub fn to_usize(&self) -> Option<usize> {
        if self.is_negative {
            return None;
        }

        let max_usize = usize::MAX as u64;
        let mut result: u64 = 0;

        // Process digits from most significant to least significant
        for &digit in self.digits.iter().rev() {
            // Check if multiplying by 10^9 would overflow
            if let Some(new_result) = result
                .checked_mul(BASE)
                .and_then(|v| v.checked_add(digit as u64))
            {
                result = new_result;
            } else {
                return None;
            }
        }

        if result <= max_usize {
            Some(result as usize)
        } else {
            None
        }
    }
    
    pub fn div(&self, other: &Self) -> Self {
        if other == &Self::zero() {
            panic!("Division by zero");
        }

        let (quotient, _) = self.div_rem(other);
        quotient
    }

    pub fn div_rem(&self, other: &Self) -> (Self, Self) {
        if other == &Self::zero() {
            panic!("Division by zero");
        }

        let dividend = self.abs();
        let divisor = other.abs();
        
        if dividend < divisor {
            return (Self::zero(), dividend.with_sign(self.is_negative));
        }

        let mut quotient_digits = Vec::new();
        let mut remainder = Self::zero();

        for &digit in dividend.digits.iter().rev() {
            remainder = remainder * Self::from_u64(BASE) + Self::from_u64(digit as u64);
            let mut q = 0u32;
            
            while remainder >= divisor {
                remainder = remainder - divisor.clone();
                q += 1;
            }
            
            quotient_digits.push(q);
        }

        quotient_digits.reverse();
        
        while quotient_digits.len() > 1 && quotient_digits.last() == Some(&0) {
            quotient_digits.pop();
        }

        let quotient = Self {
            digits: quotient_digits,
            is_negative: self.is_negative != other.is_negative,
        };

        (quotient, remainder.with_sign(self.is_negative))
    }

    fn abs(&self) -> Self {
        Self {
            digits: self.digits.clone(),
            is_negative: false,
        }
    }

    fn with_sign(&self, negative: bool) -> Self {
        if self == &Self::zero() {
            Self::zero()
        } else {
            Self {
                digits: self.digits.clone(),
                is_negative: negative,
            }
        }
    }
}

impl Div for BigInt {
    type Output = BigInt;
    fn div(self, other: BigInt) -> BigInt {
        BigInt::div(&self, &other)
    }
}

impl<'a> Div for &'a BigInt {
    type Output = BigInt;
    fn div(self, other: &'a BigInt) -> BigInt {
        BigInt::div(self, other)
    }
}

// Operator overloading for owned values
impl Add for BigInt {
    type Output = BigInt;
    fn add(self, other: BigInt) -> BigInt {
        BigInt::add(&self, &other)
    }
}

impl Sub for BigInt {
    type Output = BigInt;
    fn sub(self, other: BigInt) -> BigInt {
        BigInt::sub(&self, &other)
    }
}

impl Mul for BigInt {
    type Output = BigInt;
    fn mul(self, other: BigInt) -> BigInt {
        BigInt::mul(&self, &other)
    }
}

impl Rem for BigInt {
    type Output = BigInt;
    fn rem(self, other: BigInt) -> BigInt {
        BigInt::modulo(&self, &other)
    }
}

// Operator overloading for references
impl<'a> Add for &'a BigInt {
    type Output = BigInt;
    fn add(self, other: &'a BigInt) -> BigInt {
        BigInt::add(self, other)
    }
}

impl<'a> Sub for &'a BigInt {
    type Output = BigInt;
    fn sub(self, other: &'a BigInt) -> BigInt {
        BigInt::sub(self, other)
    }
}

impl<'a> Mul for &'a BigInt {
    type Output = BigInt;
    fn mul(self, other: &'a BigInt) -> BigInt {
        BigInt::mul(self, other)
    }
}

impl<'a> Rem for &'a BigInt {
    type Output = BigInt;
    fn rem(self, other: &'a BigInt) -> BigInt {
        BigInt::modulo(self, other)
    }
}

// Comparison
impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.is_negative, other.is_negative) {
            (false, true) => Ordering::Greater,
            (true, false) => Ordering::Less,
            _ => {
                if self.digits.len() != other.digits.len() {
                    if !self.is_negative {
                        self.digits.len().cmp(&other.digits.len())
                    } else {
                        other.digits.len().cmp(&self.digits.len())
                    }
                } else {
                    for (a, b) in self.digits.iter().rev().zip(other.digits.iter().rev()) {
                        match a.cmp(b) {
                            Ordering::Equal => continue,
                            ord => {
                                if !self.is_negative {
                                    return ord;
                                } else {
                                    return ord.reverse();
                                }
                            }
                        }
                    }
                    Ordering::Equal
                }
            }
        }
    }
}

// Display
impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_negative {
            write!(f, "-")?;
        }

        write!(f, "{}", self.digits.last().unwrap())?;

        for &digit in self.digits.iter().rev().skip(1) {
            write!(f, "{:09}", digit)?;
        }

        Ok(())
    }
}

impl FromStr for BigInt {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BigInt::from_str(s)
    }
}

// // Example usage
// fn main() {
//     let a = BigInt::from_str("12345678901234567890").unwrap();
//     let b = BigInt::from_str("98765432109876543210").unwrap();
    
//     // Using references (recommended)
//     println!("a + b = {}", &a + &b);
//     println!("a - b = {}", &a - &b);
//     println!("a * b = {}", &a * &b);
//     println!("a % 1000 = {}", &a % &BigInt::from_u64(1000));
    
//     // Also works with owned values
//     println!("Alternative: {}", a.clone() + b.clone());
// }