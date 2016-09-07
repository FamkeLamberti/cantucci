#![allow(dead_code)]

use core::math::*;
use std::ops::Sub;

pub trait ToArr {
    type Output;

    fn to_arr(&self) -> Self::Output;
}

macro_rules! to_arr_impl_gen_into_type {
    ($ty:ident, $out:ty) => {
        impl<T: BaseNum> ToArr for $ty <T> {
            type Output = $out;

            fn to_arr(&self) -> Self::Output {
                (*self).into()
            }
        }
    }
}

to_arr_impl_gen_into_type!(Matrix4, [[T; 4]; 4]);
to_arr_impl_gen_into_type!(Point3, [T; 3]);
to_arr_impl_gen_into_type!(Vector3, [T; 3]);


pub fn lerp<V, F>(a: V, b: V, t: F) -> V
    where V: Lerp<F>,
          F: LerpFactor
{
    assert!(t >= F::zero() && t <= F::one());
    a.lerp(b, t)
}

pub trait LerpFactor: Zero + One + PartialOrd + Sub<Output=Self> {}
impl LerpFactor for f32 {}
impl LerpFactor for f64 {}

pub trait Lerp<F: LerpFactor> {
    fn lerp(self, other: Self, t: F) -> Self;
}

macro_rules! impl_lerp {
    ($self_type:ty, $factor:ty) => {
        impl Lerp<$factor> for $self_type {
            fn lerp(self, other: Self, t: $factor) -> Self {
                self * (1.0 - t) + other * t
            }
        }
    }
}

impl_lerp!(Vector3<f64>, f64);
impl_lerp!(f64, f64);
impl_lerp!(Rad<f64>, f64);

impl Lerp<f64> for Point3<f64> {
    fn lerp(self, other: Self, t: f64) -> Self {
        self * (1.0 - t) + other.to_vec() * t
    }
}


pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    assert!(min < max);

    match () {
        () if val < min => min,
        () if val > max => max,
        _ => val,
    }
}
