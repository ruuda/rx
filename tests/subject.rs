// Rx -- Reactive programming for Rust
// Copyright 2016 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

extern crate rx;

use rx::{Observable, Observer, Subject};

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

// TODO: Test multiple subscriptions and combinations of values and completed/error.
