use std::ops::{Div, Rem, Mul};
use std::cmp::Ordering;
use std::hash::Hash;

#[derive(Debug, Hash)]
pub struct Ratio<T>
where T: Div + Rem + Mul + PartialOrd + Copy + PartialEq + Hash +
    From<<T as std::ops::Div>::Output> + From<<T as std::ops::Rem>::Output> +
    From<<T as std::ops::Mul>::Output> + From<i32>
{
    numeritor: T,
    denominator: T,
}

// Euler’s method. 也就是辗转相除法？
fn gcd<T>(up: T, down: T) -> T
where T: Div + Rem + Mul + PartialOrd + Copy + PartialEq + Hash +
    From<<T as std::ops::Div>::Output> + From<<T as std::ops::Rem>::Output> +
    From<<T as std::ops::Mul>::Output> + From<i32>
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

fn lcm<T>(a: T, b: T) -> T
where T: Div + Rem + Mul + PartialOrd + Copy + PartialEq + Hash +
    From<<T as std::ops::Div>::Output> + From<<T as std::ops::Rem>::Output> +
    From<<T as std::ops::Mul>::Output> + From<i32>
{
    T::from(T::from(a * b) / gcd(a, b))
}

impl<T> Ratio<T>
where T: Div + Rem + Mul + PartialOrd + Copy + PartialEq + Hash +
    From<<T as std::ops::Div>::Output> + From<<T as std::ops::Rem>::Output> +
    From<<T as std::ops::Mul>::Output> + From<i32>
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
where T: Div + Rem + Mul + PartialOrd + Copy + PartialEq + Hash +
    From<<T as std::ops::Div>::Output> + From<<T as std::ops::Rem>::Output> +
    From<<T as std::ops::Mul>::Output> + From<i32>
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
where T: Div + Rem + Mul + PartialOrd + Copy + PartialEq + Hash +
    From<<T as std::ops::Div>::Output> + From<<T as std::ops::Rem>::Output> +
    From<<T as std::ops::Mul>::Output> + From<i32>
{}

impl<T> PartialOrd for Ratio<T>
where T: Div + Rem + Mul + PartialOrd + Copy + PartialEq + Hash +
    From<<T as std::ops::Div>::Output> + From<<T as std::ops::Rem>::Output> +
    From<<T as std::ops::Mul>::Output> + From<i32>
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
    assert_eq!(gcd(12, 8), 4);
    assert_eq!(gcd(8, 12), 4);
    assert_eq!(gcd(4, 1), 1);
}

#[test]
fn testRatioEq()
{
    assert_eq!(Ratio::from(12, 8).unwrap(), Ratio::from(3, 2).unwrap());
    assert_ne!(Ratio::from(12, 8).unwrap(), Ratio::from(2, 3).unwrap());
    assert_eq!(Ratio::from(0, 8).unwrap(), Ratio::from(0, 2).unwrap());
}

#[test]
fn testRatioOrder()
{
    assert!(Ratio::from(12, 8).unwrap() < Ratio::from(24, 8).unwrap());
    assert!(Ratio::from(24, 8).unwrap() > Ratio::from(12, 8).unwrap());
}
