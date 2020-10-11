use std::fmt;
use std::ops;
use crate::shape::Shape;

extern crate num;

use self::num::{FromPrimitive, ToPrimitive};
use crate::view::{View, ImmutableView};


pub struct Ndarray<T> where T: Clone + FromPrimitive + ToPrimitive {
    pub shape: Shape,
    pub buffer: Vec<T>,
}


macro_rules! unary_operation_transfer {
    ($name:ident) => {
        pub fn $name(&mut self) -> View<T> {
            let mut view = self.reflect();
            view.$name();
            view
        }
    }
}


macro_rules! binary_operation_transfer {
    ($Op:ident, $name:ident, $op:expr) => {
        impl<L, R> ops::$Op<Ndarray<R>> for Ndarray<L>
        where L: Clone + FromPrimitive + ToPrimitive, R: Clone + FromPrimitive + ToPrimitive {
            type Output = Ndarray<f64>;
            fn $name(self, _rhs: Ndarray<R>) -> Self::Output {
                let view_left = self.immutable_reflect();
                let view_right = _rhs.immutable_reflect();
                $op(view_left, view_right)
            }
        }

        impl<'a, 'b, L, R> ops::$Op<&'b Ndarray<R>> for &'a Ndarray<L>
        where L: Clone + FromPrimitive + ToPrimitive, R: Clone + FromPrimitive + ToPrimitive {
            type Output = Ndarray<f64>;
            fn $name(self, _rhs: &Ndarray<R>) -> Self::Output {
                let view_left = self.immutable_reflect();
                let view_right = _rhs.immutable_reflect();
                $op(view_left, view_right)
            }
        }
    }
}


impl<T> Ndarray<T> where T: Clone + FromPrimitive + ToPrimitive {

    pub(crate) fn foo_with_shape(shape: &Shape, placeholder: T) -> Ndarray<T> {
        let size = shape.size();
        let mut buffer: Vec<T> = Vec::with_capacity(size);
        buffer.resize(size, placeholder);

        let mut ndarray: Ndarray<T> = Ndarray {
            shape: shape.clone(),
            buffer,
        };

        return ndarray;
    }

    pub fn foo(shape: Vec<usize>, placeholder: T) -> Ndarray<T> {
        Self::foo_with_shape(&Shape::from(shape), placeholder)
    }

    pub fn ones(shape: Vec<usize>) -> Ndarray<T> {
        Self::foo(shape, T::from_i32(1).unwrap())
    }

    pub fn zeros(shape: Vec<usize>) -> Ndarray<T> {
        Self::foo(shape, T::from_i32(1).unwrap())
    }

    fn size(&self) -> usize {
        return self.shape.size();
    }

    pub fn slice(&mut self, s: Vec<(Option<usize>, Option<usize>, Option<usize>)>) -> View<T> {
        let ss = s.iter().enumerate().map(|(i, (l, r, d))| (l.unwrap_or(0), r.unwrap_or(self.shape.at(i)), d.unwrap_or(1))).collect::<Vec<(usize, usize, usize)>>();
        self.slice_unsafe(ss)
    }

    fn slice_unsafe(&mut self, s: Vec<(usize, usize, usize)>) -> View<T> {
        View::from_slice(self, s)
    }

    fn immutable_slice(&self, s: Vec<(usize, usize, usize)>) -> ImmutableView<T> {
        ImmutableView::from_slice(self, s)
    }

    fn reflect(&mut self) -> View<T> {
        self.slice_unsafe(self.shape.shape().iter().map(|x| (0, *x, 1)).collect::<Vec<(usize, usize, usize)>>())
    }

    fn immutable_reflect(&self) -> ImmutableView<T> {
        self.immutable_slice(self.shape.shape().iter().map(|x| (0, *x, 1)).collect::<Vec<(usize, usize, usize)>>())
    }

    unary_operation_transfer!(sin);
    unary_operation_transfer!(cos);
    unary_operation_transfer!(tan);

    // fn string_to_buffer(&self, f: &mut fmt::Formatter<'_>, coords: Vec<usize>) -> fmt::Result {
    //     let level = coords.len();
    //     let without_padding = level == 0 || coords[level - 1] == 0;
    //
    //     if level == self.dim() - 1 {
    //         for i in (0..self.shape.at(level)) {
    //             write!(f, "{}", )
    //         }
    //     }
    // }
}


binary_operation_transfer!(Add, add, |a, b| a + b);
binary_operation_transfer!(Sub, sub, |a, b| a - b);
binary_operation_transfer!(Mul, mul, |a, b| a * b);
binary_operation_transfer!(Div, div, |a, b| a / b);
binary_operation_transfer!(Pow, pow, |a, b| a ** b);


impl<T> fmt::Display for Ndarray<T> where T: Clone + FromPrimitive + ToPrimitive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //  maxLength = maxLength === null ? Math.round(100 ** (1.0 / this.dim)) : maxLength;
        //
        // const level: number = curSlice.length;
        // const withoutPaddingHead = level === 0 || curSlice[level - 1] === 0;
        //
        // if (level === this.dim - 1) {
        //     const valueBuffer = [];
        //     for (let i = 0; i < this.shapeObj.at(level); i++) {
        //         valueBuffer.push(this.at(...curSlice, i).toFixed(maxDigital));
        //         if (i > maxLength) {
        //             valueBuffer.push(" ...");
        //             break;
        //         }
        //     }
        //     return `${withoutPaddingHead ? "" : " ".repeat(level)}[${valueBuffer.join(", ")}]`;
        // }
        //
        // const buf = []
        // for (let i = 0; i < this.shapeObj.at(level); i++) {
        //     const nextSlice = [...curSlice, i];
        //     buf.push(this.prettyString(nextSlice));
        //     if (i > maxLength) {
        //         buf.push(" ...");
        //         break;
        //     }
        // }
        //
        // return `${withoutPaddingHead ? "" : ' '.repeat(level)}[${buf.join(", \n")}]`;

        let _str = self.buffer.iter().map(|x| x.to_f64().unwrap().to_string()).collect::<Vec<String>>().join(", ");
        write!(f, "({0})[{1}]", self.size(), _str)
    }
}
