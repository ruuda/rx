// Rx -- Reactive programming for Rust
// Copyright 2016 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

extern crate rx;

use rx::{Observable, Observer, Subject};
use std::cell::RefCell;
use std::rc::Rc;

// Generator tests

#[test]
fn error() {
    let error = "epic fail";
    let mut observable = rx::error(error);
    let mut received_err = None;
    observable.subscribe_error(
        |_x: u8| panic!("rx::error should not produce value"),
        || panic!("rx::error should not complete"),
        |e| received_err = Some(e)
    );
    assert_eq!(Some(error), received_err);
}

// Option tests

#[test]
fn option_subscribe_next() {
    let mut received = None;

    // Subscribing to `Some` should push the value.
    Some(19).subscribe_next(|x| received = Some(x));
    assert_eq!(Some(19), received);

    None.subscribe_next(|_x: u32| panic!("none should not push a value"));
}

#[test]
fn option_subscribe_completed() {
    let mut received = None;
    let mut completed = false;

    // Subscribing to `Some` should complete after pushing the value.
    Some(19).subscribe_completed(|x| received = Some(x), || completed = true);
    assert_eq!(Some(19), received);
    assert!(completed);

    // Subscribing to `None` should complete without pushing a value.
    completed = false;
    None.subscribe_completed(
        |_x: u32| panic!("none should not push a value"),
        || completed = true
    );
    assert!(completed);
}

#[test]
fn option_subscribe_error() {
    let mut received = None;
    let mut completed = false;

    Some(23).subscribe_error(
        |x| received = Some(x),
        || completed = true,
        |_err| panic!("some observable should not fail")
    );
    assert_eq!(Some(23), received);
    assert!(completed);

    completed = false;
    None.subscribe_error(
        |_x: u32| panic!("none should not push a value"),
        || completed = true,
        |_err| panic!("none observable should not fail")
    );
    assert!(completed);
}

// Result tests

#[test]
fn result_subscribe_next_ok() {
    let mut result: Result<u32, ()> = Ok(13);
    let mut received = None;
    result.subscribe_next(|x| received = Some(x));
    assert_eq!(Some(13), received);
}

#[test]
#[should_panic]
fn result_subscribe_next_err() {
    let mut result: Result<u32, ()> = Err(());
    let mut received = None;

    // This should panic, because we did not provide an error handler.
    result.subscribe_next(|x| received = Some(x));
}

#[test]
fn result_subscribe_completed_ok() {
    let mut result: Result<u32, ()> = Ok(13);
    let mut received = None;
    let mut completed = false;
    result.subscribe_completed(|x| received = Some(x), || completed = true);
    assert_eq!(Some(13), received);
    assert!(completed);
}

#[test]
fn result_subscribe_error_ok() {
    let mut result: Result<u32, ()> = Ok(13);
    let mut received = None;
    let mut completed = false;
    result.subscribe_error(
        |x| received = Some(x),
        || completed = true,
        |_err| panic!("ok result should not be a failing observable")
    );
    assert_eq!(Some(13), received);
    assert!(completed);
}

#[test]
fn result_subscribe_error_err() {
    let mut result: Result<(), u32> = Err(17);
    let mut error = None;
    result.subscribe_error(
        |_x| panic!("err result should not push a value"),
        || panic!("err result should not complete"),
        |err| error = Some(err)
    );
    assert_eq!(Some(17), error);
}

// Slice tests

#[test]
fn slice_subscribe_next() {
    let mut values = &[2u8, 3, 5, 7, 11, 13];
    let mut received = Vec::new();
    values.subscribe_next(|&x| received.push(x));
    assert_eq!(&values[..], &received[..]);
}

#[test]
fn slice_subscribe_completed() {
    let mut values = &[2u8, 3, 5, 7, 11, 13];
    let mut received = Vec::new();
    let mut completed = false;
    values.subscribe_completed(|&x| received.push(x), || completed = true);
    assert_eq!(&values[..], &received[..]);
    assert!(completed);
}

#[test]
fn slice_subscribe_error() {
    let mut values = &[2u8, 3, 5, 7, 11, 13];
    let mut received = Vec::new();
    let mut completed = false;
    let mut failed = false;
    values.subscribe_error(|&x| received.push(x), || completed = true, |_err| failed = true);
    assert_eq!(&values[..], &received[..]);
    assert!(completed);
    assert!(!failed);
}

#[test]
fn slice_subscribe_option() {
    let mut values = &[2u8, 3, 5, 7, 11, 13];
    let expected = &[Some(2u8), Some(3), Some(5), Some(7), Some(11), Some(13), None];
    let mut received = Vec::new();
    values.subscribe_option(|x| received.push(x.cloned()));
    assert_eq!(&received[..], &expected[..]);
}

#[test]
fn slice_subscribe_result() {
    let mut values = &[2u8, 3, 5, 7];
    let expected = &[Ok(Some(2u8)), Ok(Some(3)), Ok(Some(5)), Ok(Some(7)), Ok(None)];
    let mut received = Vec::new();
    values.subscribe_result(|x| received.push(x.map(|y| y.cloned())));
    assert_eq!(&received[..], &expected[..]);
}

// Subject tests

#[test]
fn subject_on_next() {
    let mut subject = Subject::<u8, ()>::new();
    let mut received = Vec::new();
    subject.observable().subscribe_next(|x| received.push(x));

    // Subject should not push anything upon subscription.
    assert_eq!(0, received.len());

    let values = &[2u8, 3, 5, 7, 11, 13];
    for i in 0..values.len() {
        subject.on_next(values[i]);
        assert_eq!(&values[..i + 1], &received[..]);
    }
}

#[test]
fn subject_on_completed() {
    let mut subject = Subject::<u8, ()>::new();
    let mut completed = false;
    subject.observable().subscribe_completed(
        |_x| panic!("no value should be pushed"),
        || completed = true
    );

    // Subject should not push anything upon subscription.
    assert!(!completed);

    subject.on_completed();
    assert!(completed);
}

#[test]
fn subject_on_error() {
    let mut subject = Subject::<u8, u8>::new();
    let mut error = 0;
    subject.observable().subscribe_error(
        |_x| panic!("no value should be pushed"),
        || panic!("subject should not complete"),
        |err| error = err
    );

    // Subject should not fail upon subscription.
    assert_eq!(0, error);

    subject.on_error(41);
    assert_eq!(41, error);
}

/// Helper for the `subject_clones_once_per_observer()` test.
struct CloneCounter {
    counter: Rc<RefCell<u32>>,
}

impl Clone for CloneCounter {
    fn clone(&self) -> CloneCounter {
        let count: u32 = *self.counter.borrow();
        *self.counter.borrow_mut() = count + 1;
        CloneCounter {
            counter: self.counter.clone(),
        }
    }
}

#[test]
fn subject_clones_once_per_observer() {
    let mut subject = Subject::<CloneCounter, ()>::new();
    let mut first_called = false;
    let mut second_called = false;
    let counter = CloneCounter {
        counter: Rc::new(RefCell::new(0)),
    };

    // Subscribe twice.
    subject.observable().subscribe_next(|_x| first_called = true);
    subject.observable().subscribe_next(|_x| second_called = true);

    // Nothing should have been cloned yet.
    assert_eq!(0, *counter.counter.borrow());

    subject.on_next(counter.clone());

    // We cloned once, and the subject should have cloned once per subscription.
    assert_eq!(3, *counter.counter.borrow());
    assert!(first_called);
    assert!(second_called);
}

// TODO: Test multiple subscriptions and combinations of values and completed/error.

// Transform tests

#[test]
fn map() {
    let mut values = &[2u8, 3, 5, 7, 11, 13];
    let expected = &[4u8, 6, 10, 14, 22, 26];
    let mut received = Vec::new();
    let mut mapped = values.map(|x| x * 2);
    mapped.subscribe_next(|x| received.push(x));
    assert_eq!(&expected[..], &received[..]);
}
