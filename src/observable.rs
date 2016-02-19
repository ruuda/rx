// Rx -- Reactive programming for Rust
// Copyright 2016 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

use observer::Observer;
use observer::{NextObserver, CompletedObserver, ErrorObserver, OptionObserver, ResultObserver};
use std::fmt::Debug;
use transform::{ContinueWithObservable, MapErrorObservable, MapObservable};

/// A stream of values.
///
/// An observable represents a stream of values, much like an iterator,
/// but instead of being “pull-based” like an iterator, it is “push-based”.
/// Multiple observers can subscribe to an observable and when the observable
/// produces a value, all observers get called with this value.
///
/// An observable can be _finite_ or _infinite_. An example of an infinite
/// observable are mouse clicks: you never know if the user is going to click
/// once more. An example of a finite observable are the results of a database
/// query: a database is can hold only finitely many records, so one result is
/// the last one.
///
/// A finite observable can end in two ways:
///
///  * **Completed**: when the observable ends normally.
///    For instance, an observable of database query results
///    will complete after the last result has been produced.
///  * **Failed**: when an error occurred.
///    For instance, an observable of database query results
///    may fail if the connection is lost.
///
/// Failures are fatal: after an observable produces an error, it will not
/// produce any new values. If this is not the desired behavior, you can
/// use an observable of `Result`.
pub trait Observable {
    /// The value produced by the observable.
    type Item: Clone;

    /// The error produced if the observable fails.
    type Error: Clone;

    /// The result of subscribing an observer.
    // TODO: This drop bound is not required and it only complicates stuff, remove it.
    type Subscription: Drop;

    /// Subscribes an observer and returns the subscription.
    ///
    /// After subscription, `on_next` will be called on the observer for every
    /// value produced. If the observable completes, `on_completed` is called.
    /// If the observable fails with an error, `on_error` is called. It is
    /// guaranteed that no methods will be called on the observer after
    /// `on_completed` or `on_error` have been called.
    ///
    /// _When_ the observer is called is not part of the observable contract,
    /// it depends on the kind of observable. The observer may be called before
    /// `subscribe` returns, or it may be called in the future.
    ///
    /// The returned value represents the subscription. Dropping the subscription
    /// will prevent further calls on the observer.
    fn subscribe<O>(&mut self, observer: O) -> Self::Subscription
        where O: Observer<Self::Item, Self::Error>;

    /// Subscribes a function to handle values produced by the observable.
    ///
    /// For every value produced by the observable, `on_next` is called.
    ///
    /// **This subscription panics if the observable fails with an error.**
    ///
    /// See also [`subscribe()`](#tymethod.subscribe).
    fn subscribe_next<FnNext>(&mut self,
                              on_next: FnNext)
                              -> Self::Subscription
        where Self::Error: Debug, FnNext: FnMut(Self::Item) {
        let observer = NextObserver {
            fn_next: on_next,
        };
        self.subscribe(observer)
    }

    /// Subscribes functions to handle next and completion.
    ///
    /// For every value produced by the observable, `on_next` is called. If the
    /// observable completes, `on_completed` is called. A failure will cause a
    /// panic. After `on_completed` has been called, it is guaranteed that neither
    /// `on_next` nor `on_completed` is called again.
    ///
    /// **This subscription panics if the observable fails with an error.**
    ///
    /// See also [`subscribe()`](#tymethod.subscribe).
    fn subscribe_completed<FnNext, FnCompleted>(&mut self,
                                                on_next: FnNext,
                                                on_completed: FnCompleted)
                                                -> Self::Subscription
        where Self::Error: Debug, FnNext: FnMut(Self::Item), FnCompleted: FnOnce() {
        let observer = CompletedObserver {
            fn_next: on_next,
            fn_completed: on_completed,
        };
        self.subscribe(observer)
    }

    /// Subscribes functions to handle next, completion, and error.
    ///
    /// For every value produced by the observable, `on_next` is called. If the
    /// observable completes, `on_completed` is called. If it fails, `on_error`
    /// is called. After `on_completed` or `on_error` have been called, it is
    /// guaranteed that none of the three functions are called again.
    ///
    /// See also [`subscribe()`](#tymethod.subscribe).
    fn subscribe_error<FnNext, FnCompleted, FnError>(&mut self,
                                                     on_next: FnNext,
                                                     on_completed: FnCompleted,
                                                     on_error: FnError)
                                                     -> Self::Subscription
        where FnNext: FnMut(Self::Item), FnCompleted: FnOnce(), FnError: FnOnce(Self::Error) {
        let observer = ErrorObserver {
            fn_next: on_next,
            fn_completed: on_completed,
            fn_error: on_error,
        };
        self.subscribe(observer)
    }

    /// Subscribes a function that takes an option.
    ///
    /// The function translates into an observer as follows:
    ///
    ///  * `on_next(x)`: calls the functions with `Some(x)`.
    ///  * `on_completed()`: calls the function with `None`.
    ///  * `on_error(error)`: panics.
    ///
    /// After the function has been called with `None`,
    /// it is guaranteed never to be called again.
    ///
    /// **This subscription panics if the observable fails with an error.**
    ///
    /// See also [`subscribe()`](#tymethod.subscribe).
    fn subscribe_option<FnOption>(&mut self,
                                  on_next_or_completed: FnOption)
                                  -> Self::Subscription
        where Self::Error: Debug, FnOption: FnMut(Option<Self::Item>) {
        let observer = OptionObserver {
            fn_option: on_next_or_completed
        };
        self.subscribe(observer)
    }

    /// Subscribes a function that takes a result of an option.
    ///
    /// The function translates into an observer as follows:
    ///
    ///  * `on_next(x)`: calls the function with `Ok(Some(x))`.
    ///  * `on_completed()`: calls the function with `Ok(None)`.
    ///  * `on_error(error)`: calls the function with `Err(error)`.
    ///
    /// After the function has been called with `Ok(None)` or `Err(error)`,
    /// it is guaranteed never to be called again.
    ///
    /// See also [`subscribe()`](#tymethod.subscribe).
    fn subscribe_result<FnResult>(&mut self,
                                  on_next_or_completed_or_error: FnResult)
                                  -> Self::Subscription
        where FnResult: FnMut(Result<Option<Self::Item>, Self::Error>) {
        let observer = ResultObserver {
            fn_result: on_next_or_completed_or_error
        };
        self.subscribe(observer)
    }

    /// Transforms an observable by applying f to every value produced.
    fn map<'s, U, F>(&'s mut self, f: F) -> MapObservable<'s, Self, F>
        where F: Fn(Self::Item) -> U {
        MapObservable::new(self, f)
    }

    /// Transforms an observable by applying f the error in case of failure.
    fn map_error<'s, F, G>(&'s mut self, f: G) -> MapErrorObservable<'s, Self, G>
        where G: Fn(Self::Error) -> F {
        MapErrorObservable::new(self, f)
    }

    /// Joins two observables sequentially.
    ///
    /// After the current observable completes, an observer will start to
    /// receive values from `next` until that observable completes or fails.
    /// The `next` observable is only subscribed to after the current observable
    /// completes.
    fn continue_with<'s, ObNext>(&'s mut self, next: &'s mut ObNext) -> ContinueWithObservable<'s, Self, ObNext>
        where ObNext: Observable<Item = Self::Item, Error = Self::Error> {
        ContinueWithObservable::new(self, next)
    }
}
