//! Streaming XOR cipher adaptor.

use std::borrow::Borrow;

/// A stateful streaming XOR munger that holds a reference to a key.
///
/// XORs a cycling key with arbitrary data. Stateful: position in the key
/// advances with each byte processed, so repeated calls on the same instance
/// produce different results for identical inputs.
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    pos: usize,
}

impl<'a> Xorcism<'a> {
    /// Creates a new `Xorcism` munger from a key.
    ///
    /// Accepts anything with a cheap conversion to a byte slice, such as
    /// `&str` or `&[u8]`.
    ///
    /// # Panics
    ///
    /// Panics if `key` is empty.
    #[must_use]
    pub fn new<Key: AsRef<[u8]> + ?Sized>(key: &'a Key) -> Self {
        let key = key.as_ref();
        assert!(!key.is_empty(), "key must not be empty");
        Self { key, pos: 0 }
    }

    /// XORs each byte of `data` in place with a byte from the key.
    ///
    /// Stateful: repeated calls produce different results for identical inputs,
    /// because the key position advances across calls.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        for byte in data.iter_mut() {
            *byte ^= self.key[self.pos % self.key.len()];
            self.pos += 1;
        }
    }

    /// Returns an iterator that XORs each byte of `data` with a byte from the key.
    ///
    /// Stateful: repeated calls produce different results for identical inputs,
    /// because the key position advances across calls. Accepts owned or borrowed
    /// byte iterators.
    #[must_use = "iterators are lazy and do nothing unless consumed"]
    pub fn munge<'b, Data>(&'b mut self, data: Data) -> impl Iterator<Item = u8> + 'b
    where
        Data: IntoIterator + 'b,
        Data::Item: Borrow<u8>,
    {
        let key = self.key;
        let pos = &mut self.pos;
        data.into_iter().map(move |b| {
            let key_byte = key[*pos % key.len()];
            *pos += 1;
            *b.borrow() ^ key_byte
        })
    }

    /// Wraps a reader, XOR-ing all bytes read from it with the key.
    #[cfg(feature = "io")]
    #[must_use]
    pub fn reader<R: std::io::Read>(self, reader: R) -> impl std::io::Read + 'a {
        XorcismReader {
            xorcism: self,
            reader,
        }
    }

    /// Wraps a writer, XOR-ing all bytes before passing them to the underlying writer.
    #[cfg(feature = "io")]
    #[must_use]
    pub fn writer<W: std::io::Write>(self, writer: W) -> impl std::io::Write + 'a {
        XorcismWriter {
            xorcism: self,
            writer,
        }
    }
}

#[cfg(feature = "io")]
struct XorcismReader<'a, R> {
    xorcism: Xorcism<'a>,
    reader: R,
}

#[cfg(feature = "io")]
impl<'a, R: std::io::Read> std::io::Read for XorcismReader<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes_read = self.reader.read(buf)?;
        self.xorcism.munge_in_place(&mut buf[..bytes_read]);
        Ok(bytes_read)
    }
}

#[cfg(feature = "io")]
struct XorcismWriter<'a, W> {
    xorcism: Xorcism<'a>,
    writer: W,
}

#[cfg(feature = "io")]
impl<'a, W: std::io::Write> std::io::Write for XorcismWriter<'a, W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut munged = buf.to_vec();
        self.xorcism.munge_in_place(&mut munged);
        self.writer.write_all(&munged)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
