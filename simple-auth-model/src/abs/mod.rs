use std::str::FromStr;

pub(crate) trait IsValid<T> {
    fn is_valid(value: T) -> bool;
}