use std::fmt;
use std::ops::{Add, Div, Rem, Mul};
use std::iter::Sum;
use std::cmp::Ordering;
use std::hash::Hash;

pub trait ValueTraits<T=Self>
    : Div<Output=T>
    + Add<Output=T>
    + Rem<Output=T>
    + Mul<Output=T>
    + PartialOrd + Copy + PartialEq + Hash + fmt::Display + fmt::Debug + From<u8>

// where T: Div<Output=T> + Rem<Output=T> + Mul<Output=T> + PartialOrd + Copy + PartialEq + Hash + fmt::Display
{}

impl<T> ValueTraits<T> for T where T
    : Div<Output=T>
    + Add<Output=T>
    + Rem<Output=T>
    + Mul<Output=T>
    + PartialOrd + Copy + PartialEq + Hash + fmt::Display + fmt::Debug + From<u8>
{}


#[derive(Hash, Copy, Clone)]
pub struct Ratio<T>
where T: ValueTraits<T>
{
    pub numeritor: T,
    pub denominator: T,
}

// Euler’s method. 也就是辗转相除法？
pub fn gcd<T>(up: T, down: T) -> T
where T: ValueTraits<T>
{
    let mut r1 = up;
    let mut r2 = down;

    if r1 < r2
    {
        let temp = r1;
        r1 = r2;
        r2 = temp;
    }

    let zero: T = T::from(0u8);
    loop
    {
        r1 = r1 % r2;
        if r1 == zero
        {
            break r2;
        }
        r2 = r2 % r1;
        if r2 == zero
        {
            break r1;
        }
    }
}

pub fn lcm<T>(a: T, b: T) -> T
where T: ValueTraits<T>
{
    // println!("Calculating LCM of {} and {}", a, b);
    if a < b
    {
        b / gcd(a, b) * a
    }
    else
    {
        a / gcd(a, b) * b
    }
}

impl<T> Ratio<T>
where T: ValueTraits<T>
{
    pub fn from(up: T, down: T) -> Result<Self, String>
    {
        let zero = T::from(0u8);

        if down == zero
        {
            return Err(String::from("Denominator cannot be zero"));
        }

        if up == zero
        {
            Ok(Ratio { numeritor: up, denominator: down })
        }
        else
        {
            let q: T = gcd(up, down);

            Ok(Ratio
            {
                numeritor: up / q,
                denominator: down / q,
            })
        }
    }

    pub fn fromInt(x: T) -> Self
    {
        Self::from(x, T::from(1)).unwrap()
    }

    pub fn zero() -> Self
    {
        Ratio::from(T::from(0u8), T::from(1u8)).unwrap()
    }

    pub fn isZero(&self) -> bool
    {
        self.numeritor == T::from(0u8)
    }

    pub fn isInteger(&self) -> bool
    {
        self.denominator == T::from(1u8) || self.isZero()
    }

    pub fn floor(&self) -> T
    {
        self.numeritor / self.denominator
    }

    pub fn ceil(&self) -> T
    {
        if self.isInteger()
        {
            self.numeritor
        }
        else
        {
            self.floor() + T::from(1u8)
        }
    }
}

impl<T> PartialEq for Ratio<T>
where T: ValueTraits<T>
{
    fn eq(&self, rhs: &Self) -> bool
    {
        let zero = T::from(0u8);
        if self.numeritor == zero && rhs.numeritor == zero
        {
            true
        }
        else
        {
            self.numeritor == rhs.numeritor && self.denominator == rhs.denominator
        }
    }
}

impl<T> Eq for Ratio<T>
where T: ValueTraits<T>
{}

impl<T> PartialOrd for Ratio<T>
where T: ValueTraits<T>
{
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
    {
        let m = lcm(self.denominator, rhs.denominator);
        (m / self.denominator * self.numeritor).partial_cmp(
            &(m / rhs.denominator * rhs.numeritor))
    }

}

impl<T> Add<Ratio<T>> for Ratio<T> where T: ValueTraits<T>
{
    type Output = Ratio<T>;
    fn add(self, rhs: Self) -> Self
    {
        let the_lcm = lcm(self.denominator, rhs.denominator);
        let up_lhs = the_lcm / self.denominator * self.numeritor;
        let up_rhs = the_lcm / rhs.denominator * rhs.numeritor;
        Ratio::from(up_lhs + up_rhs, the_lcm).unwrap()
    }
}

impl<T> Sum<Ratio<T>> for Ratio<T> where T: ValueTraits<T>
{
    fn sum<I>(iter: I) -> Self where I: Iterator<Item = Self>
    {
        iter.fold(Self::zero(), Add::add)
    }
}

impl<T> Mul<Ratio<T>> for Ratio<T> where T: ValueTraits<T>
{
    type Output = Ratio<T>;
    fn mul(self, rhs: Self) -> Self
    {
        if self.isZero() || rhs.isZero()
        {
            return Self::zero();
        }

        // Do reduction first to minimize change of overflow.
        let gcd1 = gcd(self.denominator, rhs.numeritor);
        let up1 = rhs.numeritor / gcd1;
        let down1 = self.denominator / gcd1;
        let gcd2 = gcd(rhs.denominator, self.numeritor);
        let up2 = self.numeritor / gcd2;
        let down2 = rhs.denominator / gcd2;

        Ratio::from(up1 * up2, down1 * down2).unwrap()
    }
}

impl<T> fmt::Debug for Ratio<T> where T: ValueTraits<T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{:?}/{:?}", self.numeritor, self.denominator)
    }
}

impl<T> fmt::Display for Ratio<T> where T: ValueTraits<T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{:?}/{:?}", self.numeritor, self.denominator)
    }
}

#[test]
fn testGCD()
{
    assert_eq!(gcd(12u32, 8u32), 4u32);
    assert_eq!(gcd(8u32, 12u32), 4u32);
    assert_eq!(gcd(4u32, 1u32), 1u32);
}

#[test]
fn testRatioEq()
{
    assert_eq!(Ratio::from(12u32, 8u32).unwrap(), Ratio::from(3u32, 2u32).unwrap());
    assert_ne!(Ratio::from(12u32, 8u32).unwrap(), Ratio::from(2u32, 3u32).unwrap());
    assert_eq!(Ratio::from(0u32, 8u32).unwrap(), Ratio::from(0u32, 2u32).unwrap());
}

#[test]
fn testRatioOrder()
{
    assert!(Ratio::from(12u32, 8u32).unwrap() < Ratio::from(24u32, 8u32).unwrap());
    assert!(Ratio::from(24u32, 8u32).unwrap() > Ratio::from(12u32, 8u32).unwrap());
}

#[test]
fn testZeroInt()
{
    assert!(Ratio::from(0u8, 1u8).unwrap().isZero());
    assert!(!Ratio::from(2u8, 1u8).unwrap().isZero());
    assert!(Ratio::from(0u8, 1u8).unwrap().isInteger());
    assert!(Ratio::from(1u8, 1u8).unwrap().isInteger());
    assert!(Ratio::from(2u8, 1u8).unwrap().isInteger());
    assert!(!Ratio::from(1u8, 2u8).unwrap().isInteger());
}

#[test]
fn testAdd()
{
    assert_eq!(Ratio::from(1u8, 2u8).unwrap() + Ratio::from(1u8, 3u8).unwrap(),
               Ratio::from(5u8, 6u8).unwrap());
    assert_eq!(Ratio::from(1u8, 2u8).unwrap() + Ratio::from(1u8, 2u8).unwrap(),
               Ratio::from(1u8, 1u8).unwrap());
    assert_eq!(Ratio::from(1u8, 2u8).unwrap() + Ratio::zero(),
               Ratio::from(1u8, 2u8).unwrap());
}

#[test]
fn testSum()
{
    let xs: Vec<Ratio<u8>> = vec![Ratio::from(1u8, 2u8).unwrap(),
                                  Ratio::from(1u8, 3u8).unwrap(),
                                  Ratio::from(1u8, 4u8).unwrap()];
    let total: Ratio<u8> = xs.iter().map(|x| *x).sum();
    assert_eq!(total, Ratio::from(13u8, 12u8).unwrap());
}

#[test]
fn testMul()
{
    assert_eq!(Ratio::from(1u8, 2u8).unwrap() * Ratio::from(1u8, 3u8).unwrap(),
               Ratio::from(1u8, 6u8).unwrap());
    assert_eq!(Ratio::from(1u8, 2u8).unwrap() * Ratio::zero(),
               Ratio::zero());
}

#[test]
fn testFloor()
{
    assert_eq!(Ratio::from(3u8, 2u8).unwrap().floor(), 1);
    assert_eq!(Ratio::from(2u8, 2u8).unwrap().floor(), 1);
    assert_eq!(Ratio::from(0u8, 2u8).unwrap().floor(), 0);
}

#[test]
fn testCeil()
{
    assert_eq!(Ratio::from(3u8, 2u8).unwrap().ceil(), 2);
    assert_eq!(Ratio::from(2u8, 2u8).unwrap().ceil(), 1);
    assert_eq!(Ratio::from(0u8, 2u8).unwrap().ceil(), 0);
}
