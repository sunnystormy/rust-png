// Copyright 2014 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_doc)]

use std::iter::range_inclusive;
use std::cmp;
use std::slice;

static BIT_REV_U8: [u8, ..256] = [
    0b0000_0000, 0b1000_0000, 0b0100_0000, 0b1100_0000,
    0b0010_0000, 0b1010_0000, 0b0110_0000, 0b1110_0000,
    0b0001_0000, 0b1001_0000, 0b0101_0000, 0b1101_0000,
    0b0011_0000, 0b1011_0000, 0b0111_0000, 0b1111_0000,

    0b0000_1000, 0b1000_1000, 0b0100_1000, 0b1100_1000,
    0b0010_1000, 0b1010_1000, 0b0110_1000, 0b1110_1000,
    0b0001_1000, 0b1001_1000, 0b0101_1000, 0b1101_1000,
    0b0011_1000, 0b1011_1000, 0b0111_1000, 0b1111_1000,

    0b0000_0100, 0b1000_0100, 0b0100_0100, 0b1100_0100,
    0b0010_0100, 0b1010_0100, 0b0110_0100, 0b1110_0100,
    0b0001_0100, 0b1001_0100, 0b0101_0100, 0b1101_0100,
    0b0011_0100, 0b1011_0100, 0b0111_0100, 0b1111_0100,

    0b0000_1100, 0b1000_1100, 0b0100_1100, 0b1100_1100,
    0b0010_1100, 0b1010_1100, 0b0110_1100, 0b1110_1100,
    0b0001_1100, 0b1001_1100, 0b0101_1100, 0b1101_1100,
    0b0011_1100, 0b1011_1100, 0b0111_1100, 0b1111_1100,


    0b0000_0010, 0b1000_0010, 0b0100_0010, 0b1100_0010,
    0b0010_0010, 0b1010_0010, 0b0110_0010, 0b1110_0010,
    0b0001_0010, 0b1001_0010, 0b0101_0010, 0b1101_0010,
    0b0011_0010, 0b1011_0010, 0b0111_0010, 0b1111_0010,

    0b0000_1010, 0b1000_1010, 0b0100_1010, 0b1100_1010,
    0b0010_1010, 0b1010_1010, 0b0110_1010, 0b1110_1010,
    0b0001_1010, 0b1001_1010, 0b0101_1010, 0b1101_1010,
    0b0011_1010, 0b1011_1010, 0b0111_1010, 0b1111_1010,

    0b0000_0110, 0b1000_0110, 0b0100_0110, 0b1100_0110,
    0b0010_0110, 0b1010_0110, 0b0110_0110, 0b1110_0110,
    0b0001_0110, 0b1001_0110, 0b0101_0110, 0b1101_0110,
    0b0011_0110, 0b1011_0110, 0b0111_0110, 0b1111_0110,

    0b0000_1110, 0b1000_1110, 0b0100_1110, 0b1100_1110,
    0b0010_1110, 0b1010_1110, 0b0110_1110, 0b1110_1110,
    0b0001_1110, 0b1001_1110, 0b0101_1110, 0b1101_1110,
    0b0011_1110, 0b1011_1110, 0b0111_1110, 0b1111_1110,


    0b0000_0001, 0b1000_0001, 0b0100_0001, 0b1100_0001,
    0b0010_0001, 0b1010_0001, 0b0110_0001, 0b1110_0001,
    0b0001_0001, 0b1001_0001, 0b0101_0001, 0b1101_0001,
    0b0011_0001, 0b1011_0001, 0b0111_0001, 0b1111_0001,

    0b0000_1001, 0b1000_1001, 0b0100_1001, 0b1100_1001,
    0b0010_1001, 0b1010_1001, 0b0110_1001, 0b1110_1001,
    0b0001_1001, 0b1001_1001, 0b0101_1001, 0b1101_1001,
    0b0011_1001, 0b1011_1001, 0b0111_1001, 0b1111_1001,

    0b0000_0101, 0b1000_0101, 0b0100_0101, 0b1100_0101,
    0b0010_0101, 0b1010_0101, 0b0110_0101, 0b1110_0101,
    0b0001_0101, 0b1001_0101, 0b0101_0101, 0b1101_0101,
    0b0011_0101, 0b1011_0101, 0b0111_0101, 0b1111_0101,

    0b0000_1101, 0b1000_1101, 0b0100_1101, 0b1100_1101,
    0b0010_1101, 0b1010_1101, 0b0110_1101, 0b1110_1101,
    0b0001_1101, 0b1001_1101, 0b0101_1101, 0b1101_1101,
    0b0011_1101, 0b1011_1101, 0b0111_1101, 0b1111_1101,


    0b0000_0011, 0b1000_0011, 0b0100_0011, 0b1100_0011,
    0b0010_0011, 0b1010_0011, 0b0110_0011, 0b1110_0011,
    0b0001_0011, 0b1001_0011, 0b0101_0011, 0b1101_0011,
    0b0011_0011, 0b1011_0011, 0b0111_0011, 0b1111_0011,

    0b0000_1011, 0b1000_1011, 0b0100_1011, 0b1100_1011,
    0b0010_1011, 0b1010_1011, 0b0110_1011, 0b1110_1011,
    0b0001_1011, 0b1001_1011, 0b0101_1011, 0b1101_1011,
    0b0011_1011, 0b1011_1011, 0b0111_1011, 0b1111_1011,

    0b0000_0111, 0b1000_0111, 0b0100_0111, 0b1100_0111,
    0b0010_0111, 0b1010_0111, 0b0110_0111, 0b1110_0111,
    0b0001_0111, 0b1001_0111, 0b0101_0111, 0b1101_0111,
    0b0011_0111, 0b1011_0111, 0b0111_0111, 0b1111_0111,

    0b0000_1111, 0b1000_1111, 0b0100_1111, 0b1100_1111,
    0b0010_1111, 0b1010_1111, 0b0110_1111, 0b1110_1111,
    0b0001_1111, 0b1001_1111, 0b0101_1111, 0b1101_1111,
    0b0011_1111, 0b1011_1111, 0b0111_1111, 0b1111_1111
];

struct BitState {
    n: u8,
    v: u32
}

struct BitStream<'a> {
    bytes: slice::Items<'a, u8>,
    used: uint,
    state: BitState
}

// Use this instead of triggering a failure (that may unwind).
fn abort() -> ! {
    unsafe {
        ::std::intrinsics::abort()
    }
}

impl<'a> BitStream<'a> {
    fn new(bytes: &'a [u8], state: BitState) -> BitStream<'a> {
        BitStream {
            bytes: bytes.iter(),
            used: 0,
            state: state
        }
    }

    fn use_byte(&mut self) -> bool {
        match self.bytes.next() {
            Some(&b) => {
                self.state.v |= b as u32 << self.state.n;
                self.state.n += 8;
                self.used += 1;
                true
            }
            None => false
        }
    }

    fn need(&mut self, n: u8) -> bool {
        if self.state.n < n {
            if !self.use_byte() {
                return false;
            }
            if n > 8 && self.state.n < n {
                if n > 16 {
                    // HACK(eddyb) in place of a static assert.
                    abort();
                }
                if !self.use_byte() {
                    return false;
                }
            }
        }
        true
    }

    fn take16(&mut self, n: u8) -> Option<u16> {
        if self.need(n) {
            self.state.n -= n;
            let v = self.state.v & ((1 << n) - 1);
            self.state.v >>= n;
            Some(v as u16)
        } else {
            None
        }
    }

    fn take(&mut self, n: u8) -> Option<u8> {
        assert!(n <= 8);
        self.take16(n).map(|v: u16| v as u8)
    }
}

macro_rules! with_codes (($clens:expr, $max_bits:expr => $code_ty:ty, $cb:expr) => ({
    // Count the number of codes for each bit length.
    let mut bl_count = [0 as $code_ty, ..($max_bits+1)];
    for &bits in $clens.iter() {
        if bits != 0 {
            bl_count[bits as uint] += 1;
        }
    }

    // Compute the first code value for each bit length.
    let mut next_code = [0 as $code_ty, ..($max_bits+1)];
    for bits in range_inclusive(1, $max_bits) {
        next_code[bits as uint] = (next_code[bits as uint - 1] + bl_count[bits as uint - 1]) << 1;
    }

    for (i, &bits) in $clens.iter().enumerate() {
        if bits != 0 {
            let code = next_code[bits as uint];
            next_code[bits as uint] += 1;
            $cb(i as $code_ty, code, bits);
        }
    }
}))

struct CodeLengthReader {
    patterns: ~[u8, ..128],
    clens: ~[u8, ..19],
    result: Vec<u8>,
    num_lit: u16,
    num_dist: u8
}

impl CodeLengthReader {
    fn new(clens: ~[u8, ..19], num_lit: u16, num_dist: u8) -> CodeLengthReader {
        // Fill in the 7-bit patterns that match each code.
        let mut patterns = box() ([0xffu8, ..128]);
        with_codes!(clens, 7 => u8, |i: u8, code: u8, bits| {
            let base = BIT_REV_U8[(code << (8 - bits)) as uint];
            for rest in range(0u8, 1 << (7 - bits)) {
                patterns[(base | (rest << bits)) as uint] = i;
            }
        });

        CodeLengthReader {
            patterns: patterns,
            clens: clens,
            result: Vec::with_capacity(num_lit as uint + num_dist as uint),
            num_lit: num_lit,
            num_dist: num_dist
        }
    }

    fn read(&mut self, stream: &mut BitStream) -> bool {
        let total_len = self.num_lit as uint + self.num_dist as uint;
        while self.result.len() < total_len {
            if !stream.need(7) {
                break;
            }
            let save = stream.state;
            macro_rules! take (($n:expr) => (match stream.take($n) {
                Some(v) => v,
                None => {
                    stream.state = save;
                    return false;
                }
            }))
            let code = self.patterns[(stream.state.v & 0x7f) as uint];
            stream.take(self.clens[code as uint]);
            match code {
                0..15 => self.result.push(code),
                16 => {
                    let last = *self.result.last().unwrap();
                    for _ in range(0, 3 + take!(2)) {
                        self.result.push(last);
                    }
                }
                17 => for _ in range(0, 3 + take!(3)) {
                    self.result.push(0);
                },
                18 => for _ in range(0, 11 + take!(7)) {
                    self.result.push(0);
                },
                _ => abort()
            }
        }
        true
    }

    fn to_lit_and_dist(self) -> (DynHuffman16, DynHuffman16) {
        let num_lit = self.num_lit as uint;
        let lit = DynHuffman16::new(self.result.slice_to(num_lit));
        let dist = DynHuffman16::new(self.result.slice_from(num_lit));
        (lit, dist)
    }
}

struct Trie8bit<T> {
    data: [T, ..16],
    children: [Option<~[T, ..16]>, ..16]
}

struct DynHuffman16 {
    patterns: ~[u16, ..256],
    rest: Vec<Trie8bit<u16>>
}

impl DynHuffman16 {
    fn new(clens: &[u8]) -> DynHuffman16 {
        // Fill in the 8-bit patterns that match each code.
        // Longer patterns go into the trie.
        let mut patterns = box() ([0xffffu16, ..256]);
        let mut rest = Vec::new();
        with_codes!(clens, 15 => u16, |i: u16, code: u16, bits: u8| {
            let entry = i | (bits as u16 << 12);
            if bits <= 8 {
                let base = BIT_REV_U8[(code << (8 - bits)) as uint];
                for rest in range(0u8, 1 << (8 - bits)) {
                    patterns[(base | (rest << bits)) as uint] = entry;
                }
            } else {
                let low = BIT_REV_U8[(code >> (bits - 8)) as uint];
                let high = BIT_REV_U8[((code << (16 - bits)) & 0xff) as uint];
                let (min_bits, idx) = if patterns[low as uint] != 0xffff {
                    let bits_prev = (patterns[low as uint] >> 12) as u8;
                    (cmp::min(bits_prev, bits), patterns[low as uint] & 0x7ff)
                } else {
                    rest.push(Trie8bit {
                        data: [0xffff, ..16],
                        children: [
                            None, None, None, None,
                            None, None, None, None,
                            None, None, None, None,
                            None, None, None, None
                        ]
                    });
                    (bits, (rest.len() - 1) as u16)
                };
                patterns[low as uint] = idx | 0x800 | (min_bits as u16 << 12);
                let trie_entry = rest.get_mut(idx as uint);
                if bits <= 12 {
                    for rest in range(0u8, 1 << (12 - bits)) {
                        trie_entry.data[(high | (rest << (bits - 8))) as uint] = entry;
                    }
                } else {
                    let child = &mut trie_entry.children[(high & 0xf) as uint];
                    if child.is_none() {
                        *child = Some(box() ([0xffff, ..16]));
                    }
                    let box ref mut child = *child.as_mut().unwrap();
                    let high_top = high >> 4;
                    for rest in range(0u8, 1 << (16 - bits)) {
                        child[(high_top | (rest << (bits - 12))) as uint] = entry;
                    }
                }
            }
        });
        DynHuffman16 {
            patterns: patterns,
            rest: rest
        }
    }

    fn read(&self, stream: &mut BitStream) -> Option<(BitState, u16)> {
        let has8 = stream.need(8);
        let entry = self.patterns[(stream.state.v & 0xff) as uint];
        let bits = (entry >> 12) as u8;

        if !has8 {
            if bits <= stream.state.n {
                let save = stream.state;
                stream.state.n -= bits;
                stream.state.v >>= bits;
                Some((save, entry & 0xfff))
            } else {
                None
            }
        } else if bits <= 8 {
            let save = stream.state;
            stream.state.n -= bits;
            stream.state.v >>= bits;
            Some((save, entry & 0xfff))
        } else {
            let has16 = stream.need(16);
            let trie = self.rest.get((entry & 0x7ff) as uint);
            let idx = stream.state.v >> 8;
            let trie_entry = match trie.children[(idx & 0xf) as uint] {
                Some(ref child) => child[((idx >> 4) & 0xf) as uint],
                None => trie.data[(idx & 0xf) as uint]
            };
            let trie_bits = (trie_entry >> 12) as u8;
            if has16 || trie_bits <= stream.state.n {
                let save = stream.state;
                stream.state.n -= trie_bits;
                stream.state.v >>= trie_bits;
                Some((save, trie_entry & 0xfff))
            } else {
                None
            }
        }
    }
}

enum State {
    ZlibMethodAndFlags, // CMF
    ZlibFlags(/* CMF */ u8), // FLG,
    Bits(BitsNext, BitState),
    LenDist((BitsNext, BitState), /* len */ u16, /* dist */ u16),
    CheckCRC
}

enum BitsNext {
    BlockHeader,
    BlockUncompressed,
    BlockFixed,
    BlockDynHlit,
    BlockDynHdist(/* hlit */ u8),
    BlockDynHclen(/* hlit */ u8, /* hdist */ u8),
    BlockDynClenCodeLengths(/* hlit */ u8, /* hdist */ u8, /* hclen */ u8, /* idx */ u8, /* clens */ ~[u8, ..19]),
    BlockDynCodeLengths(CodeLengthReader),
    BlockDyn(/* lit/len */ DynHuffman16, /* dist */ DynHuffman16)
}

pub struct InflateStream {
    buffer: Vec<u8>,
    pos: u16,
    state: Option<State>,
    final_block: bool,
}

impl InflateStream {
    #[allow(dead_code)]
    pub fn new() -> InflateStream {
        let state = Bits(BlockHeader, BitState { n: 0, v: 0 });
        let buffer = Vec::with_capacity(32 * 1024);
        InflateStream::with_state_and_buffer(state, buffer)
    }

    #[allow(dead_code)]
    pub fn from_zlib() -> InflateStream {
        InflateStream::with_state_and_buffer(ZlibMethodAndFlags, Vec::new())
    }

    fn with_state_and_buffer(state: State, buffer: Vec<u8>) -> InflateStream {
        InflateStream {
            buffer: buffer,
            pos: 0,
            state: Some(state),
            final_block: false
        }
    }

    fn run_len_dist(&mut self, len: u16, dist: u16) -> Option<u16> {
        //debug!("DEFLATE -{}, ..{} (cap={} len={})", dist, len,
        //       self.buffer.capacity(), self.buffer.len());
        let buffer_size = self.buffer.capacity() as u16;
        let len = if self.pos < dist {
            // Handle copying from ahead, until we hit the end reading.
            let pos_end = self.pos + len;
            let (pos_end, left) = if pos_end <= dist {
                (pos_end, 0)
            } else {
                (dist, pos_end - dist)
            };
            let forward = buffer_size - dist;
            assert!(pos_end + forward <= self.buffer.len() as u16);
            unsafe {
                // HACK(eddyb) avoid bound checks, LLVM can't optimize these.
                let buffer = self.buffer.as_mut_slice().as_mut_ptr();
                let dst_end = buffer.offset(pos_end as int);
                let mut dst = buffer.offset(self.pos as int);
                let mut src = buffer.offset((self.pos + forward) as int);
                while dst < dst_end {
                    *dst = *src;
                    dst = dst.offset(1);
                    src = src.offset(1);
                }
            }
            self.pos = pos_end;
            left
        } else {
            len
        };
        // Handle copying from before, until we hit the end writing.
        let pos_end = self.pos + len;
        let (pos_end, left) = if pos_end <= buffer_size {
            (pos_end, None)
        } else {
            (buffer_size, Some(pos_end - buffer_size))
        };
        unsafe {
            // HACK(eddyb) avoid bound checks, LLVM can't optimize these.
            let buffer = self.buffer.as_mut_slice().as_mut_ptr();
            let mut dst = buffer.offset(self.pos as int);
            let mut src = buffer.offset((self.pos - dist) as int);
            let dst_end = buffer.offset(pos_end as int);
            while dst < dst_end {
                *dst = *src;
                dst = dst.offset(1);
                src = src.offset(1);
            }
        }
        if self.buffer.len() < pos_end as uint {
            unsafe {
                self.buffer.set_len(pos_end as uint);
            }
        }
        self.pos = pos_end;
        left
    }

    fn next_state(&mut self, data: &[u8]) -> Result<uint, String> {
        macro_rules! ok_bytes (($n:expr, $state:expr) => ({
            self.state = Some($state);
            Ok($n)
        }))
        //let debug_byte = |i, b| debug!("[{:4}] {:02x}", i, b);
        macro_rules! push_or (($b:expr, $ret:expr) => (if self.pos < self.buffer.capacity() as u16 {
            let b = $b;
            //debug_byte(self.pos, b);
            if (self.pos as uint) < self.buffer.len() {
                self.buffer.as_mut_slice()[self.pos as uint] = b;
            } else {
                if (self.pos as uint) != self.buffer.len() {
                    abort();
                }
                self.buffer.push(b);
            }
            self.pos += 1;
        } else {
            return $ret;
        }))
        macro_rules! run_len_dist (($len:expr, $dist:expr => ($bytes:expr, $next:expr, $state:expr)) => ({
            let dist = $dist;
            let left = self.run_len_dist($len, dist);
            match left {
                Some(len) => {
                    return ok_bytes!($bytes, LenDist(($next, $state), len, dist));
                }
                None => {}
            }
        }))
        match self.state.take_unwrap() {
            ZlibMethodAndFlags => {
                let b = data[0];
                let (method, info) = (b & 0xF, b >> 4);
                // debug!("ZLIB CM=0x{:x} CINFO=0x{:x}", method, info);
                match method {
                    8 => {/* DEFLATE */}
                    _ => return Err(format!("unknown ZLIB method CM=0x{:x}", method))
                }

                if info > 7 {
                    return Err(format!("invalid ZLIB info CINFO=0x{:x}", info));
                }

                self.buffer = Vec::with_capacity(1 << (8 + info));

                ok_bytes!(1, ZlibFlags(b))
            }
            ZlibFlags(cmf) => {
                let b = data[0];
                let (_check, dict, _level) = (b & 0x1F, (b & 0x20) != 0, b >> 6);
                // debug!("ZLIB FCHECK=0x{:x} FDICT={} FLEVEL=0x{:x}", check, dict, _level);

                if ((cmf as u16 << 8) | b as u16) % 31 != 0 {
                    return Err(format!("invalid ZLIB checksum CMF=0x{:x} FLG=0x{:x}", cmf, b));
                }

                if dict {
                    return Err("unimplemented ZLIB FDICT=1".to_owned());
                }

                ok_bytes!(1, Bits(BlockHeader, BitState { n: 0, v: 0 }))
            }
            Bits(next, state) => {
                let mut stream = BitStream::new(data, state);
                macro_rules! ok_state (($state:expr) => (ok_bytes!(stream.used, $state)))
                macro_rules! ok (($next:expr) => (ok_state!(Bits($next, stream.state))))
                macro_rules! need (($n:expr) => (if !stream.need($n) { return ok!(next); }))
                macro_rules! take (
                    ($n:expr => $next:expr) => (match stream.take($n) {
                        Some(v) => v,
                        None => return ok!($next)
                    });
                    ($n:expr) => (take!($n => next))
                )
                macro_rules! take16 (
                    ($n:expr => $next:expr) => (match stream.take16($n) {
                        Some(v) => v,
                        None => return ok!($next)
                    });
                    ($n:expr) => (take16!($n => next))
                )
                macro_rules! len_dist (
                    ($len:expr, $code:expr, $bits:expr => $next:expr) => ({
                        let dist = 1 + if $bits == 0 { 0 } else { // new_base
                            2 << $bits
                        } + (($code as u16 - if $bits == 0 { 0 } else { // old_base
                            $bits * 2 + 2
                        }) << $bits) + take16!($bits => $next) as u16;
                        run_len_dist!($len, dist => (stream.used, $next, stream.state));
                    });
                    ($len:expr, $code:expr, $bits:expr) => (
                        len_dist!($len, $code, $bits => next)
                    )
                )
                match next {
                    BlockHeader => {
                        let h = take!(3);
                        let (final, block_type) = ((h & 1) != 0, (h >> 1) & 0b11);

                        if self.final_block {
                            return Err("DEFLATE data after the final block".to_owned());
                        }

                        self.final_block = final;

                        match block_type {
                            0 => {
                                // Skip to the next byte for an uncompressed block.
                                stream.state = BitState { n: 0, v: 0 };
                                ok!(BlockUncompressed)
                            }
                            1 => {
                                /*let lit = DynHuffman16::new([
                                    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, // 0-15
                                    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, // 16-31
                                    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, // 32-47
                                    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, // 48-63
                                    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, // 64-79
                                    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, // 80-95
                                    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, // 96-101
                                    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, // 112-127
                                    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, // 128-143
                                    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, // 144-159
                                    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, // 160-175
                                    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, // 176-191
                                    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, // 192-207
                                    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, // 208-223
                                    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, // 224-239
                                    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, // 240-255
                                    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, // 256-271
                                    7, 7, 7, 7, 7, 7, 7, 7, // 272-279
                                    8, 8, 8, 8, 8, 8, 8, 8, // 280-287
                                ]);
                                let dist = DynHuffman16::new([
                                    5, 5, 5, 5, 5, 5, 5, 5,
                                    5, 5, 5, 5, 5, 5, 5, 5,
                                    5, 5, 5, 5, 5, 5, 5, 5,
                                    5, 5, 5, 5, 5, 5, 5, 5
                                ]);
                                ok!(BlockDyn(lit, dist))*/
                                ok!(BlockFixed)
                            }
                            2 => ok!(BlockDynHlit),
                            _ => Err(format!("unimplemented DEFLATE block type 0b{:t}",
                                             block_type))
                        }
                    }
                    BlockUncompressed => {
                        let len = take16!(16);
                        fail!("uncompressed len={}", len)
                    }
                    BlockFixed => {
                        macro_rules! len_dist2 (($len:expr, $code_const:expr, $code_rev:expr, $bits:expr) => ({
                            len_dist!($len, $code_const + ($code_rev >> 4), $bits);
                        }))
                        macro_rules! len (($code:expr, $bits:expr) => ({
                            let len = 3 + if $bits == 0 { 0 } else { // new_base
                                4 << $bits
                            } + ((if $code == 29 {
                                256
                            } else {
                                $code as u16
                            } - if $bits == 0 { 0 } else { // old_base
                                $bits * 4 + 4
                            } - 1) << $bits) + take!($bits) as u16;
                            let code = take!(5);
                            match code {
                                0b00000 | 0b10000 => len_dist2!(len, 0, code, 0),
                                0b01000 | 0b11000 => len_dist2!(len, 2, code, 0),
                                0b00100 | 0b10100 => len_dist2!(len, 4, code, 1),
                                0b01100 | 0b11100 => len_dist2!(len, 6, code, 2),
                                0b00010 | 0b10010 => len_dist2!(len, 8, code, 3),
                                0b01010 | 0b11010 => len_dist2!(len, 10, code, 4),
                                0b00110 | 0b10110 => len_dist2!(len, 12, code, 5),
                                0b01110 | 0b11110 => len_dist2!(len, 14, code, 6),
                                0b00001 | 0b10001 => len_dist2!(len, 16, code, 7),
                                0b01001 | 0b11001 => len_dist2!(len, 18, code, 8),
                                0b00101 | 0b10101 => len_dist2!(len, 20, code, 9),
                                0b01101 | 0b11101 => len_dist2!(len, 22, code, 10),
                                0b00011 | 0b10011 => len_dist2!(len, 24, code, 11),
                                0b01011 | 0b11011 => len_dist2!(len, 26, code, 12),
                                0b00111 | 0b10111 => len_dist2!(len, 28, code, 13),
                                _ => return Err(format!("bad DEFLATE dist code {}", code))
                            }
                        }))
                        loop {
                            need!(7);
                            // 0000000 through 0010111
                            if (stream.state.v & 0b11) == 0b00 &&
                                (stream.state.v & 0b1100) != 0b1100 {
                                //let save = stream.state;
                                // FIXME(eddyb) use a 7-bit rev LUT or match the huffman code directly.
                                let code = BIT_REV_U8[(stream.take(7).unwrap() << 1) as uint];
                                match code {
                                    0 => return if self.final_block {
                                        ok_state!(CheckCRC)
                                    } else {
                                        ok!(BlockHeader)
                                    },
                                    1..8 => len!(code, 0),
                                    9..12 => len!(code, 1),
                                    13..16 => len!(code, 2),
                                    17..20 => len!(code, 3),
                                    21..23 => len!(code, 4),
                                    _ => return Err(format!("bad DEFLATE len code {}", code as u16 + 256))
                                };
                                //if do_return { return ok!(Bits(next, save)) }
                                continue;
                            }

                            need!(8);
                            // 00110000 through 10111111
                            if (stream.state.v & 0b11) != 0b11 {
                                let save = stream.state;
                                //let taken = take(8);
                                //debug!("8bit {:08t} {:08t}", taken, BIT_REV_U8[taken]);
                                // FIXME(eddyb) use a specialized rev LUT with addend.
                                let code = BIT_REV_U8[(stream.take(8).unwrap()) as uint] - 0b0011_0000;
                                push_or!(code, {stream.state = save; ok!(next)});
                                continue;
                            }
                            // 11000000 through 11000111
                            if (stream.state.v & 0b11100) == 0b00000 {
                                //let save = stream.state;
                                // FIXME(eddyb) use a 3-bit rev LUT or match the huffman code directly.
                                let code = 24 + (BIT_REV_U8[(stream.take(8).unwrap() << 1) as uint] & 0b111);
                                match code {
                                    24 => len!(24, 4),
                                    25..28 => len!(code, 5),
                                    29 => len!(29, 0),
                                    _ => return Err(format!("bad DEFLATE len code {}", code as u16 + 256))
                                };
                                //if do_return { return ok!(Bits(next, save)) }
                                continue;
                            }

                            need!(9);
                            // 110010000 through 111111111
                            let save = stream.state;
                            // FIXME(eddyb) use a specialized rev LUT with addend.
                            let code = 144 + BIT_REV_U8[(stream.take16(9).unwrap() >> 1) as uint] - 0b1001_0000;
                            //debug!("DEFLATE 9bit 0x{:02x}", code);
                            push_or!(code, {stream.state = save; ok!(next)});
                        }
                    }
                    BlockDynHlit => ok!(BlockDynHdist(take!(5) + 1)),
                    BlockDynHdist(hlit) => ok!(BlockDynHclen(hlit, take!(5) + 1)),
                    BlockDynHclen(hlit, hdist) => {
                        ok!(BlockDynClenCodeLengths(hlit, hdist, take!(4) + 4, 0, box() ([0, ..19])))
                    }
                    BlockDynClenCodeLengths(hlit, hdist, hclen, i, mut clens) => {
                        let v = match stream.take(3) {
                            Some(v) => v,
                            None => return ok!(BlockDynClenCodeLengths(hlit, hdist, hclen, i, clens))
                        };
                        clens[[16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15][i as uint]] = v;
                        if i < hclen - 1 {
                            ok!(BlockDynClenCodeLengths(hlit, hdist, hclen, i + 1, clens))
                        } else {
                            ok!(BlockDynCodeLengths(CodeLengthReader::new(clens, hlit as u16 + 256, hdist)))
                        }
                    }
                    BlockDynCodeLengths(mut reader) => {
                        let finished = reader.read(&mut stream);
                        if finished {
                            let (lit, dist) = reader.to_lit_and_dist();
                            ok!(BlockDyn(lit, dist))
                        } else {
                            ok!(BlockDynCodeLengths(reader))
                        }
                    }
                    BlockDyn(lit_len, dist) => {
                        macro_rules! len (($code:expr, $bits:expr) => ({
                            let len = 3 + if $bits == 0 { 0 } else { // new_base
                                4 << $bits
                            } + ((if $code == 29 {
                                256
                            } else {
                                $code as u16
                            } - if $bits == 0 { 0 } else { // old_base
                                $bits * 4 + 4
                            } - 1) << $bits) + take!($bits => BlockDyn(lit_len, dist)) as u16;
                            let (_, code) = match dist.read(&mut stream) {
                                Some(data) => data,
                                None => break
                            };
                            match code {
                                0..3 => len_dist!(len, code, 0 => BlockDyn(lit_len, dist)),
                                4..5 => len_dist!(len, code, 1 => BlockDyn(lit_len, dist)),
                                6..7 => len_dist!(len, code, 2 => BlockDyn(lit_len, dist)),
                                8..9 => len_dist!(len, code, 3 => BlockDyn(lit_len, dist)),
                                10..11 => len_dist!(len, code, 4 => BlockDyn(lit_len, dist)),
                                12..13 => len_dist!(len, code, 5 => BlockDyn(lit_len, dist)),
                                14..15 => len_dist!(len, code, 6 => BlockDyn(lit_len, dist)),
                                16..17 => len_dist!(len, code, 7 => BlockDyn(lit_len, dist)),
                                18..19 => len_dist!(len, code, 8 => BlockDyn(lit_len, dist)),
                                20..21 => len_dist!(len, code, 9 => BlockDyn(lit_len, dist)),
                                22..23 => len_dist!(len, code, 10 => BlockDyn(lit_len, dist)),
                                24..25 => len_dist!(len, code, 11 => BlockDyn(lit_len, dist)),
                                26..27 => len_dist!(len, code, 12 => BlockDyn(lit_len, dist)),
                                28..29 => len_dist!(len, code, 13 => BlockDyn(lit_len, dist)),
                                _ => return Err(format!("bad DEFLATE dist code {}", code))
                            }
                        }))
                        loop {
                            let (save, code16) = match lit_len.read(&mut stream) {
                                Some(data) => data,
                                None => break
                            };
                            let code = code16 as u8;
                            match code16 {
                                0..255 => {
                                    push_or!(code, {
                                        stream.state = save;
                                        ok!(BlockDyn(lit_len, dist))
                                    });
                                    continue;
                                }
                                256..285 => {}
                                _ => return Err(format!("bad DEFLATE len code {}", code))
                            }
                            match code {
                                0 => return if self.final_block {
                                    ok_state!(CheckCRC)
                                } else {
                                    ok!(BlockHeader)
                                },
                                1..8 => len!(code, 0),
                                9..12 => len!(code, 1),
                                13..16 => len!(code, 2),
                                17..20 => len!(code, 3),
                                21..24 => len!(code, 4),
                                25..28 => len!(code, 5),
                                29 => len!(29, 0),
                                _ => return Err(format!("bad DEFLATE len code {}", code as u16 + 256))
                            }
                        }
                        ok!(BlockDyn(lit_len, dist))
                    }
                }
            }
            LenDist((next, state), len, dist) => {
                run_len_dist!(len, dist => (0, next, state));
                ok_bytes!(0, Bits(next, state))
            }
            CheckCRC => {
                let _b = data[0];
                //debug!("CRC check not implemented")
                ok_bytes!(1, CheckCRC)
            }
        }
    }

    #[allow(dead_code)]
    pub fn update<'a>(&'a mut self, mut data: &[u8]) -> Result<(uint, &'a [u8]), String> {
        let original_size = data.len();
        let original_pos = self.pos as uint;
        while data.len() > 0 &&
            ((self.pos as uint) < self.buffer.capacity() || self.buffer.capacity() == 0) {
            match self.next_state(data) {
                Ok(n) => { data = data.slice_from(n); }
                Err(m) => return Err(m)
            }
        }
        let output = self.buffer.as_slice().slice(original_pos, self.pos as uint);
        if self.pos as uint >= self.buffer.capacity() {
            self.pos = 0;
        }
        Ok((original_size - data.len(), output))
    }
}
