macro_rules! assert_equal { ($a:expr, $b:expr) => {
    let a_value = $a;
    let b_value = $b;
    assert!(a_value == b_value, "expected {} == {}, but {} != {}", stringify!($a), stringify!($b), a_value, b_value);
}}
pub(crate) use assert_equal;

macro_rules! assert_true { ($predicate:expr, $extra_message:expr) => {
    assert!($predicate, "expected {} to be true, but got false; {}", stringify!($predicate), $extra_message);
}}
pub(crate) use assert_true;

macro_rules! assert_false { ($predicate:expr, $extra_message:expr) => {
    assert!(!$predicate, "expected {} to be false, but got true; {}", stringify!($predicate), $extra_message);
}}
pub(crate) use assert_false;