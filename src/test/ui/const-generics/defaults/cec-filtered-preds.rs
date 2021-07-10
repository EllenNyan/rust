#![feature(const_generics, const_evaluatable_checked, const_generics_defaults, const_trait_impl)]
#![allow(incomplete_features)]

pub trait Foo<const N: usize> { const Assoc: usize; }
pub struct Bar<const N: usize, const M: usize = { <()>::Assoc }> where (): Foo<M>;
//~^ ERROR: no associated item named `Assoc` found for unit type `()` in the current scope

pub trait MakeAssoc {
    type Assoc;
    fn make_assoc() -> Self::Assoc;
}
impl const MakeAssoc for () {
    type Assoc = usize;
    fn make_assoc() -> Self::Assoc {
        10
    }
}
pub struct FwdDeclaredViaAssocTyBound<
    T,
    const N: usize = { <() as MakeAssoc>::make_assoc() },
    U = T,
>(T, U) where (): MakeAssoc<Assoc = U>;
// If we fail to filter the projection pred out then we'll get an error
// expected `usize`, found type parameter `U`

fn main() {}
