// Rx -- Reactive programming for Rust
// Copyright 2016 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

extern crate rx;

use rx::Observable;

#[test]
fn map() {
    let mut values = &[2u8, 3, 5, 7, 11, 13];
    let expected = &[4u8, 6, 10, 14, 22, 26];
    let mut received = Vec::new();
    let mut mapped = values.map(|x| x * 2);
    mapped.subscribe_next(|x| received.push(x));
    assert_eq!(&expected[..], &received[..]);
}
