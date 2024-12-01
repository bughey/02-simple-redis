use super::{
    BulkString, RespArray, RespEncode, RespFrame, RespNull, RespNullArray, RespNullBulkString,
    SimpleError, SimpleString,
};

impl RespEncode for RespFrame {
    fn encode(self) -> Vec<u8> {
        match self {
            RespFrame::SimpleString(s) => s.encode(),
            RespFrame::Error(e) => e.encode(),
            RespFrame::Integer(i) => i.encode(),
            RespFrame::BulkString(b) => b.encode(),
            RespFrame::NullBulkString(n) => n.encode(),
            RespFrame::Array(a) => a.encode(),
            RespFrame::Null(n) => n.encode(),
            RespFrame::NullArray(n) => n.encode(),
            // RespFrame::Boolean(b) => b.encode(),
            // RespFrame::Double(d) => d.encode(),
            // RespFrame::BigNumber(b) => b.encode(),
            // RespFrame::Map(m) => m.encode(),
            // RespFrame::Set(s) => s.encode(),
            _ => unimplemented!("Not implemented yet"),
        }
    }
}

impl RespEncode for i64 {
    fn encode(self) -> Vec<u8> {
        let sign = if self < 0 { "" } else { "+" };
        format!(":{}{}\r\n", sign, self).into_bytes()
    }
}

impl RespEncode for SimpleString {
    fn encode(self) -> Vec<u8> {
        format!("+{}\r\n", self.0).into_bytes()
    }
}

impl RespEncode for SimpleError {
    fn encode(self) -> Vec<u8> {
        format!("-{}\r\n", self.0).into_bytes()
    }
}

impl RespEncode for BulkString {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.len() + 16);
        buf.extend_from_slice(&format!("${}\r\n", self.len()).into_bytes());
        buf.extend_from_slice(&self);
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

// RespNullBulkString
impl RespEncode for RespNullBulkString {
    fn encode(self) -> Vec<u8> {
        b"$-1\r\n".to_vec()
    }
}

const ARRAY_CAP: usize = 4096;
// RespArray
impl RespEncode for RespArray {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(ARRAY_CAP);
        buf.extend_from_slice(&format!("*{}\r\n", self.len()).into_bytes());
        for frame in self.0 {
            buf.extend_from_slice(&frame.encode());
        }
        buf
    }
}

// RespNullArray
impl RespEncode for RespNullArray {
    fn encode(self) -> Vec<u8> {
        b"*-1\r\n".to_vec()
    }
}

// RespNull
impl RespEncode for RespNull {
    fn encode(self) -> Vec<u8> {
        b"_\r\n".to_vec()
    }
}

// bool
impl RespEncode for bool {
    fn encode(self) -> Vec<u8> {
        if self {
            b"#t\r\n".to_vec()
        } else {
            b"#f\r\n".to_vec()
        }
    }
}

// f64
impl RespEncode for f64 {
    fn encode(self) -> Vec<u8> {
        format!("$,{:+e}\r\n", self).into_bytes()
    }
}