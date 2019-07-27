extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigInt;
use num_traits::One;
use num_traits::Zero;
use num_traits::pow;
use std::cmp::Ordering;
use std::fmt;
use std::ops; 

// coef x^xpow y^ypow
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Unit {
    pub coef: BigInt,
    pub xpow: BigInt,
    pub ypow: BigInt,
}

impl_op_ex!(* |a: &Unit, b: &Unit| -> Unit {
    Unit {
        coef: &a.coef * &b.coef,
        xpow: &a.xpow + &b.xpow,
        ypow: &a.ypow + &b.ypow,
    }
});

impl_op_ex!(/ |a: &Unit, b: &Unit| -> Unit {
    Unit {
        coef: &a.coef / &b.coef,
        xpow: &a.xpow - &b.xpow,
        ypow: &a.ypow - &b.ypow,
    }
});

impl_op_ex!(- |a: &Unit| -> Unit {
    Unit {
        coef: -&a.coef,
        xpow: a.xpow.clone(),
        ypow: a.ypow.clone(),
    }
});

impl Unit {
    pub fn equal_order(&self, other: &Self) -> bool {
        return &self.xpow == &other.xpow && self.ypow == other.ypow 
    }
    pub fn power(&self, val: usize) -> Self {
        let coef = pow(self.coef.clone(), val);
        let xpow = &self.xpow * val;
        let ypow = &self.ypow * val;
        Unit {
            coef: coef,
            xpow: xpow,
            ypow: ypow,
        }
    }
    pub fn to_frob(&self, val: usize) -> Self {
        Unit {
            coef: self.coef.clone(),
            xpow: &self.xpow * val,
            ypow: &self.ypow * val,
        }
    }
    pub fn modular(&self, val: BigInt) -> Self {
        Unit {
            coef: &self.coef % val,
            xpow: self.xpow.clone(),
            ypow: self.ypow.clone(),
        }
    }
    pub fn is_zero(&self) -> bool {
        self.coef == Zero::zero()
    }

    pub fn has_y(&self) -> bool {
        self.ypow != Zero::zero()
    }
}

impl Ord for Unit {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.xpow < other.xpow {
            return Ordering::Less;
        } else if self.xpow > other.xpow {
            return Ordering::Greater;
        }
        if self.ypow < other.ypow {
            return Ordering::Less;
        } else if self.ypow > other.ypow {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.coef == One::one() {
            if self.xpow == Zero::zero() && self.ypow == Zero::zero() {
                write!(f, "1")
            } else {
                let mut st = String::new();
                if self.xpow != Zero::zero() {
                    if self.xpow == One::one() {
                        st.push_str("x");
                    } else {
                        st.push_str("x^");
                        st.push_str(&self.xpow.to_string());
                    }
                    st.push_str(" ");
                }
                if self.ypow != Zero::zero() {
                    if self.ypow == One::one() {
                        st.push_str("y");
                    } else {
                        st.push_str("y^");
                        st.push_str(&self.ypow.to_string());
                    }
                }
                write!(f, "{}", st.trim_end())
            }
        } else if self.coef == BigInt::from(-1) {
            if self.xpow == Zero::zero() && self.ypow == Zero::zero() {
                write!(f, "- 1")
            } else {
                let mut st = String::new();
                st.push_str("- ");
                if self.xpow != Zero::zero() {
                    if self.xpow == One::one() {
                        st.push_str("x");
                    } else {
                        st.push_str("x^");
                        st.push_str(&self.xpow.to_string());
                    }
                    st.push_str(" ");
                }
                if self.ypow != Zero::zero() {
                    if self.ypow == One::one() {
                        st.push_str("y");
                    } else {
                        st.push_str("y^");
                        st.push_str(&self.ypow.to_string());
                    }
                }
                write!(f, "{}", st.trim_end())
            }

        } else {
            let mut st = String::new();
            if self.coef >= Zero::zero() {
                st.push_str(&self.coef.to_string());
            } else {
                let abs_coef = &self.coef * BigInt::from(-1);
                st.push_str("- ");
                st.push_str(&abs_coef.to_string());
            }
            st.push_str(" ");
            if self.xpow != Zero::zero() {
                if self.xpow == One::one() {
                    st.push_str("x");
                } else {
                    st.push_str("x^");
                    st.push_str(&self.xpow.to_string());
                }
                st.push_str(" ");
            }
            if self.ypow != Zero::zero() {
                if self.ypow == One::one() {
                    st.push_str("y");
                } else {
                    st.push_str("y^");
                    st.push_str(&self.ypow.to_string());
                }
            }
            write!(f, "{}", st.trim_end())
        }
    }
}

#[test]
fn unit_test() {
    let u0: Unit = Default::default();
    assert_eq!(&u0.to_string(), "0");

    let u1 = Unit {
        coef: BigInt::from(3),
        .. Default::default()
    };
    assert_eq!(&u1.to_string(), "3");

    let u2 = Unit {
        coef: BigInt::from(4),
        .. Default::default()
    };
    let u3 = Unit {
        coef: BigInt::from(3),
        .. Default::default()
    };
    assert_eq!((u2.clone() * u3.clone()).to_string(), "12");
    assert_eq!((&u2 * &u3).to_string(), "12");
}


