extern crate num;
use self::num::{FromPrimitive, PrimInt, ToPrimitive};
use self::num::traits::AsPrimitive;


// struct BinaryOperate;
//
//
// pub trait BinaryOperateTraits<L, R> where T: Clone + FromPrimitive + ToPrimitive, R: Clone + FromPrimitive + ToPrimitive {
//     type Output;
// }
//
// impl<T> BinaryOperateTraits<T, f64> for BinaryOperate where T: Clone + FromPrimitive + ToPrimitive {
//     type Output = f64;
// }
//
// impl<T> BinaryOperateTraits<T, f32> for BinaryOperate where T: Clone + FromPrimitive + ToPrimitive {
//     type Output = f32;
// }
//
// impl BinaryOperateTraits<i32, i32> for BinaryOperate where T: Clone + FromPrimitive + ToPrimitive {
//     type Output = i32;
// }
//
// impl BinaryOperateTraits<i32, i16> for BinaryOperate where T: Clone + FromPrimitive + ToPrimitive {
//     type Output = i32;
// }
//
//
//
// struct UnaryOperate;
//
//
// pub trait UnaryOperateTraits<L, R> where T: Clone + FromPrimitive + ToPrimitive, R: Clone + FromPrimitive + ToPrimitive {
//     type Output;
// }
//
// impl<T> UnaryOperateTraits<T, f64> for BinaryOperate where T: Clone + FromPrimitive + ToPrimitive {
//     type Output = f64;
// }
//
// impl<T> UnaryOperateTraits<T, f32> for BinaryOperate where T: Clone + FromPrimitive + ToPrimitive {
//     type Output = f32;
// }
