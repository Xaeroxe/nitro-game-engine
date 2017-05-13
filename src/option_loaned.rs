use std::borrow::{Borrow, BorrowMut};

/// Special variant of Option that indicates a value may exist but is currently loaned out in a
/// &mut.
pub enum OptionLoaned<T> {
    Some(T),
    Loaned,
    None,
}

impl<T> From<Option<Option<T>>> for OptionLoaned<T> {
    fn from(result: Option<Option<T>>) -> OptionLoaned<T> {
        match result {
            Some(Some(result)) => OptionLoaned::Some(result),
            Some(None) => OptionLoaned::Loaned,
            None => OptionLoaned::None,
        }
    }
}

impl<'a, T, M> From<Option<&'a Option<M>>> for OptionLoaned<&'a T>
    where M: Borrow<T>
{
    fn from(result: Option<&'a Option<M>>) -> OptionLoaned<&'a T> {
        match result {
            Some(inner) => {
                match inner.as_ref() {
                    Some(inner) => OptionLoaned::Some(inner.borrow()),
                    None => OptionLoaned::Loaned,
                }
            }
            None => OptionLoaned::None,
        }
    }
}

impl<'a, T, M> From<Option<&'a mut Option<M>>> for OptionLoaned<&'a mut T>
    where M: BorrowMut<T>
{
    fn from(result: Option<&'a mut Option<M>>) -> OptionLoaned<&'a mut T> {
        match result {
            Some(inner) => {
                match inner.as_mut() {
                    Some(inner) => OptionLoaned::Some(inner.borrow_mut()),
                    None => OptionLoaned::Loaned,
                }
            }
            None => OptionLoaned::None,
        }
    }
}
