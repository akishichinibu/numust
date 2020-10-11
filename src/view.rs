use std::fmt;
use std::ops;
use crate::shape::Shape;
use std::ops::{Add, Sub, Mul, Div};

extern crate num;

use self::num::{FromPrimitive, PrimInt, ToPrimitive};
use self::num::traits::AsPrimitive;
use crate::ndarray::Ndarray;


struct BaseView {
    pub slice: Vec<(usize, usize, usize)>,
    shape: Shape,
}


pub struct View<'a, T> where T: Clone + FromPrimitive + ToPrimitive {
    base: BaseView,
    pub src: &'a mut Ndarray<T>,
}


pub struct ImmutableView<'a, T> where T: Clone + FromPrimitive + ToPrimitive {
    base: BaseView,
    pub src: &'a Ndarray<T>,
}


macro_rules! unary_operator_override {
    ($name:ident) => {
        pub fn $name(&mut self) -> &View<T> where T: Clone + FromPrimitive + ToPrimitive {
            self.unary_op(|a| a.$name())
        }
    }
}


macro_rules! binary_operator_override {
    ($Op:ident, $name:ident, $op:expr) => {
        impl<L, R> ops::$Op<ImmutableView<'_, R>> for ImmutableView<'_, L>
        where L: Clone + FromPrimitive + ToPrimitive, R: Clone + FromPrimitive + ToPrimitive {
            type Output = Ndarray<f64>;
            fn $name(self, _rhs: ImmutableView<R>) -> Self::Output {
                self.binary_op(_rhs, $op)
            }
        }
    }
}


impl BaseView {
    fn remapping(&self, t: usize) -> usize {
        (0..self.shape.dim())
            .map(|i| (t % self.shape.restrict[i]) / self.shape.projector[i])
            .enumerate()
            .map(|(i, x)| self.shape.projector[i] * (self.slice[i].0 + self.slice[i].2 * x))
            .fold(0, |s, x| s + x)
    }
}


impl<T> View<'_, T> where T: Clone + FromPrimitive + ToPrimitive {
    
    pub fn from_slice(src: &mut Ndarray<T>, slice: Vec<(usize, usize, usize)>) -> View<T> {
        let sp = slice.iter().map(|(l, r, d)| (r - l) / d).collect::<Vec<usize>>();
        let shape = Shape::from(sp);

        View {
            base: BaseView {
                slice,
                shape,
            },
            src,
        }
    }

    pub fn unary_op<F>(&mut self, op: F) -> &View<'_, T> where F: Fn(&f64) -> f64 {
        (0..self.base.shape.size()).for_each(|i| {
            let t = self.base.remapping(i);
            self.src.buffer[t] = T::from_f64(op(&self.src.buffer[t].to_f64().unwrap())).unwrap();
        });
        self
    }

    unary_operator_override!(sin);
    unary_operator_override!(cos);
    unary_operator_override!(tan);
}


impl<T> ImmutableView<'_, T> where T: Clone + FromPrimitive + ToPrimitive {
    pub fn from_slice(src: &Ndarray<T>, slice: Vec<(usize, usize, usize)>) -> ImmutableView<T> {
        let sp = slice.iter().map(|(l, r, d)| (r - l) / d).collect::<Vec<usize>>();

        let shape = Shape::from(sp);

        ImmutableView {
            base: BaseView {
                slice,
                shape,
            },
            src,
        }
    }

    pub fn binary_op<R, F>(&self, _rhs: ImmutableView<R>, op: F) -> Ndarray<f64>
        where R: Clone + FromPrimitive + ToPrimitive,
              F: Fn(&f64, &f64) -> f64 {
        let shape = Shape::binary_op(&self.base.shape, &_rhs.base.shape);
        let mut result: Ndarray<f64> = Ndarray::foo_with_shape(&shape, 0.0);

        for i in 0..result.shape.size() {
            let lv = &self.src.buffer[self.base.remapping(i)].to_f64().unwrap();
            let rv = &_rhs.src.buffer[_rhs.base.remapping(i)].to_f64().unwrap();
            result.buffer[i] = op(lv, rv);
        }

        result
    }
}


binary_operator_override!(Add, add, |a, b| a + b);
binary_operator_override!(Sub, sub, |a, b| a - b);
binary_operator_override!(Mul, mul, |a, b| a * b);
binary_operator_override!(Div, div, |a, b| a / b);
binary_operator_override!(Pow, pow, |a, b| a ** b);


impl<T> fmt::Display for View<'_, T> where T: Clone + FromPrimitive + ToPrimitive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _str = (0..self.base.shape.size())
            .map(|i| self.src.buffer[self.base.remapping(i)].to_f64().unwrap().to_string())
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "({0})[{1}]", self.base.shape.size(), _str)
    }
}
