// Rust Bitcoin Library
// Written in 2014 by
//     Andrew Poelstra <apoelstra@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! Stream reader
//!
//! This module defines `StreamReader` struct and its implementation which is used
//! for parsing incoming stream into separate `RawNetworkMessage`s, handling assembling
//! messages from multiple packets or dealing with partial or multiple messages in the stream
//! (like can happen with reading from TCP socket)
//!

use std::fmt;
use std::io::{self, Read};

use crate::{Error, Decodable, deserialize_partial};

/// Struct used to configure stream reader function
pub struct StreamReader<R: Read> {
    /// Stream to read from
    pub stream: R,
    /// I/O buffer
    data: Vec<u8>,
    /// Buffer containing unparsed message part
    unparsed: Vec<u8>
}

impl<R: Read> fmt::Debug for StreamReader<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StreamReader with buffer_size={} and unparsed content {:?}",
               self.data.capacity(), self.unparsed)
    }
}

impl<R: Read> StreamReader<R> {
    /// Constructs new stream reader for a given input stream `stream` with
    /// optional parameter `buffer_size` determining reading buffer size
    pub fn new(stream: R, buffer_size: Option<usize>) -> StreamReader<R> {
        StreamReader {
            stream,
            data: vec![0u8; buffer_size.unwrap_or(64 * 1024)],
            unparsed: vec![]
        }
    }

    /// Reads stream and parses next message from its current input,
    /// also taking into account previously unparsed partial message (if there was such).
    pub fn read_next<D: Decodable>(&mut self) -> Result<D, Error> {
        loop {
            match deserialize_partial::<D>(&self.unparsed) {
                // In this case we just have an incomplete data, so we need to read more
                Err(Error::Io(ref err)) if err.kind () == io::ErrorKind::UnexpectedEof => {
                    let count = self.stream.read(&mut self.data)?;
                    if count > 0 {
                        self.unparsed.extend(self.data[0..count].iter());
                    }
                    else {
                        return Err(Error::Io(io::Error::from(io::ErrorKind::UnexpectedEof)));
                    }
                },
                Err(err) => return Err(err),
                // We have successfully read from the buffer
                Ok((message, index)) => {
                    self.unparsed.drain(..index);
                    return Ok(message)
                },
            }
        }
    }
}
