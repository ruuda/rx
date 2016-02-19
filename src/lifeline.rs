// Rx -- Reactive programming for Rust
// Copyright 2016 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

//! A module for dealing with decoupled lifetime and ownership.
//!
//! A lifeline and owner come in pairs: the lifeline is in control of the
//! lifetime of the stored value, but the owner is only one who can access the
//! value. If the lifeline is dropped, the stored value is dropped as well. The
//! owner will not be able to use the value then. If the owner consumes the
//! value, dropping the lifeline is a no-op.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// Struct that controls the lifetime of the value in the lifeline-owner pair.
struct Lifeline<T> {
    value: Rc<RefCell<Option<T>>>,
}


/// Struct that allows access to the value in the lifeline-owner pair.
struct Owner<T> {
    value: Weak<RefCell<Option<T>>>,
}

impl<T> Owner<T> {
    fn with_value<F: FnOnce(&T)>(&self, action: F) {
        if let Some(cell) = self.value.upgrade() {
            if let Some(ref value) = *cell.borrow() {
                action(value);
            }
        }
    }

    fn with_mut_value<F: FnOnce(&mut T)>(&mut self, action: F) {
        if let Some(cell) = self.value.upgrade() {
            if let Some(ref mut value) = *cell.borrow_mut() {
                action(value);
            }
        }
    }

    fn take(self) -> Option<T> {
        if let Some(cell) = self.value.upgrade() {
            cell.borrow_mut().take()
        } else {
            None
        }
    }
}

/// Creates a value with decoupled lifetime and ownership.
fn new<T>(value: T) -> (Lifeline<T>, Owner<T>) {
    let rc = Rc::new(RefCell::new(Some(value)));
    let owner = Owner { value: Rc::downgrade(&rc) };
    let lifeline = Lifeline { value: rc };
    (lifeline, owner)
}
