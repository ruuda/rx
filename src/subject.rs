// Rx -- Reactive programming for Rust
// Copyright 2016 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

use observable::Observable;
use observer::{Observer, BoxedObserver};

/// Both an observer and observable.
///
/// A subject is a low-level primitive for creating observables.
///
/// TODO: Add example.
pub struct Subject<T, E> {
    observers: Vec<Box<BoxedObserver<T, E>>>,
}

/// Proxy object that exposes the observable part of a subject.
pub struct SubjectObservable<'s, T: 's, E: 's> {
    subject: &'s mut Subject<T, E>,
}

impl<T, E> Subject<T, E> {
    /// Creates a new subject.
    pub fn new() -> Subject<T, E> {
        Subject {
            observers: Vec::new(),
        }
    }

    /// Returns a proxy object that exposes the observable part of a subject.
    ///
    /// This can be used to avoid exposing the observer methods while still
    /// allowing subscription. When a subject is used internally as the source
    /// of an observable, a getter can expose the `observable()` of the subject.
    pub fn observable<'s>(&'s mut self) -> SubjectObservable<'s, T, E> {
        SubjectObservable {
            subject: self,
        }
    }
}

impl<T: Clone, E: Clone> Observer<T, E> for Subject<T, E> {
    fn on_next(&mut self, item: T) {
        for observer in &mut self.observers {
            observer.on_next(item.clone());
        }
    }

    fn on_completed(mut self) {
        for observer in self.observers.drain(..) {
            observer.on_completed_box();
        }
    }

    fn on_error(mut self, error: E) {
        for observer in self.observers.drain(..) {
            observer.on_error_box(error.clone());
        }
    }
}

impl<'s, T: Clone, E: Clone> Observable for SubjectObservable<'s, T, E> {
    type Item = T;
    type Error = E;
    type Subscription = super::UncancellableSubscription; // TODO: Make it cancellable.

    fn subscribe<O: 'static>(&mut self, observer: O) -> Self::Subscription
        where O: Observer<Self::Item, Self::Error> {
        let boxed: Box<BoxedObserver<T, E>> = Box::new(observer);
        self.subject.observers.push(boxed);
        super::UncancellableSubscription
    }
}
