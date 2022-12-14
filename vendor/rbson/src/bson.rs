// The MIT License (MIT)

// Copyright (c) 2015 Y. T. Chung <zonyitoo@gmail.com>

// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! BSON definition

use std::{
    convert::{TryFrom, TryInto},
    fmt::{self, Debug, Display, Formatter},
};

use chrono::Datelike;
use serde_json::{json, Value};

pub use crate::document::Document;
use crate::{
    oid::{self, ObjectId},
    spec::{BinarySubtype, ElementType},
    Decimal128,
};

/// Possible BSON value types.
#[derive(Clone, PartialEq)]
pub enum Bson {
    /// 64-bit binary floating point
    Double(f64),
    /// UTF-8 string
    String(String),
    /// Array
    Array(Array),
    /// Embedded document
    Document(Document),
    /// Boolean value
    Boolean(bool),
    /// Null value
    Null,
    /// 32-bit signed integer
    Int32(i32),
    /// 64-bit signed integer
    Int64(i64),
    /// 32-bit signed integer
    UInt32(u32),
    /// 64-bit signed integer
    UInt64(u64),
    /// Timestamp
    Timestamp(Timestamp),
    /// Binary data
    Binary(Binary),
    /// UTC datetime
    DateTime(crate::DateTime),
    /// [128-bit decimal floating point](https://github.com/mongodb/specifications/blob/master/source/bson-decimal128/decimal128.rst)
    Decimal128(Decimal128),
}

/// Alias for `Vec<Bson>`.
pub type Array = Vec<Bson>;

impl Default for Bson {
    fn default() -> Self {
        Bson::Null
    }
}

impl Display for Bson {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Bson::Double(f) => write!(fmt, "{}", f),
            Bson::String(ref s) => write!(fmt, "\"{}\"", s),
            Bson::Array(ref vec) => {
                fmt.write_str("[")?;

                let mut first = true;
                for bson in vec {
                    if !first {
                        fmt.write_str(", ")?;
                    }

                    write!(fmt, "{}", bson)?;
                    first = false;
                }

                fmt.write_str("]")
            }
            Bson::Document(ref doc) => write!(fmt, "{}", doc),
            Bson::Boolean(b) => write!(fmt, "{}", b),
            Bson::Null => write!(fmt, "null"),
            Bson::Int32(i) => write!(fmt, "{}", i),
            Bson::Int64(i) => write!(fmt, "{}", i),
            Bson::UInt32(i) => write!(fmt, "{}", i),
            Bson::UInt64(i) => write!(fmt, "{}", i),
            Bson::Timestamp(ref x) => write!(fmt, "{}", x),
            Bson::Binary(ref x) => write!(fmt, "{}", x),
            Bson::DateTime(date_time) => write!(fmt, "DateTime(\"{}\")", date_time),
            Bson::Decimal128(ref d) => write!(fmt, "{}", d),
        }
    }
}

impl Debug for Bson {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match *self {
            Bson::Double(f) => fmt.debug_tuple("Double").field(&f).finish(),
            Bson::String(ref s) => fmt.debug_tuple("String").field(s).finish(),
            Bson::Array(ref vec) => {
                write!(fmt, "Array(")?;
                Debug::fmt(vec, fmt)?;
                write!(fmt, ")")
            }
            Bson::Document(ref doc) => Debug::fmt(doc, fmt),
            Bson::Boolean(b) => fmt.debug_tuple("Boolean").field(&b).finish(),
            Bson::Null => write!(fmt, "Null"),
            Bson::Int32(i) => fmt.debug_tuple("Int32").field(&i).finish(),
            Bson::Int64(i) => fmt.debug_tuple("Int64").field(&i).finish(),
            Bson::UInt32(i) => fmt.debug_tuple("UInt32").field(&i).finish(),
            Bson::UInt64(i) => fmt.debug_tuple("UInt64").field(&i).finish(),
            Bson::Timestamp(ref t) => Debug::fmt(t, fmt),
            Bson::Binary(ref b) => Debug::fmt(b, fmt),
            Bson::DateTime(ref date_time) => Debug::fmt(date_time, fmt),
            Bson::Decimal128(ref d) => Debug::fmt(d, fmt),
        }
    }
}

impl From<f32> for Bson {
    fn from(a: f32) -> Bson {
        Bson::Double(a.into())
    }
}

impl From<f64> for Bson {
    fn from(a: f64) -> Bson {
        Bson::Double(a)
    }
}

impl From<&str> for Bson {
    fn from(s: &str) -> Bson {
        Bson::String(s.to_owned())
    }
}

impl From<String> for Bson {
    fn from(a: String) -> Bson {
        Bson::String(a)
    }
}

impl From<Document> for Bson {
    fn from(a: Document) -> Bson {
        Bson::Document(a)
    }
}

impl From<bool> for Bson {
    fn from(a: bool) -> Bson {
        Bson::Boolean(a)
    }
}

impl From<Binary> for Bson {
    fn from(binary: Binary) -> Bson {
        Bson::Binary(binary)
    }
}

impl From<Timestamp> for Bson {
    fn from(ts: Timestamp) -> Bson {
        Bson::Timestamp(ts)
    }
}

impl<T> From<&T> for Bson
where
    T: Clone + Into<Bson>,
{
    fn from(t: &T) -> Bson {
        t.clone().into()
    }
}

impl<T> From<Vec<T>> for Bson
where
    T: Into<Bson>,
{
    fn from(v: Vec<T>) -> Bson {
        Bson::Array(v.into_iter().map(|val| val.into()).collect())
    }
}

impl<T> From<&[T]> for Bson
where
    T: Clone + Into<Bson>,
{
    fn from(s: &[T]) -> Bson {
        Bson::Array(s.iter().cloned().map(|val| val.into()).collect())
    }
}

impl<T: Into<Bson>> ::std::iter::FromIterator<T> for Bson {
    /// # Examples
    ///
    /// ```
    /// use std::iter::FromIterator;
    /// use bson::Bson;
    ///
    /// let x: Bson = Bson::from_iter(vec!["lorem", "ipsum", "dolor"]);
    /// // or
    /// let x: Bson = vec!["lorem", "ipsum", "dolor"].into_iter().collect();
    /// ```
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Bson::Array(iter.into_iter().map(Into::into).collect())
    }
}

impl From<i32> for Bson {
    fn from(a: i32) -> Bson {
        Bson::Int32(a)
    }
}

impl From<i64> for Bson {
    fn from(a: i64) -> Bson {
        Bson::Int64(a)
    }
}

impl From<u32> for Bson {
    fn from(a: u32) -> Bson {
        Bson::UInt32(a)
    }
}

impl From<u64> for Bson {
    fn from(a: u64) -> Bson {
        Bson::UInt64(a)
    }
}


#[cfg(feature = "chrono-0_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "chrono-0_4")))]
impl<T: chrono::TimeZone> From<chrono::DateTime<T>> for Bson {
    fn from(a: chrono::DateTime<T>) -> Bson {
        Bson::DateTime(crate::DateTime::from(a))
    }
}

#[cfg(feature = "uuid-0_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "uuid-0_8")))]
impl From<uuid::Uuid> for Bson {
    fn from(uuid: uuid::Uuid) -> Self {
        Bson::Binary(uuid.into())
    }
}

impl From<crate::DateTime> for Bson {
    fn from(dt: crate::DateTime) -> Self {
        Bson::DateTime(dt)
    }
}

impl<T> From<Option<T>> for Bson
where
    T: Into<Bson>,
{
    fn from(a: Option<T>) -> Bson {
        match a {
            None => Bson::Null,
            Some(t) => t.into(),
        }
    }
}

/// This will create the [relaxed Extended JSON v2](https://docs.mongodb.com/manual/reference/mongodb-extended-json/) representation of the provided [`Bson`](../enum.Bson.html).
impl From<Bson> for Value {
    fn from(bson: Bson) -> Self {
        bson.into_relaxed_extjson()
    }
}

impl Bson {
    /// Converts the Bson value into its [relaxed extended JSON representation](https://docs.mongodb.com/manual/reference/mongodb-extended-json/).
    ///
    /// Note: If this method is called on a case which contains a `Decimal128` value, it will panic.
    pub fn into_relaxed_extjson(self) -> Value {
        match self {
            Bson::Double(v) if v.is_nan() => {
                let s = if v.is_sign_negative() { "-NaN" } else { "NaN" };

                json!({ "$numberDouble": s })
            }
            Bson::Double(v) if v.is_infinite() => {
                let s = if v.is_sign_negative() {
                    "-Infinity"
                } else {
                    "Infinity"
                };

                json!({ "$numberDouble": s })
            }
            Bson::Double(v) => json!(v),
            Bson::String(v) => json!(v),
            Bson::Array(v) => json!(v),
            Bson::Document(v) => {
                Value::Object(v.into_iter().map(|(k, v)| (k, Value::from(v))).collect())
            }
            Bson::Boolean(v) => json!(v),
            Bson::Null => Value::Null,
            Bson::Int32(v) => v.into(),
            Bson::Int64(v) => v.into(),
            Bson::UInt32(v) => v.into(),
            Bson::UInt64(v) => v.into(),
            Bson::Timestamp(Timestamp { time, increment }) => json!({
                "$timestamp": {
                    "t": time,
                    "i": increment,
                }
            }),
            Bson::Binary(Binary { subtype, ref bytes }) => {
                let tval: u8 = From::from(subtype);
                json!({
                    "$binary": {
                        "base64": base64::encode(bytes),
                        "subType": hex::encode([tval]),
                    }
                })
            }
            Bson::DateTime(v) if v.timestamp_millis() >= 0 && v.to_chrono().year() <= 99999 => {
                json!({
                    "$date": v.to_rfc3339_string(),
                })
            }
            Bson::DateTime(v) => json!({
                "$date": { "$numberLong": v.timestamp_millis().to_string() },
            }),
            Bson::Decimal128(_) => panic!("Decimal128 extended JSON not implemented yet."),
        }
    }

    /// Converts the Bson value into its [canonical extended JSON representation](https://docs.mongodb.com/manual/reference/mongodb-extended-json/).
    ///
    /// Note: extended json encoding for `Decimal128` values is not supported. If this method is
    /// called on a case which contains a `Decimal128` value, it will panic.
    pub fn into_canonical_extjson(self) -> Value {
        match self {
            Bson::Int32(i) => json!({ "$numberInt": i.to_string() }),
            Bson::Int64(i) => json!({ "$numberLong": i.to_string() }),
            Bson::UInt32(i) => json!({ "$numberUInt32": i.to_string() }),
            Bson::UInt64(i) => json!({ "$numberUInt64": i.to_string() }),
            Bson::Double(f) if f.is_normal() => {
                let mut s = f.to_string();
                if f.fract() == 0.0 {
                    s.push_str(".0");
                }

                json!({ "$numberDouble": s })
            }
            Bson::Double(f) if f == 0.0 => {
                let s = if f.is_sign_negative() { "-0.0" } else { "0.0" };

                json!({ "$numberDouble": s })
            }
            Bson::DateTime(date) => {
                json!({ "$date": { "$numberLong": date.timestamp_millis().to_string() } })
            }
            Bson::Array(arr) => {
                Value::Array(arr.into_iter().map(Bson::into_canonical_extjson).collect())
            }
            Bson::Document(arr) => Value::Object(
                arr.into_iter()
                    .map(|(k, v)| (k, v.into_canonical_extjson()))
                    .collect(),
            ),
            other => other.into_relaxed_extjson(),
        }
    }

    /// Get the `ElementType` of this value.
    pub fn element_type(&self) -> ElementType {
        match *self {
            Bson::Double(..) => ElementType::Double,
            Bson::String(..) => ElementType::String,
            Bson::Array(..) => ElementType::Array,
            Bson::Document(..) => ElementType::EmbeddedDocument,
            Bson::Boolean(..) => ElementType::Boolean,
            Bson::Null => ElementType::Null,
            Bson::Int32(..) => ElementType::Int32,
            Bson::Int64(..) => ElementType::Int64,
            Bson::UInt32(..) => ElementType::UInt32,
            Bson::UInt64(..) => ElementType::UInt64,
            Bson::Timestamp(..) => ElementType::Timestamp,
            Bson::Binary(..) => ElementType::Binary,
            Bson::DateTime(..) => ElementType::DateTime,
            Bson::Decimal128(..) => ElementType::Decimal128,
        }
    }

    /// Converts to extended format.
    /// This function mainly used for [extended JSON format](https://docs.mongodb.com/manual/reference/mongodb-extended-json/).
    // TODO RUST-426: Investigate either removing this from the serde implementation or unifying
    // with the extended JSON implementation.
    pub(crate) fn into_extended_document(self) -> Document {
        match self {
            Bson::Timestamp(Timestamp { time, increment }) => {
                doc! {
                    "$timestamp": {
                        "t": time,
                        "i": increment,
                    }
                }
            }
            Bson::Binary(Binary { subtype, ref bytes }) => {
                let tval: u8 = From::from(subtype);
                doc! {
                    "$binary": {
                        "base64": base64::encode(bytes),
                        "subType": hex::encode([tval]),
                    }
                }
            }
            Bson::DateTime(v) if v.timestamp_millis() >= 0 && v.to_chrono().year() <= 9999 => {
                doc! {
                    "$date": v.to_rfc3339_string(),
                }
            }
            Bson::DateTime(v) => doc! {
                "$date": { "$numberLong": v.timestamp_millis().to_string() },
            },
            _ => panic!("Attempted conversion of invalid data type: {}", self),
        }
    }

    pub(crate) fn from_extended_document(doc: Document) -> Bson {
        if doc.len() > 2 {
            return Bson::Document(doc);
        }

        let mut keys: Vec<_> = doc.keys().map(|s| s.as_str()).collect();
        keys.sort_unstable();

        match keys.as_slice() {
            ["$numberInt"] => {
                if let Ok(i) = doc.get_str("$numberInt") {
                    if let Ok(i) = i.parse() {
                        return Bson::Int32(i);
                    }
                }
            }

            ["$numberLong"] => {
                if let Ok(i) = doc.get_str("$numberLong") {
                    if let Ok(i) = i.parse() {
                        return Bson::Int64(i);
                    }
                }
            }

            ["$numberDouble"] => match doc.get_str("$numberDouble") {
                Ok("Infinity") => return Bson::Double(std::f64::INFINITY),
                Ok("-Infinity") => return Bson::Double(std::f64::NEG_INFINITY),
                Ok("NaN") => return Bson::Double(std::f64::NAN),
                Ok(other) => {
                    if let Ok(d) = other.parse() {
                        return Bson::Double(d);
                    }
                }
                _ => {}
            },

            ["$numberDecimalBytes"] => {
                if let Ok(bytes) = doc.get_binary_generic("$numberDecimalBytes") {
                    if let Ok(b) = bytes.clone().try_into() {
                        return Bson::Decimal128(Decimal128 { bytes: b });
                    }
                }
            }

            ["$binary"] => {
                if let Some(binary) = Binary::from_extended_doc(&doc) {
                    return Bson::Binary(binary);
                }
            }

            ["$timestamp"] => {
                if let Ok(timestamp) = doc.get_document("$timestamp") {
                    if let Ok(t) = timestamp.get_i32("t") {
                        if let Ok(i) = timestamp.get_i32("i") {
                            return Bson::Timestamp(Timestamp {
                                time: t as u32,
                                increment: i as u32,
                            });
                        }
                    }

                    if let Ok(t) = timestamp.get_i64("t") {
                        if let Ok(i) = timestamp.get_i64("i") {
                            if t >= 0
                                && i >= 0
                                && t <= (std::u32::MAX as i64)
                                && i <= (std::u32::MAX as i64)
                            {
                                return Bson::Timestamp(Timestamp {
                                    time: t as u32,
                                    increment: i as u32,
                                });
                            }
                        }
                    }
                }
            }

            ["$date"] => {
                if let Ok(date) = doc.get_i64("$date") {
                    return Bson::DateTime(crate::DateTime::from_millis(date));
                }

                if let Ok(date) = doc.get_str("$date") {
                    if let Ok(date) = chrono::DateTime::parse_from_rfc3339(date) {
                        return Bson::DateTime(crate::DateTime::from_chrono(date));
                    }
                }
            }

            _ => {}
        };

        Bson::Document(
            doc.into_iter()
                .map(|(k, v)| {
                    let v = match v {
                        Bson::Document(v) => Bson::from_extended_document(v),
                        other => other,
                    };

                    (k, v)
                })
                .collect(),
        )
    }
}

/// Value helpers
impl Bson {
    /// If `Bson` is `Double`, return its value as an `f64`. Returns `None` otherwise
    pub fn as_f64(&self) -> Option<f64> {
        match *self {
            Bson::Double(v) => Some(v),
            _ => None,
        }
    }

    /// If `Bson` is `String`, return its value as a `&str`. Returns `None` otherwise
    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Bson::String(ref s) => Some(s),
            _ => None,
        }
    }

    /// If `Bson` is `String`, return a mutable reference to its value as a `str`. Returns `None`
    /// otherwise
    pub fn as_str_mut(&mut self) -> Option<&mut str> {
        match *self {
            Bson::String(ref mut s) => Some(s),
            _ => None,
        }
    }

    /// If `Bson` is `Array`, return its value. Returns `None` otherwise
    pub fn as_array(&self) -> Option<&Array> {
        match *self {
            Bson::Array(ref v) => Some(v),
            _ => None,
        }
    }

    /// If `Bson` is `Array`, return a mutable reference to its value. Returns `None` otherwise
    pub fn as_array_mut(&mut self) -> Option<&mut Array> {
        match *self {
            Bson::Array(ref mut v) => Some(v),
            _ => None,
        }
    }

    /// If `Bson` is `Document`, return its value. Returns `None` otherwise
    pub fn as_document(&self) -> Option<&Document> {
        match *self {
            Bson::Document(ref v) => Some(v),
            _ => None,
        }
    }

    /// If `Bson` is `Document`, return a mutable reference to its value. Returns `None` otherwise
    pub fn as_document_mut(&mut self) -> Option<&mut Document> {
        match *self {
            Bson::Document(ref mut v) => Some(v),
            _ => None,
        }
    }

    /// If `Bson` is `Bool`, return its value. Returns `None` otherwise
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Bson::Boolean(v) => Some(v),
            _ => None,
        }
    }

    /// If `Bson` is `I32`, return its value. Returns `None` otherwise
    pub fn as_i32(&self) -> Option<i32> {
        match *self {
            Bson::Int32(v) => Some(v),
            _ => None,
        }
    }

    /// If `Bson` is `I64`, return its value. Returns `None` otherwise
    pub fn as_i64(&self) -> Option<i64> {
        match *self {
            Bson::Int64(v) => Some(v),
            _ => None,
        }
    }

    /// If `Bson` is `U32`, return its value. Returns `None` otherwise
    pub fn as_u32(&self) -> Option<u32> {
        match *self {
            Bson::UInt32(v) => Some(v),
            _ => None,
        }
    }

    /// If `Bson` is `U64`, return its value. Returns `None` otherwise
    pub fn as_u64(&self) -> Option<u64> {
        match *self {
            Bson::UInt64(v) => Some(v),
            _ => None,
        }
    }

    /// If `Bson` is `DateTime`, return its value. Returns `None` otherwise
    pub fn as_datetime(&self) -> Option<&crate::DateTime> {
        match *self {
            Bson::DateTime(ref v) => Some(v),
            _ => None,
        }
    }

    /// If `Bson` is `DateTime`, return a mutable reference to its value. Returns `None`
    /// otherwise
    pub fn as_datetime_mut(&mut self) -> Option<&mut crate::DateTime> {
        match *self {
            Bson::DateTime(ref mut v) => Some(v),
            _ => None,
        }
    }

    /// If `Bson` is `Timestamp`, return its value. Returns `None` otherwise
    pub fn as_timestamp(&self) -> Option<Timestamp> {
        match *self {
            Bson::Timestamp(timestamp) => Some(timestamp),
            _ => None,
        }
    }

    /// If `Bson` is `Null`, return its value. Returns `None` otherwise
    pub fn as_null(&self) -> Option<()> {
        match *self {
            Bson::Null => Some(()),
            _ => None,
        }
    }

}

/// Represents a BSON timestamp value.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy, Hash)]
pub struct Timestamp {
    /// The number of seconds since the Unix epoch.
    pub time: u32,

    /// An incrementing value to order timestamps with the same number of seconds in the `time`
    /// field.
    pub increment: u32,
}

impl Display for Timestamp {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Timestamp({}, {})", self.time, self.increment)
    }
}

impl Timestamp {
    pub(crate) fn to_le_i64(self) -> i64 {
        let upper = (self.time.to_le() as u64) << 32;
        let lower = self.increment.to_le() as u64;

        (upper | lower) as i64
    }

    pub(crate) fn from_le_i64(val: i64) -> Self {
        let ts = val.to_le();

        Timestamp {
            time: ((ts as u64) >> 32) as u32,
            increment: (ts & 0xFFFF_FFFF) as u32,
        }
    }
}


/// Represents a BSON binary value.
#[derive(Debug, Clone, PartialEq)]
pub struct Binary {
    /// The subtype of the bytes.
    pub subtype: BinarySubtype,

    /// The binary bytes.
    pub bytes: Vec<u8>,
}

impl Display for Binary {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "Binary({:#x}, {})",
            u8::from(self.subtype),
            base64::encode(&self.bytes)
        )
    }
}

impl Binary {
    pub(crate) fn from_extended_doc(doc: &Document) -> Option<Self> {
        let binary_doc = doc.get_document("$binary").ok()?;

        if let Ok(bytes) = binary_doc.get_str("base64") {
            let bytes = base64::decode(bytes).ok()?;
            let subtype = binary_doc.get_str("subType").ok()?;
            let subtype = hex::decode(subtype).ok()?;
            if subtype.len() == 1 {
                Some(Self {
                    bytes,
                    subtype: subtype[0].into(),
                })
            } else {
                None
            }
        } else {
            // in non-human-readable mode, RawBinary will serialize as
            // { "$binary": { "bytes": <bytes>, "subType": <i32> } };
            let binary = binary_doc.get_binary_generic("bytes").ok()?;
            let subtype = binary_doc.get_i32("subType").ok()?;

            Some(Self {
                bytes: binary.clone(),
                subtype: u8::try_from(subtype).ok()?.into(),
            })
        }
    }
}
