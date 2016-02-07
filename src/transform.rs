// Rx -- Reactive programming for Rust
// Copyright 2016 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

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

/// The result of calling `map` on an observable.
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
