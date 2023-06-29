#![allow(unused)]
use std::{io, mem::MaybeUninit};

fn main() {
    fn read_chunk (reader: &'_ mut dyn io::Read) -> io::Result<[u8; 64]>
    {
        let mut buffer = MaybeUninit::<[u8; 64]>::uninit();
        reader.read_exact(unsafe { buffer.assume_init_mut() })?;
        Ok(unsafe { buffer.assume_init() })
    }
}