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

pub struct NextObserver<FnNext> {
    pub fn_next: FnNext,
}

pub struct CompletedObserver<FnNext, FnCompleted> {
    pub fn_next: FnNext,
    pub fn_completed: FnCompleted,
}

pub struct ErrorObserver<FnNext, FnCompleted, FnError> {
    pub fn_next: FnNext,
    pub fn_completed: FnCompleted,
    pub fn_error: FnError,
}

pub struct OptionObserver<FnOption> {
    pub fn_option: FnOption
}

pub struct ResultObserver<FnResult> {
    pub fn_result: FnResult
}

impl<T, E, FnNext> Observer<T, E> for NextObserver<FnNext>
    where E: Debug, FnNext: FnMut(T) {

    fn on_next(&mut self, item: T) {
        self.fn_next.call_mut((item,));
    }

    fn on_completed(self) {
        // Ignore completion.
    }

    fn on_error(self, error: E) {
        panic!("observer received error: {:?}", error);
    }
}

impl<T, E, FnNext, FnCompleted> Observer<T, E> for CompletedObserver<FnNext, FnCompleted>
    where E: Debug, FnNext: FnMut(T), FnCompleted: FnOnce() {

    fn on_next(&mut self, item: T) {
        self.fn_next.call_mut((item,));
    }

    fn on_completed(self) {
        self.fn_completed.call_once(());
    }

    fn on_error(self, error: E) {
        panic!("observer received error: {:?}", error);
    }
}

impl<T, E, FnNext, FnCompleted, FnError>
    Observer<T, E> for ErrorObserver<FnNext, FnCompleted, FnError>
    where FnNext: FnMut(T), FnCompleted: FnOnce(), FnError: FnOnce(E) {

    fn on_next(&mut self, item: T) {
        self.fn_next.call_mut((item,));
    }

    fn on_completed(self) {
        self.fn_completed.call_once(());
    }

    fn on_error(self, error: E) {
        self.fn_error.call_once((error,));
    }
}

impl<T, E, FnOption> Observer<T, E> for OptionObserver<FnOption>
    where E: Debug, FnOption: FnMut(Option<T>) {

    fn on_next(&mut self, item: T) {
        self.fn_option.call_mut((Some(item),));
    }

    fn on_completed(mut self) {
        self.fn_option.call_mut((None,));
    }

    fn on_error(self, error: E) {
        panic!("observer received error: {:?}", error);
    }
}

impl<T, E, FnResult> Observer<T, E> for ResultObserver<FnResult>
    where FnResult: FnMut(Result<Option<T>, E>) {

    fn on_next(&mut self, item: T) {
        self.fn_result.call_mut((Ok(Some(item)),));
    }

    fn on_completed(mut self) {
        self.fn_result.call_mut((Ok(None),));
    }

    fn on_error(mut self, error: E) {
        self.fn_result.call_mut((Err(error),));
    }
}
