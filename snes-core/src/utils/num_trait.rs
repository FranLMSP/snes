pub trait SnesNum: Copy + Clone + Sized + Eq + PartialEq {
    fn add_will_carry(&self, v: Self, carry: bool) -> bool;
    fn sub_will_carry(&self, v: Self, carry: bool) -> bool;
    fn add_snes(&self, v: Self, carry: bool) -> Self;
    fn sub_snes(&self, v: Self, carry: bool) -> Self;
    fn and(&self, v: Self) -> Self;
    fn is_negative(&self) -> bool;
    fn is_zero(&self) -> bool;
}

macro_rules! define_will_carry {
    ($t:ty, $method:ident, $inner_method:ident) => {
        fn $method(&self, v: $t, carry: bool) -> bool {
            match self.$inner_method(v) {
                None => true,
                Some(res) => match res.$inner_method(carry as $t) {
                    None => true,
                    Some(_) => false,
                },
            }
        }
    }
}

macro_rules! define_operation {
    ($t:ty, $method:ident, $inner_method:ident) => {
        fn $method(&self, v: $t, carry: bool) -> $t {
            self.$inner_method(v).$inner_method(carry as $t)
        }
    }
}

macro_rules! define_impl {
    ($t:ty) => {
        impl SnesNum for $t {
            define_will_carry!($t, add_will_carry, checked_add);
            define_will_carry!($t, sub_will_carry, checked_sub);

            define_operation!($t, add_snes, wrapping_add);
            define_operation!($t, sub_snes, wrapping_sub);

            fn and(&self, v: $t) -> $t {
                (* self) & v
            }

            fn is_negative(&self) -> bool {
                (*self) & !(<$t>::MAX >> 1) != 0
            }

            fn is_zero(&self) -> bool {
                (*self) == 0
            }
        }
    };
}

define_impl!(u8);
define_impl!(u16);