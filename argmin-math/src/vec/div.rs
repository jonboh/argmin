// Copyright 2018-2022 argmin developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::ArgminDiv;
use num_complex::Complex;

macro_rules! make_div {
    ($t:ty) => {
        impl ArgminDiv<$t, Vec<$t>> for Vec<$t> {
            #[inline]
            fn div(&self, other: &$t) -> Vec<$t> {
                self.iter().map(|a| a / other).collect()
            }
        }

        impl ArgminDiv<Vec<$t>, Vec<$t>> for $t {
            #[inline]
            fn div(&self, other: &Vec<$t>) -> Vec<$t> {
                other.iter().map(|a| self / a).collect()
            }
        }

        impl ArgminDiv<Vec<$t>, Vec<$t>> for Vec<$t> {
            #[inline]
            fn div(&self, other: &Vec<$t>) -> Vec<$t> {
                let n1 = self.len();
                let n2 = other.len();
                assert!(n1 > 0);
                assert_eq!(n1, n2);
                self.iter().zip(other.iter()).map(|(a, b)| a / b).collect()
            }
        }

        impl ArgminDiv<Vec<Vec<$t>>, Vec<Vec<$t>>> for Vec<Vec<$t>> {
            #[inline]
            fn div(&self, other: &Vec<Vec<$t>>) -> Vec<Vec<$t>> {
                let sr = self.len();
                let or = other.len();
                assert!(sr > 0);
                // implicitly, or > 0
                assert_eq!(sr, or);
                let sc = self[0].len();
                self.iter()
                    .zip(other.iter())
                    .map(|(a, b)| {
                        assert_eq!(a.len(), sc);
                        assert_eq!(b.len(), sc);
                        <Vec<$t> as ArgminDiv<Vec<$t>, Vec<$t>>>::div(&a, &b)
                    })
                    .collect()
            }
        }
    };
}

make_div!(isize);
make_div!(usize);
make_div!(i8);
make_div!(u8);
make_div!(i16);
make_div!(u16);
make_div!(i32);
make_div!(u32);
make_div!(i64);
make_div!(u64);
make_div!(f32);
make_div!(f64);
make_div!(Complex<isize>);
make_div!(Complex<usize>);
make_div!(Complex<i8>);
make_div!(Complex<u8>);
make_div!(Complex<i16>);
make_div!(Complex<u16>);
make_div!(Complex<i32>);
make_div!(Complex<u32>);
make_div!(Complex<i64>);
make_div!(Complex<u64>);
make_div!(Complex<f32>);
make_div!(Complex<f64>);

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use paste::item;

    macro_rules! make_test {
        ($t:ty) => {
            item! {
                #[test]
                fn [<test_div_vec_scalar_ $t>]() {
                    let a = vec![2 as $t, 4 as $t, 8 as $t];
                    let b = 2 as $t;
                    let target = vec![1 as $t, 2 as $t, 4 as $t];
                    let res = <Vec<$t> as ArgminDiv<$t, Vec<$t>>>::div(&a, &b);
                    for i in 0..3 {
                        assert_relative_eq!(target[i] as f64, res[i] as f64, epsilon = std::f64::EPSILON);
                    }
                }
            }

            item! {
                #[test]
                fn [<test_div_vec_scalar_complex_ $t>]() {
                    let a = vec![
                        Complex::new(2 as $t, 4 as $t),
                        Complex::new(4 as $t, 8 as $t),
                        Complex::new(3 as $t, 9 as $t),
                    ];
                    let b = Complex::new(2 as $t, 4 as $t);
                    let target = vec![a[0]/b, a[1]/b, a[2]/b];
                    let res = <Vec<Complex<$t>> as ArgminDiv<Complex<$t>, Vec<Complex<$t>>>>::div(&a, &b);
                    for i in 0..3 {
                        assert_relative_eq!(target[i].re as f64, res[i].re as f64, epsilon = std::f64::EPSILON);
                        assert_relative_eq!(target[i].im as f64, res[i].im as f64, epsilon = std::f64::EPSILON);
                    }
                }
            }

            item! {
                #[test]
                fn [<test_div_scalar_vec_ $t>]() {
                    let a = vec![2 as $t, 4 as $t, 8 as $t];
                    let b = 64 as $t;
                    let target = vec![32 as $t, 16 as $t, 8 as $t];
                    let res = <$t as ArgminDiv<Vec<$t>, Vec<$t>>>::div(&b, &a);
                    for i in 0..3 {
                        assert_relative_eq!(target[i] as f64, res[i] as f64, epsilon = std::f64::EPSILON);
                    }
                }
            }

            item! {
                #[test]
                fn [<test_div_scalar_vec_complex_ $t>]() {
                    let a = vec![
                        Complex::new(2 as $t, 1 as $t),
                        Complex::new(2 as $t, 2 as $t),
                        Complex::new(3 as $t, 2 as $t),
                    ];
                    let b = Complex::new(10 as $t, 12 as $t);
                    let target = vec![b/a[0], b/a[1], b/a[2]];
                    let res = <Complex<$t> as ArgminDiv<Vec<Complex<$t>>, Vec<Complex<$t>>>>::div(&b, &a);
                    for i in 0..3 {
                        assert_relative_eq!(target[i].re as f64, res[i].re as f64, epsilon = std::f64::EPSILON);
                        assert_relative_eq!(target[i].im as f64, res[i].im as f64, epsilon = std::f64::EPSILON);
                    }
                }
            }

            item! {
                #[test]
                fn [<test_div_vec_vec_ $t>]() {
                    let a = vec![4 as $t, 9 as $t, 8 as $t];
                    let b = vec![2 as $t, 3 as $t, 4 as $t];
                    let target = vec![2 as $t, 3 as $t, 2 as $t];
                    let res = <Vec<$t> as ArgminDiv<Vec<$t>, Vec<$t>>>::div(&a, &b);
                    for i in 0..3 {
                        assert_relative_eq!(target[i] as f64, res[i] as f64, epsilon = std::f64::EPSILON);
                    }
                }
            }

            item! {
                #[test]
                fn [<test_div_vec_vec_complex_ $t>]() {
                    let a = vec![
                        Complex::new(8 as $t, 7 as $t),
                        Complex::new(7 as $t, 6 as $t),
                        Complex::new(6 as $t, 5 as $t),
                    ];
                    let b = vec![
                        Complex::new(4 as $t, 2 as $t),
                        Complex::new(4 as $t, 3 as $t),
                        Complex::new(3 as $t, 2 as $t),
                    ];
                    let target = vec![a[0]/b[0], a[1]/b[1], a[2]/b[2]];
                    let res = <Vec<Complex<$t>> as ArgminDiv<Vec<Complex<$t>>, Vec<Complex<$t>>>>::div(&a, &b);
                    for i in 0..3 {
                        assert_relative_eq!(target[i].re as f64, res[i].re as f64, epsilon = std::f64::EPSILON);
                        assert_relative_eq!(target[i].im as f64, res[i].im as f64, epsilon = std::f64::EPSILON);
                    }
                }
            }

            item! {
                #[test]
                #[should_panic]
                fn [<test_div_vec_vec_panic_ $t>]() {
                    let a = vec![1 as $t, 4 as $t];
                    let b = vec![41 as $t, 38 as $t, 34 as $t];
                    <Vec<$t> as ArgminDiv<Vec<$t>, Vec<$t>>>::div(&a, &b);
                }
            }

            item! {
                #[test]
                #[should_panic]
                fn [<test_div_vec_vec_panic_2_ $t>]() {
                    let a = vec![];
                    let b = vec![41 as $t, 38 as $t, 34 as $t];
                    <Vec<$t> as ArgminDiv<Vec<$t>, Vec<$t>>>::div(&a, &b);
                }
            }

            item! {
                #[test]
                #[should_panic]
                fn [<test_div_vec_vec_panic_3_ $t>]() {
                    let a = vec![41 as $t, 38 as $t, 34 as $t];
                    let b = vec![];
                    <Vec<$t> as ArgminDiv<Vec<$t>, Vec<$t>>>::div(&a, &b);
                }
            }

            item! {
                #[test]
                fn [<test_div_mat_mat_ $t>]() {
                    let a = vec![
                        vec![4 as $t, 12 as $t, 8 as $t],
                        vec![9 as $t, 20 as $t, 45 as $t]
                    ];
                    let b = vec![
                        vec![2 as $t, 3 as $t, 4 as $t],
                        vec![3 as $t, 4 as $t, 5 as $t]
                    ];
                    let target = vec![
                        vec![2 as $t, 4 as $t, 2 as $t],
                        vec![3 as $t, 5 as $t, 9 as $t]
                    ];
                    let res = <Vec<Vec<$t>> as ArgminDiv<Vec<Vec<$t>>, Vec<Vec<$t>>>>::div(&a, &b);
                    for i in 0..3 {
                        for j in 0..2 {
                            assert_relative_eq!(target[j][i] as f64, res[j][i] as f64, epsilon = std::f64::EPSILON);
                        }
                    }
                }
            }

            item! {
                #[test]
                fn [<test_div_mat_mat_complex_ $t>]() {
                    let a = vec![
                        vec![
                            Complex::new(8 as $t, 7 as $t),
                            Complex::new(7 as $t, 6 as $t),
                            Complex::new(6 as $t, 5 as $t)
                        ],
                        vec![
                            Complex::new(9 as $t, 6 as $t),
                            Complex::new(7 as $t, 6 as $t),
                            Complex::new(6 as $t, 4 as $t)
                        ],
                    ];
                    let b = vec![
                        vec![
                            Complex::new(4 as $t, 2 as $t),
                            Complex::new(4 as $t, 3 as $t),
                            Complex::new(3 as $t, 2 as $t),
                        ],
                        vec![
                            Complex::new(3 as $t, 2 as $t),
                            Complex::new(4 as $t, 3 as $t),
                            Complex::new(3 as $t, 1 as $t),
                        ],
                    ];
                    let target = vec![
                        vec![a[0][0]/b[0][0], a[0][1]/b[0][1], a[0][2]/b[0][2]],
                        vec![a[1][0]/b[1][0], a[1][1]/b[1][1], a[1][2]/b[1][2]]
                    ];
                    let res = <Vec<Vec<Complex<$t>>> as ArgminDiv<Vec<Vec<Complex<$t>>>, Vec<Vec<Complex<$t>>>>>::div(&a, &b);
                    for i in 0..3 {
                        for j in 0..2 {
                            assert_relative_eq!(target[j][i].re as f64, res[j][i].re as f64, epsilon = std::f64::EPSILON);
                            assert_relative_eq!(target[j][i].im as f64, res[j][i].im as f64, epsilon = std::f64::EPSILON);
                        }
                    }
                }
            }

            item! {
                #[test]
                #[should_panic]
                fn [<test_div_mat_mat_panic_1_ $t>]() {
                    let a = vec![
                        vec![1 as $t, 4 as $t, 8 as $t],
                        vec![2 as $t, 9 as $t]
                    ];
                    let b = vec![
                        vec![41 as $t, 38 as $t, 34 as $t],
                        vec![40 as $t, 37 as $t, 33 as $t]
                    ];
                    <Vec<Vec<$t>> as ArgminDiv<Vec<Vec<$t>>, Vec<Vec<$t>>>>::div(&a, &b);
                }
            }

            item! {
                #[test]
                #[should_panic]
                fn [<test_div_mat_mat_panic_2_ $t>]() {
                    let a = vec![
                        vec![1 as $t, 4 as $t, 8 as $t],
                        vec![2 as $t, 5 as $t, 9 as $t]
                    ];
                    let b = vec![
                        vec![41 as $t, 38 as $t, 34 as $t],
                    ];
                    <Vec<Vec<$t>> as ArgminDiv<Vec<Vec<$t>>, Vec<Vec<$t>>>>::div(&a, &b);
                }
            }

            item! {
                #[test]
                #[should_panic]
                fn [<test_div_mat_mat_panic_3_ $t>]() {
                    let a = vec![
                        vec![1 as $t, 4 as $t, 8 as $t],
                        vec![2 as $t, 5 as $t, 9 as $t]
                    ];
                    let b = vec![];
                    <Vec<Vec<$t>> as ArgminDiv<Vec<Vec<$t>>, Vec<Vec<$t>>>>::div(&a, &b);
                }
            }
        };
    }

    make_test!(isize);
    make_test!(usize);
    make_test!(i8);
    make_test!(u8);
    make_test!(i16);
    make_test!(u16);
    make_test!(i32);
    make_test!(u32);
    make_test!(i64);
    make_test!(u64);
    make_test!(f32);
    make_test!(f64);
}
