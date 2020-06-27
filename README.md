storage_endian
==============

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
