// font-kit/src/utils.rs
//
// Copyright © 2018 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Miscellaneous utilities for use in this crate.

#![allow(dead_code)]

pub(crate) static SFNT_VERSIONS: [[u8; 4]; 4] = [
    [0x00, 0x01, 0x00, 0x00],
    [b'O', b'T', b'T', b'O'],
    [b't', b'r', b'u', b'e'],
    [b't', b'y', b'p', b'1'],
];

pub(crate) fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub(crate) fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
