use crate::common::has_kind::HasKind;

pub trait Subscriber<T>
where
    T: HasKind,
{
    fn subscriptions(&self) -> &'static [T::Kind];
    fn priority(&self, kind: &T::Kind) -> i32;
}
