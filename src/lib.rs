// Rx -- Reactive programming for Rust
// Copyright 2016 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

#![feature(fn_traits, unboxed_closures)]

mod observer;

pub use observer::{Observer, PanickingObserver};

trait Observable {
    type Item;
    type Error;
    type Subscription: Drop;

    fn subscribe<O: Observer<Self::Item, Self::Error>>(observer: O) -> Self::Subscription;
}

#[test]
fn it_works() {

}
