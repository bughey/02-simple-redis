use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

pub mod decode;
pub mod encode;

pub trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

pub trait RespDecode {
    fn decode(buf: Self) -> Result<RespFrame, String>;
}

pub enum RespFrame {
    SimpleString(SimpleString),
    Error(SimpleError),
    Integer(i64),
    BulkString(BulkString),
    NullBulkString(RespNullBulkString),
    Array(RespArray),
    Null(RespNull),
    NullArray(RespNullArray),
    Boolean(bool),
    Double(f64),
    BigNumber(Vec<u8>),
    Map(HashMap<String, RespFrame>),
    Set(HashSet<RespFrame>),
}

pub struct SimpleString(String);
pub struct SimpleError(String);
pub struct BulkString(Vec<u8>);
pub struct RespArray(Vec<RespFrame>);
pub struct RespNull;
pub struct RespNullArray;
pub struct RespNullBulkString;

// impl Deref for SimpleString, SimpleError, BulkString
impl Deref for SimpleString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for SimpleError {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for BulkString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for RespArray {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
