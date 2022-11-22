pub trait SnesNum: Copy + Clone + Sized + Eq + PartialEq {
    fn add_will_carry(&self, v: Self, carry: bool) -> bool;
    fn sbc_will_carry(&self, v: Self, carry: bool) -> bool;
    fn is_overflow(&self, v: Self, r: Self) -> bool;
    fn add_snes(&self, v: Self, carry: bool) -> Self;
    fn sbc_snes(&self, v: Self, carry: bool) -> Self;
    fn and(&self, v: Self) -> Self;
    fn asl(&self) -> Self;
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

macro_rules! define_is_overflow {
    ($t:ty) => {
        fn is_overflow(&self, v: $t, r: $t) -> bool {
            let target = (*self).is_negative();
            let value = v.is_negative();
            let result = r.is_negative();
            (target ^ result) && (target ^ value)
        }
    }
}

macro_rules! define_impl {
    ($t:ty) => {
        impl SnesNum for $t {
            define_will_carry!($t, add_will_carry, checked_add);
            define_will_carry!($t, sbc_will_carry, checked_sub);

            define_operation!($t, add_snes, wrapping_add);
            define_operation!($t, sbc_snes, wrapping_sub);

            define_is_overflow!($t);

            fn and(&self, v: $t) -> $t {
                (* self) & v
            }

            fn asl(&self) -> $t {
                (* self) << 1
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