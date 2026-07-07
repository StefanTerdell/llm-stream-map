/// Helper trait to join two options of a type T that implements Add together, or pick the only one with a value, or None.
use std::ops::Add;

pub trait OrAdd {
    fn or_add(self, rhs: Self) -> Self;
}

impl<T: Add<Output = T>> OrAdd for Option<T> {
    fn or_add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Some(a), Some(b)) => Some(a + b),
            (a, b) => a.or(b),
        }
    }
}
