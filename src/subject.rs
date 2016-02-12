// Rx -- Reactive programming for Rust
// Copyright 2016 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

use observable::Observable;
use observer::{Observer, BoxedObserver};
use std::rc;

struct ObserverBox<T, E> {
    observer: Box<BoxedObserver<T, E>>,

    /// If the Rc is still alive, the observer should be kept alive.
    alive: rc::Weak<()>,
}

/// Both an observer and observable.
///
/// A subject is a low-level primitive for creating observables.
///
/// TODO: Add example.
pub struct Subject<T, E> {
    observers: Vec<ObserverBox<T, E>>,
}

/// Proxy object that exposes the observable part of a subject.
pub struct SubjectObservable<'s, T: 's, E: 's> {
    subject: &'s mut Subject<T, E>,
}

pub struct SubjectSubscription {
    /// The owner of the unit that `ObserverBox` has a weak reference to.
    ///
    /// Once this is dropped, the observer will not be used any more.
    _alive: rc::Rc<()>,
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
        for observer_box in &mut self.observers {
            if let Some(_) = observer_box.alive.upgrade() {
                // Subscription was not dropped, invoke method.
                observer_box.observer.on_next(item.clone());
            } else {
                // TODO: Remove observer from list, the subscription has been
                // dropped.
            }
        }
    }

    fn on_completed(mut self) {
        for observer_box in self.observers.drain(..) {
            if let Some(_) = observer_box.alive.upgrade() {
                // Subscription was not dropped, invoke method.
                observer_box.observer.on_completed_box();
            } else {
                // TODO: Remove observer from list, the subscription has been
                // dropped.
            }
        }
    }

    fn on_error(mut self, error: E) {
        for observer_box in self.observers.drain(..) {
            if let Some(_) = observer_box.alive.upgrade() {
                // Subscription was not dropped, invoke method.
                observer_box.observer.on_error_box(error.clone());
            } else {
                // TODO: Remove observer from list, the subscription has been
                // dropped.
            }
        }
    }
}

impl<'s, T: Clone, E: Clone> Observable for SubjectObservable<'s, T, E> {
    type Item = T;
    type Error = E;
    type Subscription = SubjectSubscription;

    fn subscribe<O: 'static>(&mut self, observer: O) -> Self::Subscription
        where O: Observer<Self::Item, Self::Error> {
        let boxed: Box<BoxedObserver<T, E>> = Box::new(observer);
        let alive_owner = rc::Rc::new(());
        let observer_box = ObserverBox {
            observer: boxed,
            alive: rc::Rc::downgrade(&alive_owner),
        };
        self.subject.observers.push(observer_box);
        SubjectSubscription {
            _alive: alive_owner,
        }
    }
}

impl Drop for SubjectSubscription {
    fn drop(&mut self) {
        // Nothing to do, the Rc already does the right thing.
    }
}
