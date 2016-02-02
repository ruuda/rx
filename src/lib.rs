// Rx -- Reactive programming for Rust
// Copyright 2016 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

#![feature(fn_traits, unboxed_closures)]

use std::iter::IntoIterator;

mod observer;

pub use observer::{Observer, PanickingObserver};

trait Observable {
    type Item;
    type Error;
    type Subscription: Drop;

    fn subscribe<O>(&mut self, observer: O) -> Self::Subscription
        where O: Observer<Self::Item, Self::Error>;
}

struct UncancellableSubscription;

impl Drop for UncancellableSubscription {
    fn drop(&mut self) { }
}

impl<'i, I> Observable for &'i I where &'i I: IntoIterator {
    type Item = <&'i I as IntoIterator>::Item;
    type Error = ();
    type Subscription = UncancellableSubscription;

    fn subscribe<O>(&mut self, mut observer: O) -> UncancellableSubscription
        where O: Observer<Self::Item, Self::Error> {
        for x in self.into_iter() {
            observer.on_next(x);
        }
        observer.on_completed();
        UncancellableSubscription
    }
}

#[test]
fn it_works() {
    let mut values = &[2u8, 3, 5, 7, 11, 13];
    values.subscribe(|x| println!("{:?}", x));
}
