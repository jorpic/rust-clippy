// Copyright 2014-2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[warn(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::cast_lossless
)]
#[allow(clippy::no_effect, clippy::unnecessary_operation)]
fn main() {
    // Casting from *size
    1isize as i8;
    1isize as f64;
    1usize as f64;
    1isize as f32;
    1usize as f32;
    1isize as i32;
    1isize as u32;
    1usize as u32;
    1usize as i32;
    // Casting to *size
    1i64 as isize;
    1i64 as usize;
    1u64 as isize;
    1u64 as usize;
    1u32 as isize;
    1u32 as usize; // Should not trigger any lint
    1i32 as isize; // Neither should this
    1i32 as usize;
}
