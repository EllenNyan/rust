#![feature(const_generics, const_evaluatable_checked)]
#![allow(incomplete_features)]

// This tests that during error handling for the "trait not implemented" error
// we dont try to evaluate std::mem::size_of::<Self::Assoc> causing an ICE

struct ADT;

trait Foo {
    type Assoc;
    fn foo()
    where
        [ADT; std::mem::size_of::<Self::Assoc>()]: ,
    {
        <[ADT; std::mem::size_of::<Self::Assoc>()] as Foo>::bar()
        //~^ Error: the trait bound
    }

    fn bar() {}
}

fn main() {}
