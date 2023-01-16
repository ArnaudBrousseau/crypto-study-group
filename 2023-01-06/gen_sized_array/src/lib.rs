use std::fmt::Debug;

/// Uses const generics to wrap an array
/// See https://practice.rs/generics-traits/const-generics.html
pub struct GenArrayWrapper<T: Default + Copy + Debug, const SIZE: usize> {
    inner: [T; SIZE],
}

impl<T: Default + Copy + Debug, const SIZE: usize> GenArrayWrapper<T, SIZE> {
    pub fn new() -> Self {
        Self {
            inner: [T::default(); SIZE],
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_index(&self, index: usize) -> Option<&T> {
        self.inner.get(index)
    }

    pub fn set_index(&mut self, index: usize, val: T) -> Option<T> {
        if let Some(old) = self.inner.get_mut(index) {
            *old = val;
            return Some(val);
        }
        None
    }
}

impl<T: Default + Copy + Debug, const SIZE: usize> Default for GenArrayWrapper<T, SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use crate::GenArrayWrapper;

    #[test]
    fn test_len_and_is_empty() {
        let array = GenArrayWrapper::<u32, 8>::new();
        assert_eq!(array.len(), 8);
        assert!(!array.is_empty());
    }

    #[test]
    fn test_set_and_get_index() {
        let mut array = GenArrayWrapper::<u8, 10>::new();
        let set_res = array.set_index(1, 42);
        assert!(set_res.is_some());
        assert_eq!(set_res.unwrap(), 42);

        let get_res = array.get_index(1);
        assert!(get_res.is_some());
        assert_eq!(get_res.unwrap(), &42);
    }

    #[test]
    fn test_outside_of_bounds() {
        let mut array = GenArrayWrapper::<u8, 10>::new();
        let set_res = array.set_index(11, 42);
        assert!(set_res.is_none());

        let get_res = array.get_index(11);
        assert!(get_res.is_none());
    }
}
