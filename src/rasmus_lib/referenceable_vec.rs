// use std::slice::Iter;
// use std::slice::IterMut;
// use std::vec::IntoIter;

// pub struct Reference<T> {
//     index: usize,
// }

// pub struct ReferenceVec<T> {
//     items: Vec<T>,
// }

// impl<T> ReferenceVec<T> {
//     pub fn new(items: Vec<T>) -> Self {
//         Self { items }
//     }

//     pub fn create_reference_for(index: usize) -> Reference<T> {}

//     pub fn len(&self) -> usize {
//         self.items.len()
//     }

//     pub fn iter(&self) -> Iter<T> {
//         self.items.iter()
//     }
//     pub fn iter_mut(&self) -> IterMut<T> {
//         self.items.iter_mut()
//     }
//     pub fn iter_mut(&self) -> IntoIter<T> {
//         self.items.into_iter()
//     }
// }

pub trait ReferenceableVec<T> {
    fn create_reference_for(index: usize) -> Index<T>;
    fn get_with(&self, index: &Index<T>) -> Option<&T>;
    fn get_with_mut(&mut self, index: &Index<T>) -> Option<&mut T>;
}
impl<T> ReferenceableVec<T> for Vec<T> {
    fn create_reference_for(index: usize) -> Index<T> {
        Index::new(index)
    }
    fn get_with(&self, index: &Index<T>) -> Option<&T> {
        self.get(index.0)
    }
    fn get_with_mut(&mut self, index: &Index<T>) -> Option<&mut T> {
        self.get_mut(index.0)
    }
}

impl<T> ReferenceableVec<T> for [T] {
    fn create_reference_for(index: usize) -> Index<T> {
        Index::new(index)
    }
    fn get_with(&self, index: &Index<T>) -> Option<&T> {
        self.get(index.0)
    }
    fn get_with_mut(&mut self, index: &Index<T>) -> Option<&mut T> {
        self.get_mut(index.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub struct Index<T>(usize, std::marker::PhantomData<T>);
impl<T> Index<T> {
    pub const fn new(index: usize) -> Self {
        Self(index, std::marker::PhantomData)
    }
}
