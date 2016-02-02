// Rx -- Reactive programming for Rust
// Copyright 2016 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

use observer::{Observer, BoxedObserver};

/// Both an observer and observable.
///
/// A subject is a low-level primitive for creating observables.
///
/// TODO: Flesh out how this will work; if it is a memer, it likely should not expose the
/// observer methods but only the observable methods. Have a proxy object?
pub struct Subject<T, E> {
    observers: Vec<Box<BoxedObserver<T, E>>>,
}

impl<T, E> Subject<T, E> {
    /// Creates a new subject.
    pub fn new() -> Subject<T, E> {
        Subject {
            observers: Vec::new(),
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
