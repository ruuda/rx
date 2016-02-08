// Rx -- Reactive programming for Rust
// Copyright 2016 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

extern crate rx;

use rx::Observable;

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
