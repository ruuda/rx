// Rx -- Reactive programming for Rust
// Copyright 2016 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

extern crate rx;

use rx::Observable;

#[test]
fn subscribe_next_slice() {
    let mut values = &[2u8, 3, 5, 7, 11, 13];
    let mut received = Vec::new();
    values.subscribe_next(|&x| received.push(x));
    assert_eq!(&values[..], &received[..]);
}

#[test]
fn subscribe_completed_slice() {
    let mut values = &[2u8, 3, 5, 7, 11, 13];
    let mut received = Vec::new();
    let mut completed = false;
    values.subscribe_completed(|&x| received.push(x), || completed = true);
    assert_eq!(&values[..], &received[..]);
    assert!(completed);
}

#[test]
fn subscribe_error_slice() {
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
fn subscribe_option_slice() {
    let mut values = &[2u8, 3, 5, 7, 11, 13];
    let expected = &[Some(2u8), Some(3), Some(5), Some(7), Some(11), Some(13), None];
    let mut received = Vec::new();
    values.subscribe_option(|x| received.push(x.cloned()));
    assert_eq!(&received[..], &expected[..]);
}

#[test]
fn subscribe_result_slice() {
    let mut values = &[2u8, 3, 5, 7];
    let expected = &[Ok(Some(2u8)), Ok(Some(3)), Ok(Some(5)), Ok(Some(7)), Ok(None)];
    let mut received: Vec<Result<Option<u8>, ()>> = Vec::new();
    values.subscribe_result(|x| received.push(x.map(|y| y.cloned())));
    assert_eq!(&received[..], &expected[..]);
}
