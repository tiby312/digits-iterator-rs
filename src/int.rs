pub trait Int: Copy + PartialOrd<Self> {
    type Unsigned;

    fn as_unsigned(self) -> Self::Unsigned;
    fn one() -> Self;
    fn checked_mul(self, other: Self) -> Option<Self>;
    fn from_base(base: u32) -> Self;
}

macro_rules! impl_int {
    { $t: ty, $u: ty } => {
        impl Int for $t {
            type Unsigned = $u;

            fn as_unsigned(self) -> $u {
                self as $u
            }

            fn one() -> $t { 1 }

            fn checked_mul(self, other: Self) -> Option<Self> {
                self.checked_mul(other)
            }

            fn from_base(base: u32) -> $t {
                base as $t
            }
        }
    }
}

impl_int!(usize, usize);
impl_int!(isize, usize);

impl_int!(u128, u128);
impl_int!(i128, u128);

impl_int!(u64, u64);
impl_int!(i64, u64);

impl_int!(u32, u32);
impl_int!(i32, u32);

impl_int!(u16, u16);
impl_int!(i16, u16);

impl_int!(u8, u8);
impl_int!(i8, u8);
