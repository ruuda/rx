// Rx -- Reactive programming for Rust
// Copyright 2016 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

//! Rx, a library for reactive programming in Rust.
//! Inspired by [Reactive Extensions for C#](https://rx.codeplex.com/).
//!
//! TODO: Add intro to observables for people who are unfamiliar with them.
//!
//! TODO: Add examples.
//!
//! # Simple Observables
//!
//! The `Option` and `Result` types implement `Observable`. These can be used
//! to create empty or singleton observables.
//!
//!  * `Some(x)` and `Ok(x)` both produce `x` upon subscription and complete
//!    immediately.
//!  * `None` does not produce any value and completes immediately upon
//!    subscription.
//!  * `Err(err)` fails with `err` upon subscription.
//!
//! Note that subscribing to an option or result does not modify it in any way.
//! (Subscribing requires a mutable reference, because in general, the
//! observable might want to store the observer.) For instance, the following
//! example prints “received 7” twice.
//!
//! ```
//! use rx::Observable;
//! let mut some = Some(7);
//! some.subscribe_next(|x| println!("received {}", x));
//! some.subscribe_next(|x| println!("received {}", x));
//! ```
//!
//! TODO: Could I have an `ImmutableObservable` to get rid of the mutability
//! requirement?

#![warn(missing_docs)]
#![feature(fn_traits, unboxed_closures)]

use std::iter::IntoIterator;

mod generate;
mod observable;
mod observer;
mod subject;
mod transform;

pub use generate::error;
pub use observable::Observable;
pub use observer::Observer;
pub use subject::Subject;

/// A subscription where `drop()` is a no-op.
pub struct UncancellableSubscription;

impl Drop for UncancellableSubscription {
    fn drop(&mut self) { }
}

/// Observable implementation for types that can be converted into an iterator.
///
/// Upon subscription, this pushes a value for every value returned by the
/// iterator and then completes (if the iterator is finite). The returned
/// subscription is not cancellable: if the observable completes, it completes
/// before the call to `subscribe()` returns. This observable does not fail.
impl<'i, I> Observable for &'i I where &'i I: IntoIterator, <&'i I as IntoIterator>::Item: Clone {
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

/// Observable implementation for `Result`.
///
/// Upon subscription, this pushes either the result and completes, or the
/// observable fails with the error. The returned subscription is not
/// cancellable: the observable completes or fails before the call to
/// `subscribe()` returns.
impl<T: Clone, E: Clone> Observable for Result<T, E> {
    type Item = T;
    type Error = E;
    type Subscription = UncancellableSubscription;

    fn subscribe<O>(&mut self, mut observer: O) -> UncancellableSubscription
        where O: Observer<Self::Item, Self::Error> {
        match *self {
            Ok(ref item) => {
                observer.on_next(item.clone());
                observer.on_completed();
            }
            Err(ref error) => {
                observer.on_error(error.clone());
            }
        }
        UncancellableSubscription
    }
}

/// Observable implementation for `Option`.
///
/// Upon subscription, this pushes the value if the option is `Some`, and then
/// completes. If the option is `None` it completes immediately. The returned
/// subscription is not cancellable: the observable completes before the call to
/// `subscribe()` returns. This observable does not fail.
impl<T: Clone> Observable for Option<T> {
    type Item = T;
    type Error = ();
    type Subscription = UncancellableSubscription;

    fn subscribe<O>(&mut self, mut observer: O) -> UncancellableSubscription
        where O: Observer<Self::Item, Self::Error> {
        if let Some(ref item) = *self {
            observer.on_next(item.clone());
        }
        observer.on_completed();
        UncancellableSubscription
    }
}
