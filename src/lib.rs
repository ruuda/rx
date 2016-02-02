// Rx -- Reactive programming for Rust
// Copyright 2016 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

#![feature(fn_traits, unboxed_closures)]

use observer::{NextObserver, CompletedObserver, ErrorObserver};
use std::fmt::Debug;
use std::iter::IntoIterator;

mod observer;

pub use observer::{Observer, PanickingObserver};

trait Observable {
    type Item;
    type Error;
    type Subscription: Drop;

    fn subscribe<O>(&mut self, observer: O) -> Self::Subscription
        where O: Observer<Self::Item, Self::Error>;

    fn subscribe_next<FnNext>(&mut self,
                              on_next: FnNext)
                              -> Self::Subscription
        where Self::Error: Debug,
              FnNext: FnMut(Self::Item) {
        let observer = NextObserver {
            fn_next: on_next,
        };
        self.subscribe(observer)
    }

    fn subscribe_completed<FnNext, FnCompleted>(&mut self,
                                                on_next: FnNext,
                                                on_completed: FnCompleted)
                                                -> Self::Subscription
        where Self::Error: Debug,
              FnNext: FnMut(Self::Item),
              FnCompleted: FnOnce() {
        let observer = CompletedObserver {
            fn_next: on_next,
            fn_completed: on_completed,
        };
        self.subscribe(observer)
    }

    fn subscribe_error<FnNext, FnCompleted, FnError>(&mut self,
                                                     on_next: FnNext,
                                                     on_completed: FnCompleted,
                                                     on_error: FnError)
                                                     -> Self::Subscription
        where FnNext: FnMut(Self::Item),
              FnCompleted: FnOnce(),
              FnError: FnOnce(Self::Error) {
        let observer = ErrorObserver {
            fn_next: on_next,
            fn_completed: on_completed,
            fn_error: on_error,
        };
        self.subscribe(observer)
    }
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
fn subscribe_to_slice() {
    let mut values = &[2u8, 3, 5, 7, 11, 13];
    values.subscribe(|x| println!("{:?}", x));
}

#[test]
fn subscribe_next_slice() {
    let mut values = &[2u8, 3, 5, 7, 11, 13];
    let mut received = Vec::new();
    values.subscribe_next(|&x| received.push(x));
    assert_eq!(&values[..], &received[..]);
}
