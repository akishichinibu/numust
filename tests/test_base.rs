extern crate numust;

use std::option::Option;

use numust::ndarray::Ndarray;
use numust::static_shape::Shape3;


macro_rules! s {
    ($start:tt) => { { (Option::from($start), Option::<usize>::None, Option::<usize>::None) } };
    ($start:tt:$end:tt) => { { (Option::from($start), Option::from($end), Option::from(1)) } };
    ($start:tt::$step:tt) => { { (Option::from($start), Option::<usize>::None, Option::from($step)) } };
    (:$end:tt:$step:tt) => { { (Option::<usize>::None, Option::from($end), Option::from($step)) } };
    ($start:tt:$end:tt:$step:tt) => { { (Option::from($start), Option::from($end), Option::from($step)) } };
}


#[test]
fn test_base() {
    let mut s1: Ndarray<f64> = Ndarray::ones(vec! [7, 5, 1]);
    let mut s2: Ndarray<f32> = Ndarray::ones(vec! [1, 1, 3]);

    let mut s = &s1 + &s2;
    // s.slice(vec![s!(2:6:1), s!(0:4:2), s!(0:2)]).sin();

    let h1 = s!(2:6:1);
    let hx1 = vec!(s!(2:6:1), s!(2:6:1), s!(2:6:1));

    let kk = Shape3::from([3, 4, 2]);


    // // println!("{}", s1.shape);
    // println!("{}", s1.sin());
    // // println!("{}", s2.cos());
    // println!("{}", s);
    // println!("{}", h);
}
