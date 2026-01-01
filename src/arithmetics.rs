pub mod have_same_sign;
pub mod partial_sum;

/// Examples:
/// ```
/// use crate::bb_qol::arithmetics::have_same_sign;
/// let a: i32 = 2;
/// let b: i32 = 3;
/// let c: f64 = 0.0;
/// let d: f64 = -3.14;
/// assert!(have_same_sign(a, b));
/// assert!(!have_same_sign(c, d));
/// assert!(!have_same_sign(1, 0));
/// assert!(have_same_sign(0, 0))
/// ```
pub fn have_same_sign<T>(a: T, b: T) -> bool
where
    T: PartialOrd + Default
{
    let zero: T = Default::default();
    ( a > zero && b > zero )
        || ( a < zero && b < zero )
        || ( a == zero && b == zero)
}

