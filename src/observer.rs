// Rx -- Reactive programming for Rust
// Copyright 2016 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

use std::fmt::Debug;

pub trait Observer<T, E> {
    /// Provides the observer with new data.
    fn on_next(&mut self, item: T);

    /// Notifies the observer that the provider has finished sending notifications.
    fn on_completed(self);

    /// Notifies the observer that the provider experienced an error condition.
    fn on_error(self, error: E);
}

pub trait PanickingObserver<T> {
    /// Provides the observer with new data.
    fn on_next(&mut self, item: T);

    /// Notifies the observer that the provider has finished sending notifications.
    fn on_completed(self);

    /// Creates an `Observer` out of a `PanickingObserver`.
    ///
    /// An `on_error()` call to the observer is handled by panicking.
    fn unwrap(self) -> UnwrappingObserver<Self> where Self: Sized {
        UnwrappingObserver {
            panicking_observer: self
        }
    }
}

pub struct UnwrappingObserver<PO: Sized> {
    panicking_observer: PO
}

impl<T, E: Debug, PO: PanickingObserver<T>> Observer<T, E> for UnwrappingObserver<PO> {
    fn on_next(&mut self, item: T) {
        self.panicking_observer.on_next(item);
    }

    fn on_completed(self) {
        self.panicking_observer.on_completed();
    }

    fn on_error(self, error: E) {
        panic!("panicking observer received error: {:?}", error);
    }
}

/// Observer implementation for functions.
///
/// Observer behavior translates into a function all as follows:
///
///  * `on_next(item)` calls the function with `Some(Ok(item))`.
///  * `on_completed()` calls the function with `None`.
///    The function will not be called after that.
///  * `on_error(err)` calls the function with `Some(Err(error))`.
///    The function will not be called after that.
impl<F, T, E> Observer<T, E> for F where F: FnMut(Option<Result<T, E>>) {
    fn on_next(&mut self, item: T) {
        self.call_mut((Some(Ok(item)),));
    }

    fn on_completed(self) {
        self.call_once((None,));
    }

    fn on_error(self, error: E) {
        self.call_once((Some(Err(error)),));
    }
}

/// Observer implementation for functions ignoring error handling.
///
/// Observer behavior translates into a function all as follows:
///
///  * `on_next(item)` calls the function with `Some(Ok(item))`.
///  * `on_completed()` calls the function with `None`.
///    The function will not be called after that.
impl<F, T> PanickingObserver<T> for F where F: FnMut(Option<T>) {
    fn on_next(&mut self, item: T) {
        self.call_mut((Some(item),));
    }

    fn on_completed(self) {
        self.call_once((None,));
    }
}
