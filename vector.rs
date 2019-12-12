use std::fmt;
use std::ops::{Index, IndexMut, Add};

pub trait ValueTraits<T>: Add + fmt::Display + From<<T as std::ops::Add>::Output> + Copy
where T: Add + fmt::Display + Copy
{}
impl<T: Add + fmt::Display + From<<T as std::ops::Add>::Output> + Copy> ValueTraits<T> for T
{}

// Traits alias is still experimental
// https://github.com/rust-lang/rust/issues/41517

// trait ValueTraits<T> = Add + fmt::Display + From<<T as std::ops::Add>::Output> + Copy;

pub struct Vec3<T> where T: ValueTraits<T>
{
    data: [T; 3]
}

impl<T> Vec3<T> where T: ValueTraits<T>
{
    pub fn new(x: T, y: T, z: T) -> Self
    {
        Self
        {
            data: [x, y, z],
        }
    }
}

impl<T> Index<usize> for Vec3<T> where T: ValueTraits<T>
{
    type Output = T;
    fn index(&self, i: usize) -> &Self::Output
    {
        &self.data[i]
    }
}

impl<T> IndexMut<usize> for Vec3<T> where T: ValueTraits<T>
{
    fn index_mut(&mut self, i: usize) -> &mut Self::Output
    {
        &mut self.data[i]
    }
}

impl<T> Add for Vec3<T> where T: ValueTraits<T>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self
    {
        Self::new(T::from(self[0] + rhs[0]),
                  T::from(self[1] + rhs[1]),
                  T::from(self[2] + rhs[2]))
    }
}

impl<T> fmt::Display for Vec3<T> where T: ValueTraits<T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "({}, {}, {})", self[0], self[1], self[2])
    }
}
