// Rx -- Reactive programming for Rust
// Copyright 2016 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

use observable::Observable;
use observer::Observer;
use std::marker::PhantomData;

/// An observable that produces an error when subscribed to.
pub struct ErrorObservable<T: Clone, E: Clone> {
    _phantom_t: PhantomData<T>,
    error: E,
}

impl<T: Clone, E: Clone> Observable for ErrorObservable<T, E> {
    type Item = T;
    type Error = E;
    type Subscription = super::UncancellableSubscription;

    fn subscribe<O>(&mut self, observer: O) -> Self::Subscription
        where O: Observer<Self::Item, Self::Error> {
        observer.on_error(self.error.clone());
        super::UncancellableSubscription
    }
}

/// Returns an observable that fails with the given error when subscribed to.
///
/// On its own an observable that always fails has limited use, but it can be
/// convenient when combining observables.
///
/// TODO: Add example once `continue_with` has been implemented.
pub fn error<T: Clone, E: Clone>(error: E) -> ErrorObservable<T, E> {
    ErrorObservable {
        _phantom_t: PhantomData,
        error: error,
    }
}
