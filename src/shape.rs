use std::fmt;
use std::ops;


#[derive(Clone, Debug)]
pub struct Shape {
    dim: usize,
    shape: Vec<usize>,
    pub projector: Vec<usize>,
    pub restrict: Vec<usize>,
    size: usize,
}

impl Shape {

    pub fn from(shape: Vec<usize>) -> Shape {
        let dim = shape.len();
        let size = shape.iter().fold(1, |r, x| r * x);

        let mut projector: Vec<usize> = Vec::with_capacity(dim);
        projector.resize(dim, 1);
        (0..dim - 1).rev().for_each(|i| projector[i] = shape[i + 1] * projector[i + 1]);

        let mut restrict: Vec<usize> = Vec::with_capacity(dim);
        restrict.resize(dim, shape[dim - 1]);
        (0..dim - 1).rev().for_each(|i| restrict[i] = shape[i] * restrict[i + 1]);

        Shape {
            dim,
            shape,
            projector,
            restrict,
            size,
        }
    }

    #[inline]
    pub fn dim(&self) -> usize {
        self.dim
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn shape(&self) -> &Vec<usize> {
        &self.shape
    }

    #[inline]
    pub fn at(&self, i: usize) -> usize {
        self.shape[i]
    }

    pub fn projection(&self, src: &Shape, t: usize) -> usize {
        (0..self.dim())
            .map(|i| (t % src.restrict[i]) / src.projector[i])
            .enumerate()
            .map(|(i, x)| x * self.projector[i])
            .fold(0, |s, x| s + x)
    }

    pub fn binary_op(sp1: &Shape, sp2: &Shape) -> Shape {
        if sp1.dim != sp2.dim {
            panic!("aa");
        }

        let mut buf: Vec<usize> = Vec::new();
        buf.resize(sp1.dim, 1);

        for i in 0..sp1.dim {
            let [ls, rs] = [sp1.shape[i], sp2.shape[i]];

            if ls == 1 {
                buf[i] = rs;
            } else if rs == 1 {
                buf[i] = ls;
            } else {
                if ls == rs {
                    buf[i] = ls;
                } else {
                    panic!("aa");
                }
            }
        }

        Shape::from(buf)
    }
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let shape_str = self.shape.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        write!(f, "({0})[{1}]", shape_str.join(" x "), self.size)
    }
}
