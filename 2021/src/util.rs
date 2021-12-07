/// Recreation of the abs_diff function to allow using it while staying on stable
pub fn abs_diff<T, U>(a: T, b: T) -> U
where
    T: std::cmp::Ord + std::ops::Sub<Output = U>,
{
    if a < b {
        b - a
    } else {
        a - b
    }
}
