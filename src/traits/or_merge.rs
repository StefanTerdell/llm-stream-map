use crate::traits::merge::Merge;

pub trait OrMerge<R = Self> {
    fn or_merge(self, other: R) -> R;
}

impl<T: Merge> OrMerge for Option<T> {
    fn or_merge(self, other: Self) -> Self {
        match (self, other) {
            (Some(a), Some(b)) => Some(a.merge(b)),
            (a, b) => a.or(b),
        }
    }
}

impl<T: Merge> OrMerge<T> for Option<T> {
    fn or_merge(self, b: T) -> T {
        if let Some(a) = self { a.merge(b) } else { b }
    }
}
