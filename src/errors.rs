macro_rules! assert_equal { ($a:expr, $b:expr) => {
    let a_value = $a;
    let b_value = $b;
    assert!(a_value == b_value, "expected {} == {}, but {} != {}", stringify!($a), stringify!($b), a_value, b_value);
}}
pub(crate) use assert_equal;

macro_rules! assert_less { ($a:expr, $b:expr) => {
    let a_value = $a;
    let b_value = $b;
    assert!(a_value < b_value, "expected {} < {}, but {} >= {}", stringify!($a), stringify!($b), a_value, b_value);
}}
pub(crate) use assert_less;

macro_rules! assert_more { ($a:expr, $b:expr) => {
    let a_value = $a;
    let b_value = $b;
    assert!(a_value > b_value, "expected {} > {}, but {} <= {}", stringify!($a), stringify!($b), a_value, b_value);
}}
pub(crate) use assert_more;