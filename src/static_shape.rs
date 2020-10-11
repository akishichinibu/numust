
macro_rules! declare_static_shape {
    ($dim_token:ident, $dim:expr) => {

pub struct $dim_token {
    shape: [usize; $dim],
    pub projector: [usize; $dim],
    pub restrict: [usize; $dim],
    size: usize,
}


impl $dim_token {
    const DIM: usize = $dim;

    pub fn from(shape: [usize; Self::DIM]) -> $dim_token {
        let size = shape.iter().fold(1, |r, x| r * x);

        let mut projector: [usize; Self::DIM] = [1; Self::DIM];
        (0..Self::DIM - 1).rev().for_each(|i| projector[i] = shape[i + 1] * projector[i + 1]);

        let mut restrict: [usize; Self::DIM] = [shape[Self::DIM - 1]; Self::DIM];
        (0..Self::DIM - 1).rev().for_each(|i| restrict[i] = shape[i] * restrict[i + 1]);

        $dim_token {
            shape,
            projector,
            restrict,
            size,
        }
    }

    #[inline]
    pub fn dim(&self) -> usize {
        Self::DIM
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }
    //
    // #[inline]
    // pub fn shape(&self) -> &Vec<usize> {
    //     &self.shape
    // }

    #[inline]
    pub fn at(&self, i: usize) -> usize {
        self.shape[i]
    }

    pub fn projection(&self, src: &$dim_token, t: usize) -> usize {
        (0..Self::DIM)
            .map(|i| (t % src.restrict[i]) / src.projector[i])
            .enumerate()
            .map(|(i, x)| x * self.projector[i])
            .fold(0, |s, x| s + x)
    }

    pub fn binary_op(sp1: &$dim_token, sp2: &$dim_token) -> $dim_token {
        let mut buf: [usize; Self::DIM] = [0; Self::DIM];

        for i in 0..sp1.dim() {
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

        $dim_token::from(buf)
    }
}
    }
}

declare_static_shape!(Shape1, 1);
declare_static_shape!(Shape2, 2);
declare_static_shape!(Shape3, 3);

