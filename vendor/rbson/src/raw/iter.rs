use std::convert::TryInto;

use crate::{
    de::{read_bool, MIN_BSON_DOCUMENT_SIZE, MIN_CODE_WITH_SCOPE_SIZE},
    oid::ObjectId,
    raw::{Error, ErrorKind, Result},
    spec::{BinarySubtype, ElementType},
    DateTime,
    Decimal128,
    Timestamp,
};
use crate::raw::{u32_from_slice, u64_from_slice};

use super::{
    checked_add,
    error::try_with_key,
    f64_from_slice,
    i32_from_slice,
    i64_from_slice,
    read_lenencoded,
    read_nullterminated,
    RawArray,
    RawBinary,
    RawBson,
    RawDocument,
};

/// An iterator over the document's entries.
pub struct Iter<'a> {
    doc: &'a RawDocument,
    offset: usize,

    /// Whether the underlying doc is assumed to be valid or if an error has been encountered.
    /// After an error, all subsequent iterations will return None.
    valid: bool,
}

impl<'a> Iter<'a> {
    pub(crate) fn new(doc: &'a RawDocument) -> Self {
        Self {
            doc,
            offset: 4,
            valid: true,
        }
    }

    fn verify_enough_bytes(&self, start: usize, num_bytes: usize) -> Result<()> {
        let end = checked_add(start, num_bytes)?;
        if self.doc.as_bytes().get(start..end).is_none() {
            return Err(Error::new_without_key(ErrorKind::MalformedValue {
                message: format!(
                    "length exceeds remaining length of buffer: {} vs {}",
                    num_bytes,
                    self.doc.as_bytes().len() - start
                ),
            }));
        }
        Ok(())
    }

    fn next_oid(&self, starting_at: usize) -> Result<ObjectId> {
        self.verify_enough_bytes(starting_at, 12)?;
        let oid = ObjectId::from_bytes(
            self.doc.as_bytes()[starting_at..(starting_at + 12)]
                .try_into()
                .unwrap(), // ok because we know slice is 12 bytes long
        );
        Ok(oid)
    }

    fn next_document(&self, starting_at: usize) -> Result<&'a RawDocument> {
        self.verify_enough_bytes(starting_at, MIN_BSON_DOCUMENT_SIZE as usize)?;
        let size = i32_from_slice(&self.doc.as_bytes()[starting_at..])? as usize;

        if size < MIN_BSON_DOCUMENT_SIZE as usize {
            return Err(Error::new_without_key(ErrorKind::MalformedValue {
                message: format!("document too small: {} bytes", size),
            }));
        }

        self.verify_enough_bytes(starting_at, size)?;
        let end = starting_at + size;

        if self.doc.as_bytes()[end - 1] != 0 {
            return Err(Error {
                key: None,
                kind: ErrorKind::MalformedValue {
                    message: "not null terminated".into(),
                },
            });
        }
        RawDocument::new(&self.doc.as_bytes()[starting_at..end])
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Result<(&'a str, RawBson<'a>)>;

    fn next(&mut self) -> Option<Result<(&'a str, RawBson<'a>)>> {
        if !self.valid {
            return None;
        } else if self.offset == self.doc.as_bytes().len() - 1 {
            if self.doc.as_bytes()[self.offset] == 0 {
                // end of document marker
                return None;
            } else {
                self.valid = false;
                return Some(Err(Error {
                    key: None,
                    kind: ErrorKind::MalformedValue {
                        message: "document not null terminated".into(),
                    },
                }));
            }
        } else if self.offset >= self.doc.as_bytes().len() {
            self.valid = false;
            return Some(Err(Error::new_without_key(ErrorKind::MalformedValue {
                message: "iteration overflowed document".to_string(),
            })));
        }

        let key = match read_nullterminated(&self.doc.as_bytes()[self.offset + 1..]) {
            Ok(k) => k,
            Err(e) => {
                self.valid = false;
                return Some(Err(e));
            }
        };

        let kvp_result = try_with_key(key, || {
            let valueoffset = self.offset + 1 + key.len() + 1; // type specifier + key + \0

            let element_type = match ElementType::from(self.doc.as_bytes()[self.offset]) {
                Some(et) => et,
                None => {
                    return Err(Error::new_with_key(
                        key,
                        ErrorKind::MalformedValue {
                            message: format!("invalid tag: {}", self.doc.as_bytes()[self.offset]),
                        },
                    ))
                }
            };

            let (element, element_size) = match element_type {
                ElementType::Int32 => {
                    let i = i32_from_slice(&self.doc.as_bytes()[valueoffset..])?;
                    (RawBson::Int32(i), 4)
                }
                ElementType::Int64 => {
                    let i = i64_from_slice(&self.doc.as_bytes()[valueoffset..])?;
                    (RawBson::Int64(i), 8)
                }
                ElementType::UInt32 => {
                    let i = u32_from_slice(&self.doc.as_bytes()[valueoffset..])?;
                    (RawBson::UInt32(i), 4)
                }
                ElementType::UInt64 => {
                    let i = u64_from_slice(&self.doc.as_bytes()[valueoffset..])?;
                    (RawBson::UInt64(i), 8)
                }
                ElementType::Double => {
                    let f = f64_from_slice(&self.doc.as_bytes()[valueoffset..])?;
                    (RawBson::Double(f), 8)
                }
                ElementType::String => {
                    let s = read_lenencoded(&self.doc.as_bytes()[valueoffset..])?;
                    (RawBson::String(s), 4 + s.len() + 1)
                }
                ElementType::EmbeddedDocument => {
                    let doc = self.next_document(valueoffset)?;
                    (RawBson::Document(doc), doc.as_bytes().len())
                }
                ElementType::Array => {
                    let doc = self.next_document(valueoffset)?;
                    (
                        RawBson::Array(RawArray::from_doc(doc)),
                        doc.as_bytes().len(),
                    )
                }
                ElementType::Binary => {
                    let len = i32_from_slice(&self.doc.as_bytes()[valueoffset..])? as usize;
                    let data_start = valueoffset + 4 + 1;
                    self.verify_enough_bytes(valueoffset, len)?;
                    let subtype = BinarySubtype::from(self.doc.as_bytes()[valueoffset + 4]);
                    let data = match subtype {
                        BinarySubtype::BinaryOld => {
                            if len < 4 {
                                return Err(Error::new_without_key(ErrorKind::MalformedValue {
                                    message: "old binary subtype has no inner declared length"
                                        .into(),
                                }));
                            }
                            let oldlength =
                                i32_from_slice(&self.doc.as_bytes()[data_start..])? as usize;
                            if checked_add(oldlength, 4)? != len {
                                return Err(Error::new_without_key(ErrorKind::MalformedValue {
                                    message: "old binary subtype has wrong inner declared length"
                                        .into(),
                                }));
                            }
                            &self.doc.as_bytes()[(data_start + 4)..(data_start + len)]
                        }
                        _ => &self.doc.as_bytes()[data_start..(data_start + len)],
                    };
                    (
                        RawBson::Binary(RawBinary {
                            subtype,
                            bytes: data,
                        }),
                        4 + 1 + len,
                    )
                }
                ElementType::Boolean => {
                    let b = read_bool(&self.doc.as_bytes()[valueoffset..]).map_err(|e| {
                        Error::new_with_key(
                            key,
                            ErrorKind::MalformedValue {
                                message: e.to_string(),
                            },
                        )
                    })?;
                    (RawBson::Boolean(b), 1)
                }
                ElementType::DateTime => {
                    let ms = i64_from_slice(&self.doc.as_bytes()[valueoffset..])?;
                    (RawBson::DateTime(DateTime::from_millis(ms)), 8)
                }
                ElementType::Null => (RawBson::Null, 0),
                ElementType::Timestamp => {
                    let ts = Timestamp::from_reader(&self.doc.as_bytes()[valueoffset..]).map_err(
                        |e| {
                            Error::new_without_key(ErrorKind::MalformedValue {
                                message: e.to_string(),
                            })
                        },
                    )?;
                    (RawBson::Timestamp(ts), 8)
                }
                ElementType::Decimal128 => {
                    self.verify_enough_bytes(valueoffset, 16)?;
                    (
                        RawBson::Decimal128(Decimal128::from_bytes(
                            self.doc.as_bytes()[valueoffset..(valueoffset + 16)]
                                .try_into()
                                .unwrap(),
                        )),
                        16,
                    )
                }
            };

            self.offset = valueoffset + element_size;
            self.verify_enough_bytes(valueoffset, element_size)?;

            Ok((key, element))
        });

        if kvp_result.is_err() {
            self.valid = false;
        }

        Some(kvp_result)
    }
}
