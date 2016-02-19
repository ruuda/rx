// Rx -- Reactive programming for Rust
// Copyright 2016 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

use lifeline;
use observable::Observable;
use observer::Observer;
use std::marker::PhantomData;

struct MapObserver<T, U, E, O, F>
where O: Observer<U, E>,
      F: Fn(T) -> U {
    observer: O,
    f: F,
    _phantom_t: PhantomData<*mut T>,
    _phantom_u: PhantomData<*mut U>,
    _phantom_e: PhantomData<*mut E>,
}

impl<T, U, E, O, F> Observer<T, E> for MapObserver<T, U, E, O, F>
where T: Clone,
      U: Clone,
      E: Clone,
      O: Observer<U, E>,
      F: Fn(T) -> U {
    fn on_next(&mut self, item: T) {
        self.observer.on_next(self.f.call((item,)));
    }

    fn on_completed(self) {
        self.observer.on_completed();
    }

    fn on_error(self, error: E) {
        self.observer.on_error(error);
    }
}

/// The result of calling `map()` on an observable.
pub struct MapObservable<'a, Source: 'a + ?Sized, F> {
    source: &'a mut Source,
    f: F
}

impl<'a, Source: 'a + ?Sized, F> MapObservable<'a, Source, F> {
    pub fn new(source: &'a mut Source, f: F) -> MapObservable<'a, Source, F> {
        MapObservable {
            source: source,
            f: f,
        }
    }
}

impl<'a, Source, U, F> Observable for MapObservable<'a, Source, F>
where Source: Observable,
      U: Clone,
      F: Fn(<Source as Observable>::Item) -> U {
    type Item = U;
    type Error = <Source as Observable>::Error;
    type Subscription = <Source as Observable>::Subscription;

    fn subscribe<O>(&mut self, observer: O) -> Self::Subscription
        where O: Observer<Self::Item, Self::Error> {
        let mapped_observer = MapObserver {
            observer: observer,
            f: &self.f,
            _phantom_t: PhantomData,
            _phantom_u: PhantomData,
            _phantom_e: PhantomData,
        };
        self.source.subscribe(mapped_observer)
    }
}

struct MapErrorObserver<T, E, F, O, G>
where O: Observer<T, F>,
      G: Fn(E) -> F {
    observer: O,
    f: G,
    _phantom_t: PhantomData<*mut T>,
    _phantom_e: PhantomData<*mut E>,
    _phantom_f: PhantomData<*mut F>,
}

impl<T, E, F, O, G> Observer<T, E> for MapErrorObserver<T, E, F, O, G>
where T: Clone,
      E: Clone,
      F: Clone,
      O: Observer<T, F>,
      G: Fn(E) -> F {
    fn on_next(&mut self, item: T) {
        self.observer.on_next(item);
    }

    fn on_completed(self) {
        self.observer.on_completed();
    }

    fn on_error(self, error: E) {
        self.observer.on_error(self.f.call((error,)));
    }
}

/// The result of calling `map_error()` on an observable.
pub struct MapErrorObservable<'a, Source: 'a + ?Sized, G> {
    source: &'a mut Source,
    f: G
}

impl<'a, Source: 'a + ?Sized, G> MapErrorObservable<'a, Source, G> {
    pub fn new(source: &'a mut Source, f: G) -> MapErrorObservable<'a, Source, G> {
        MapErrorObservable {
            source: source,
            f: f,
        }
    }
}

impl<'a, Source, F, G> Observable for MapErrorObservable<'a, Source, G>
where Source: Observable,
      F: Clone,
      G: Fn(<Source as Observable>::Error) -> F {
    type Item = <Source as Observable>::Item;
    type Error = F;
    type Subscription = <Source as Observable>::Subscription;

    fn subscribe<O>(&mut self, observer: O) -> Self::Subscription
        where O: Observer<Self::Item, Self::Error> {
        // Note that the function `G` cannot be `FnOnce` because every observer
        // receives a copy of it. Alternatively, `map_error` could be
        // implemented with a subject to only call the mapping function once,
        // but that would prevent stream fusion and require memory and a virtual
        // function call.
        let mapped_observer = MapErrorObserver {
            observer: observer,
            f: &self.f,
            _phantom_t: PhantomData,
            _phantom_e: PhantomData,
            _phantom_f: PhantomData,
        };
        self.source.subscribe(mapped_observer)
    }
}

pub struct ContinueWithSubscription<Source: Observable, ObNext: Observable> {
    #[allow(dead_code)] // This code is not dead, it keeps the subscription alive.
    subs_source: Source::Subscription,

    #[allow(dead_code)] // Same here.
    subs_next: lifeline::Lifeline<Option<ObNext::Subscription>>,
}

impl<Source: Observable, ObNext: Observable> Drop for ContinueWithSubscription<Source, ObNext> {
    fn drop(&mut self) {
        // This is a no-op, the lifeline handles everything automatically.
    }
}

struct ContinueWithObserver<'a, T: Clone, E: Clone, ObNext: 'a, O>
where ObNext: Observable<Item = T, Error = E>,
      O: Observer<T, E> {
    observer: O,
    next: &'a mut ObNext,
    subscription: lifeline::Owner<Option<ObNext::Subscription>>,
}

impl<'a, T, E, ObNext, O> Observer<T, E> for ContinueWithObserver<'a, T, E, ObNext, O>
where T: Clone,
      E: Clone,
      ObNext: Observable<Item = T, Error = E>,
      O: Observer<T, E> {
    fn on_next(&mut self, item: T) {
        self.observer.on_next(item);
    }

    fn on_completed(mut self) {
        use std::mem;
        let subs_next = self.next.subscribe(self.observer);
        self.subscription.with_mut_value(|subs| {
            mem::replace(subs, Some(subs_next));
        });
    }

    fn on_error(self, error: E) {
        self.observer.on_error(error);
    }
}

/// The result of calling `continue_with()` on an observable.
pub struct ContinueWithObservable<'a, Source: 'a + ?Sized, ObNext: 'a + ?Sized> {
    source: &'a mut Source,
    next: &'a mut ObNext,
}

impl<'a, Source: 'a + ?Sized, ObNext: 'a + ?Sized> ContinueWithObservable<'a, Source, ObNext> {
    pub fn new(source: &'a mut Source, next: &'a mut ObNext) -> ContinueWithObservable<'a, Source, ObNext> {
        ContinueWithObservable {
            source: source,
            next: next,
        }
    }
}

impl<'a, T: Clone, E: Clone, Source, ObNext> Observable for ContinueWithObservable<'a, Source, ObNext>
where Source: Observable<Item = T, Error = E>,
      ObNext: Observable<Item = T, Error = E> {
    type Item = <Source as Observable>::Item;
    type Error = <Source as Observable>::Error;
    type Subscription = ContinueWithSubscription<Source, ObNext>;

    fn subscribe<O>(&mut self, observer: O) -> Self::Subscription
        where O: Observer<Self::Item, Self::Error> {
        let (life, owner) = lifeline::new(None);
        let continued_observer = ContinueWithObserver {
            observer: observer,
            next: self.next,
            subscription: owner,
        };
        let subs_source = self.source.subscribe(continued_observer);
        ContinueWithSubscription {
            subs_source: subs_source,
            subs_next: life,
        }
    }
}
