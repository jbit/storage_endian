/*
 * storage_endian - Simple integer wrappers for explicitly defining storage endianess
 *
 * https://jbit.net/storage_endian
 *
 * Copyright 2020 James Lee (jbit@jbit.net)
 *
 * Redistribution and use in source and binary forms, with or without modification,
 * are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain
 *      the above copyright notice,
 *      this list of conditions
 *      and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce
 *      the above copyright notice,
 *      this list of conditions
 *      and the following disclaimer
 *      in the documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
 * INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 * IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * SPDX-License-Identifier: BSD-2-Clause
 */

/*!
    Simple integer wrappers for explicitly defining storage endianess.

    The wrappers provide comparison, arithmetic, and conversion using standard Rust traits.

    I've ended up writing bits of this crate several times on various projects, so I decided it's time to make it a crate! :)

    Example Usage
    -------------
    ```rust
    use storage_endian::{BEu32, BEu64};

    #[repr(C)]
    struct Data {
        magic: BEu32,
        version: BEu32,
        size: BEu64,
        thing: BEu64,
    }
    impl Data {
        pub const SIZE: usize = core::mem::size_of::<Self>();
        pub const MAGIC: u32 = 0x1337_beef;

        fn handle_thing(thing: u64) {
            // ...
        }

        pub fn from_bytes(data: [u8; Self::SIZE]) -> Self {
            let mut data: Self = unsafe { core::mem::transmute(data) };

            assert_eq!(data.magic, Self::MAGIC);
            assert_eq!((data.version >> 16) & 0xff, 0x01);
            assert!(data.size >= Self::SIZE as u64);
            Self::handle_thing(data.thing.into());

            data
        }
    }
    ```

    As you can see, most of the time you don't have to worry what endianess the underlying data is, the operator overloading handles it for you.

    To avoid bugs, there is intentionally no easy way to access the data in the underlying representation.

    Alternatives
    ------------
    There are various other solutions to manage endian flipping in Rust, you might be interested in using:
    * https://crates.io/crates/bswap
    * https://crates.io/crates/byteorder
    * https://crates.io/crates/endian
    * https://crates.io/crates/simple_endian

    For one reason or another these haven't worked perfectly for some of my use cases, but they might work for others!
*/

#![no_std]

use core::cmp::Ordering;
use core::fmt::{Binary, Debug, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex};
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem, Shl, Shr, Sub};

macro_rules! wrapper {
    // Implement various traits needed for the endian wrappers
    ( $( $Wrapper:ident :: $fn:ident  ),* ) => { $(
        wrapper!(derive IntFrom(usize::$fn, u128::$fn, u64::$fn, u32::$fn, u16::$fn, u8::$fn,) for $Wrapper);
        wrapper!(derive IntFrom(isize::$fn, i128::$fn, i64::$fn, i32::$fn, i16::$fn, i8::$fn,) for $Wrapper);
        wrapper!(derive Fmt(Debug::fmt, Display::fmt, LowerExp::fmt, Pointer::fmt, UpperExp::fmt, ) for $Wrapper);
        wrapper!(derive Fmt(Binary::fmt, LowerHex::fmt, Octal::fmt, UpperHex::fmt,) for $Wrapper);
        wrapper!(derive Math(Add::add, Div::div, Mul::mul, Rem:: rem, Sub::sub,) for $Wrapper);
        wrapper!(derive Math(BitAnd::bitand, BitOr::bitor, BitXor::bitxor, Shl::shl, Shr::shr,) for $Wrapper);

        impl<T: Copy + From<$Wrapper<T>> + PartialEq> PartialEq<T> for $Wrapper<T> {
            fn eq(&self, other: &T) -> bool {
                T::eq(&T::from(*self), other)
            }
        }
        impl<T: Copy + From<$Wrapper<T>> + PartialEq> PartialEq for $Wrapper<T> {
            fn eq(&self, other: &Self) -> bool {
                T::eq(&T::from(*self), &T::from(*other))
            }
        }
        impl<T: Copy + From<$Wrapper<T>> + PartialOrd> PartialOrd<T> for $Wrapper<T> {
            fn partial_cmp(&self, other: &T) -> Option<Ordering> {
                T::partial_cmp(&T::from(*self), other)
            }
        }
        impl<T: Copy + From<$Wrapper<T>> + PartialOrd> PartialOrd for $Wrapper<T> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                T::partial_cmp(&T::from(*self), &T::from(*other))
            }
        }
        impl<T: Copy + From<$Wrapper<T>> + Ord> Ord for $Wrapper<T> {
            fn cmp(&self, other: &Self) -> Ordering {
                T::cmp(&T::from(*self), &T::from(*other))
            }
        }
        impl<T: Copy + From<$Wrapper<T>> + Eq> Eq for $Wrapper<T> {}
    )* };


    // Expand `derive Foo(a, b,) for Bar` into `derive Foo a for Bar`, `derive Foo b for Bar`
    ( derive $kind:ident($( $trait:ident :: $fn:ident , )*) for $Wrapper:ident ) => {
        $( wrapper!{ derive $kind $trait :: $fn for $Wrapper } )*
    };

    // Implement bi-directional `From` for a type supporting `to_be`
    ( derive IntFrom $t:ident :: $fn:ident for $Wrapper:ident ) => {
        impl From<$Wrapper<$t>> for $t {
            fn from(other: $Wrapper<$t>) -> $t {
                <$t>::$fn(other.0)
            }
        }
        impl From<$t> for $Wrapper<$t> {
            fn from(other: $t) -> Self {
                Self(<$t>::$fn(other))
            }
        }
    };

    // Implement a formatting trait for a wrapper type
    ( derive Fmt $Trait:ident :: $fn:ident for $Wrapper:ident ) => {
        impl<T: Copy + From<$Wrapper<T>> + $Trait> $Trait for $Wrapper<T> {
            fn $fn(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                $Trait::$fn(&T::from(*self), f)
            }
        }
    };

    // Implement a math trait for a wrapper type
    ( derive Math $Trait:ident :: $fn:ident for $Wrapper:ident ) => {
        // wrapper = wrapper + native
        impl<T: Copy + From<$Wrapper<T>> + $Trait> $Trait<T> for $Wrapper<T> where <T as $Trait>::Output: Into<$Wrapper<T>> {
            type Output = Self;
            fn $fn(self, other: T) -> Self::Output {
                $Trait::$fn(T::from(self), other).into()
            }
        }
        // wrapper = wrapper + wrapper
        impl<T: Copy + From<$Wrapper<T>> + $Trait> $Trait for $Wrapper<T> where <T as $Trait>::Output: Into<$Wrapper<T>> {
            type Output = Self;
            fn $fn(self, other: Self) -> Self::Output {
                $Trait::$fn(T::from(self), T::from(other)).into()
            }
        }
    };
}

#[repr(transparent)]
#[derive(Copy, Clone)]
/// Wrapper type for data that's explicitly stored in memory as big endian
pub struct BigEndian<T>(T);

#[repr(transparent)]
#[derive(Copy, Clone)]
/// Wrapper type for data that's explicitly stored in memory as little endian
pub struct LittleEndian<T>(T);

wrapper!(BigEndian::to_be, LittleEndian::to_le);

// Big-endian type aliases
pub type BEu128 = BigEndian<u128>;
pub type BEu64 = BigEndian<u64>;
pub type BEu32 = BigEndian<u32>;
pub type BEu16 = BigEndian<u16>;
pub type BEu8 = BigEndian<u8>;
pub type BEi128 = BigEndian<i128>;
pub type BEi64 = BigEndian<i64>;
pub type BEi32 = BigEndian<i32>;
pub type BEi16 = BigEndian<i16>;
pub type BEi8 = BigEndian<i8>;

// Little-endian type aliases
pub type LEu128 = LittleEndian<u128>;
pub type LEu64 = LittleEndian<u64>;
pub type LEu32 = LittleEndian<u32>;
pub type LEu16 = LittleEndian<u16>;
pub type LEu8 = LittleEndian<u8>;
pub type LEi128 = LittleEndian<i128>;
pub type LEi64 = LittleEndian<i64>;
pub type LEi32 = LittleEndian<i32>;
pub type LEi16 = LittleEndian<i16>;
pub type LEi8 = LittleEndian<i8>;

#[cfg(test)]
extern crate std;
#[cfg(test)]
mod test;
