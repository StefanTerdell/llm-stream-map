use indexmap::IndexMap;
use std::hash::Hash;

pub trait Merge<R = Self> {
    fn merge(self, rhs: R) -> Self;
}

impl<T: Merge<T>> Merge<Option<T>> for T {
    fn merge(self, other_opt: Option<T>) -> Self {
        if let Some(other) = other_opt {
            self.merge(other)
        } else {
            self
        }
    }
}

impl<K: Eq + Hash, V: Merge<Option<V>>> Merge for IndexMap<K, V> {
    fn merge(self, mut rhs: Self) -> Self {
        let mut m = self
            .into_iter()
            .map(|(k, a)| {
                let v = a.merge(rhs.swap_remove(&k));

                (k, v)
            })
            .collect::<IndexMap<_, _>>();

        m.append(&mut rhs);

        m
    }
}

impl Merge for String {
    fn merge(self, rhs: Self) -> Self {
        format!("{self}{rhs}")
    }
}

impl<T> Merge for Vec<T> {
    fn merge(mut self, mut rhs: Self) -> Self {
        self.append(&mut rhs);
        self
    }
}
