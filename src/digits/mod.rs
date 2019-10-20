#[cfg(test)]
mod tests;

use int::Int;

pub struct Digits<T>
where
    T: Int,
{
    n: T::Unsigned,
    div: T::Unsigned,
    base: T::Unsigned,
}

impl<T> Digits<T>
where
    T: Int,
{
    pub fn new(n: T) -> Digits<T> {
        Self::with_base(n, 10)
    }

    pub fn with_base(n: T, base: u32) -> Digits<T> {
        fn is_ok_as_base(base: u32) -> bool {
            base >= 2 && base <= 36
        }
        assert!(is_ok_as_base(base));

        let base = T::from_base(base);

        let mut div = T::one();
        loop {
            div = match div.checked_mul(base) {
                None => break,
                Some(i) if n < i => break,
                Some(i) => i,
            };
        }

        Digits {
            n: n.as_unsigned(),
            div: div.as_unsigned(),
            base: base.as_unsigned(),
        }
    }
}

macro_rules! impl_iterators {
    { $from: ty, $u: ty } => {
        impl Iterator for Digits<$from> {
            type Item = u8;

            fn next(&mut self) -> Option<Self::Item> {
                if self.div == 0 {
                    None
                } else {
                    let result = self.n / self.div;
                    self.n %= self.div;
                    self.div /= self.base;

                    Some(result as u8)
                }
            }
        }

        impl DoubleEndedIterator for Digits<$from> {
            fn next_back(&mut self) -> Option<Self::Item> {
                if self.div == 0 {
                    None
                } else {
                    let result = self.n % self.base;
                    self.n /= self.base;
                    self.div /= self.base;

                    Some(result as u8)
                }
            }
        }
    }
}

impl_iterators!(isize, usize);
impl_iterators!(usize, usize);

impl_iterators!(i128, u128);
impl_iterators!(u128, u128);

impl_iterators!(i64, u64);
impl_iterators!(u64, u64);

impl_iterators!(i32, u32);
impl_iterators!(u32, u32);

impl_iterators!(i16, u16);
impl_iterators!(u16, u16);

impl_iterators!(i8, u8);
impl_iterators!(u8, u8);
