use std::error;
use std::fmt;
use std::io::{self, Read, Seek, SeekFrom, Write};

const IPS_FILE_HEADER: &[u8] = b"PATCH";
const IPS_FILE_EOF: &[u8] = b"EOF";

pub fn apply_patch<R: Read, W: Write + Seek>(
  ips_file: &mut R,
  rom_file: &mut R,
  out_file: &mut W,
) -> Result<(), Error> {
  let mut ips_bytes = ips_file.bytes();

  let header = ips_bytes
    .by_ref()
    .take(IPS_FILE_HEADER.len())
    .collect::<Result<Vec<_>, _>>()?;

  if header.as_slice() != IPS_FILE_HEADER {
    return Err(Error::ExpectedHeader);
  }

  io::copy(rom_file, out_file)?;

  while let Some(record) = Record::try_from_bytes(&mut ips_bytes)? {
    let patch_bytes = match record.run_length {
      Some(length) => vec![
        ips_bytes.next().ok_or(Error::UnexpectedEof)??;
        length.into()
      ],

      None => ips_bytes
        .by_ref()
        .take(record.size)
        .collect::<Result<Vec<_>, _>>()?
    };

    out_file.seek(SeekFrom::Start(record.offset))?;

    io::copy(&mut patch_bytes.as_slice(), out_file)?;
  }

  Ok(())
}

#[allow(clippy::enum_variant_names)]
pub enum Error {
  ExpectedHeader,
  IoError(io::Error),
  UnexpectedEof,
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::ExpectedHeader => write!(f, "Invalid patch file with unknown header"),
      Error::IoError(io_err) => write!(f, "{io_err}"),
      Error::UnexpectedEof => write!(f, "Reached end of patch file while decoding"),
    }
  }
}

impl fmt::Debug for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{self}")
  }
}

impl error::Error for Error {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    match self {
      Error::IoError(io_err) => Some(io_err),
      _ => None,
    }
  }
}

impl From<io::Error> for Error {
  fn from(e: io::Error) -> Self {
    Error::IoError(e)
  }
}

struct Record {
  offset: u64,
  run_length: Option<u16>,
  size: usize,
}

impl Record {
  fn try_from_bytes<I>(bytes: &mut I) -> Result<Option<Self>, Error>
  where
    I: Iterator<Item = Result<u8, io::Error>>
  {
    let offset_bytes = [
      bytes.next().ok_or(Error::UnexpectedEof)??,
      bytes.next().ok_or(Error::UnexpectedEof)??,
      bytes.next().ok_or(Error::UnexpectedEof)??,
    ];

    if offset_bytes.as_slice() == IPS_FILE_EOF {
      return Ok(None);
    }

    let offset = u32::from_be_bytes([
      0,
      offset_bytes[0],
      offset_bytes[1],
      offset_bytes[2],
    ]);

    let size = u16::from_be_bytes([
      bytes.next().ok_or(Error::UnexpectedEof)??,
      bytes.next().ok_or(Error::UnexpectedEof)??,      
    ]);

    let mut run_length = None;

    if size == 0 {
      run_length.replace(u16::from_be_bytes([
        bytes.next().ok_or(Error::UnexpectedEof)??,
        bytes.next().ok_or(Error::UnexpectedEof)??,
      ]));
    }

    Ok(Some(Self {
      offset: offset as u64,
      run_length,
      size: size as usize,
    }))
  }
}
