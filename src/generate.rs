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
/// TODO: Actually, `Err(error)` can already be used for this. This should be removed.
pub fn error<T: Clone, E: Clone>(error: E) -> ErrorObservable<T, E> {
    ErrorObservable {
        _phantom_t: PhantomData,
        error: error,
    }
}

/// An observable that never pushes a value and never completes.
pub struct Never<T: Clone, E: Clone> {
    _phantom_t: PhantomData<T>,
    _phantom_e: PhantomData<E>,
}

/// The result of subscribing to a never observable.
///
/// Note that dropping this subscription has no effect, as a never observable
/// never pushes a value anyway.
pub struct NeverSubscription;

impl<T: Clone, E: Clone> Never<T, E> {
    /// Creates an observable that never pushes a value and never completes.
    pub fn new() -> Never<T, E> {
        Never {
            _phantom_t: PhantomData,
            _phantom_e: PhantomData,
        }
    }
}

impl<T: Clone, E: Clone> Observable for Never<T, E> {
    type Item = T;
    type Error = E;
    type Subscription = NeverSubscription;

    fn subscribe<O>(&mut self, _observer: O) -> Self::Subscription
        where O: Observer<Self::Item, Self::Error> {
        // Do nothing, forget about the observer.
        NeverSubscription
    }
}

impl Drop for NeverSubscription {
    fn drop(&mut self) {
        // This is a no-op.
    }
}
