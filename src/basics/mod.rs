// Copyright 2016 Masaki Hara
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ops::{Deref, DerefMut};

/// An ASN.1 tag class, used in [`Tag`][tag].
///
/// [tag]: struct.Tag.html
///
/// A tag class is one of:
///
/// - UNIVERSAL
/// - APPLICATION
/// - context specific
/// - PRIVATE
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TagClass {
    Universal = 0, Application = 1, ContextSpecific = 2, Private = 3,
}

/// An ASN.1 tag.
///
/// An ASN.1 tag is a pair of a tag class and a tag number.
///
/// - A tag class is one of:
///   - UNIVERSAL
///   - APPLICATION
///   - context specific
///   - PRIVATE
/// - A tag number is a nonnegative integer.
///   In this library. Tag numbers are assumed to be in `u64`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tag {
    pub tag_class: TagClass,
    pub tag_number: u64,
}

pub const TAG_BOOLEAN : Tag = Tag {
    tag_class: TagClass::Universal,
    tag_number: 1,
};

pub const TAG_INTEGER : Tag = Tag {
    tag_class: TagClass::Universal,
    tag_number: 2,
};

pub const TAG_BITSTRING : Tag = Tag {
    tag_class: TagClass::Universal,
    tag_number: 3,
};

pub const TAG_OCTETSTRING : Tag = Tag {
    tag_class: TagClass::Universal,
    tag_number: 4,
};

pub const TAG_NULL : Tag = Tag {
    tag_class: TagClass::Universal,
    tag_number: 5,
};

pub const TAG_OID : Tag = Tag {
    tag_class: TagClass::Universal,
    tag_number: 6,
};

pub const TAG_UTF8STRING : Tag = Tag {
    tag_class: TagClass::Universal,
    tag_number: 12,
};

pub const TAG_SEQUENCE : Tag = Tag {
    tag_class: TagClass::Universal,
    tag_number: 16,
};

pub const TAG_SET : Tag = Tag {
    tag_class: TagClass::Universal,
    tag_number: 17,
};

pub const TAG_PRINTABLESTRING : Tag = Tag {
    tag_class: TagClass::Universal,
    tag_number: 19,
};

pub const TAG_UTCTIME : Tag = Tag {
    tag_class: TagClass::Universal,
    tag_number: 23,
};

impl Tag {
    /// Constructs an APPLICATION tag, namely \[APPLICATION n\].
    pub fn application(tag_number: u64) -> Tag {
        return Tag {
            tag_class: TagClass::Application,
            tag_number: tag_number,
        }
    }
    /// Constructs a context specific tag, namely \[n\].
    pub fn context(tag_number: u64) -> Tag {
        return Tag {
            tag_class: TagClass::ContextSpecific,
            tag_number: tag_number,
        }
    }
    /// Constructs a PRIVATE tag, namely \[PRIVATE n\].
    pub fn private(tag_number: u64) -> Tag {
        return Tag {
            tag_class: TagClass::Private,
            tag_number: tag_number,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BitString {
    pub unused_bits: usize,
    pub buf: Vec<u8>,
}

impl BitString {
    pub fn new() -> Self {
        return BitString {
            unused_bits: 0,
            buf: Vec::new(),
        };
    }
    pub fn from_buf(unused_bits : usize, buf: Vec<u8>) -> Self {
        return BitString {
            unused_bits: unused_bits,
            buf: buf,
        };
    }
    pub fn push(&mut self, b: bool) {
        if self.unused_bits == 0 {
            self.buf.push(0);
            self.unused_bits = 8;
        }
        let last = self.buf.last_mut().unwrap();
        self.unused_bits -= 1;
        *last = *last | ((b as u8) << self.unused_bits);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SetOf<T> {
    pub vec: Vec<T>,
}

impl<T> SetOf<T> {
    pub fn new() -> Self {
        SetOf {
            vec: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ObjectIdentifier {
    ids: Vec<u64>,
}

use std::slice::Iter;
impl ObjectIdentifier {
    pub fn new(ids: Vec<u64>) -> Self {
        return ObjectIdentifier {
            ids: ids,
        };
    }
    pub fn iter(&self) -> Iter<u64> {
        self.ids.iter()
    }
}

impl Deref for ObjectIdentifier {
    type Target = [u64];
    fn deref(&self) -> &Self::Target {
        return &self.ids;
    }
}

impl DerefMut for ObjectIdentifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.ids;
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PrintableString {
    string: String,
}

impl PrintableString {
    pub fn from_bytes(bytes: Vec<u8>) -> Option<Self> {
        for &b in bytes.iter() {
            let ok =
                (b'0' <= b && b <= b'9') ||
                (b'A' <= b && b <= b'Z') ||
                (b'a' <= b && b <= b'z') ||
                b == b' ' || b == b'\'' || b == b'(' || b == b')' ||
                b == b'+' || b == b',' || b == b'-' || b == b'.' ||
                b == b'/' || b == b':' || b == b'=' || b == b'?';
            if !ok {
                return None;
            }
        }
        return Some(PrintableString {
            string: String::from_utf8(bytes).unwrap(),
        });
    }
}

impl Deref for PrintableString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        return &self.string;
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UtcTime {
    bytes: Vec<u8>,
}

impl UtcTime {
    pub fn new(bytes: Vec<u8>) -> Self {
        return UtcTime {
            bytes: bytes,
        };
    }
}
