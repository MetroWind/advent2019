use std::fmt;
use std::ops::{Div, Rem, Mul};
use std::cmp::Ordering;
use std::hash::Hash;

pub trait ValueTraits<T>: Div + Rem + Mul + PartialOrd + Copy +
    PartialEq + Hash + fmt::Display + From<<T as
    std::ops::Div>::Output> + From<<T as std::ops::Rem>::Output> +
    From<<T as std::ops::Mul>::Output> + From<u32>

where T: Div + Rem + Mul + PartialOrd + Copy + PartialEq + Hash + fmt::Display
{}

impl<T: Div + Rem + Mul + PartialOrd + Copy + PartialEq + Hash +
     fmt::Display + From<<T as std::ops::Div>::Output> + From<<T as
    std::ops::Rem>::Output> + From<<T as std::ops::Mul>::Output> +
     From<u32>> ValueTraits<T> for T
{}


#[derive(Debug, Hash)]
pub struct Ratio<T>
where T: ValueTraits<T>
{
    numeritor: T,
    denominator: T,
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

    let zero: T = T::from(0);
    loop
    {
        r1 = T::from(r1 % r2);
        if r1 == zero
        {
            break r2;
        }
        r2 = T::from(r2 % r1);
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
        T::from(T::from(b / gcd(a, b)) * a)
    }
    else
    {
        T::from(T::from(a / gcd(a, b)) * b)
    }
}

impl<T> Ratio<T>
where T: ValueTraits<T>
{
    pub fn from(up: T, down: T) -> Result<Self, String>
    {
        let zero = T::from(0);

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
                numeritor: T::from(up / q),
                denominator: T::from(down / q),
            })
        }
    }
}

impl<T> PartialEq for Ratio<T>
where T: ValueTraits<T>
{
    fn eq(&self, rhs: &Self) -> bool
    {
        let zero = T::from(0);
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
        T::from(T::from(m / self.denominator) * self.numeritor).partial_cmp(
            &(T::from(T::from(m / rhs.denominator) * rhs.numeritor)))
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
