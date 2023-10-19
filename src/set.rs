use crate::Set;

impl<T> Default for Set<T> {
    fn default() -> Self {
        Self { vec: Default::default() }
    }
}

impl<T> Set<T> {
    pub fn insert(&mut self, t: T) -> Option<T>
    where
        T: Ord,
    {
        match self.vec.binary_search(&t) {
            Ok(index) => Some(std::mem::replace(&mut self.vec[index], t)),
            Err(index) => {
                self.vec.insert(index, t);
                None
            }
        }
    }

    pub fn contains(&self, t: &T) -> bool
    where
        T: Ord,
    {
        self.find(t).is_some()
    }

    pub fn find(&self, t: &T) -> Option<&T>
    where
        T: Ord,
    {
        if let Ok(index) = self.vec.binary_search(&t) {
            return self.vec.get(index);
        }
        None
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.vec.iter()
    }
}
