use Invoke;
use ValueNode;

pub(crate) trait Push {
    type Element: TypedIndex;
    fn index_push(&mut self, element: Self::Element) -> <Self::Element as TypedIndex>::Index;
}

impl<T> Push for Vec<T>
where
    T: TypedIndex,
{
    type Element = T;

    fn index_push(&mut self, element: T) -> T::Index {
        self.push(element);
        T::index(self.len() - 1)
    }
}

pub(crate) trait TypedIndex {
    type Index;
    fn index(i: usize) -> Self::Index;
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct ValueRef(pub usize);

impl TypedIndex for ValueNode {
    type Index = ValueRef;

    fn index(i: usize) -> Self::Index {
        ValueRef(i)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct InvokeRef(pub usize);

impl TypedIndex for Invoke {
    type Index = InvokeRef;

    fn index(i: usize) -> Self::Index {
        InvokeRef(i)
    }
}
