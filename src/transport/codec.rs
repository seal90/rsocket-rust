use crate::errors::RSocketError;
use crate::frame::{Frame, Writeable, U24};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::io::{Error, ErrorKind};
use tokio_util::codec::{Decoder, Encoder};

pub struct RFrameCodec;

impl Decoder for RFrameCodec {
    type Item = Frame;
    type Error = Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let actual = buf.len();
        if actual < 3 {
            return Ok(None);
        }
        let l = U24::read(buf) as usize;
        if actual < 3 + l {
            return Ok(None);
        }
        buf.advance(3);
        let mut bb = buf.split_to(l);
        match Frame::decode(&mut bb) {
            Ok(v) => Ok(Some(v)),
            Err(e) => Err(Error::from(ErrorKind::InvalidInput)),
        }
    }
}

impl Encoder for RFrameCodec {
    type Item = Frame;
    type Error = Error;
    fn encode(&mut self, item: Frame, buf: &mut BytesMut) -> Result<(), Self::Error> {
        let sid = item.get_stream_id();
        let l = item.len();
        buf.reserve(3 + l);
        U24::write(l as u32, buf);
        item.write_to(buf);
        Ok(())
    }
}
