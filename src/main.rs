use std::env;
use std::error;
use std::fmt;
use std::fs::File;
use std::io;

mod ips;

fn main() -> Result<(), Error> {
  let args = Args::try_from(env::args())?;

  let mut ips_file = File::open(&args.patch)?;
  let mut rom_file = File::open(&args.rom)?;
  let mut out_file = File::create(&args.target)?;

  match ips::apply_patch(&mut ips_file, &mut rom_file, &mut out_file) {
    Ok(()) => {
      println!("==> Patch written to {}", args.target);
      Ok(())
    },

    Err(e) => Err(e.into()),
  }
}

#[allow(clippy::enum_variant_names)]
enum Error {
  ExpectedArg,
  IoError(io::Error),
  IpsError(ips::Error),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::ExpectedArg => {
        writeln!(f, "Missing required argument(s)")?;
        Args::help(f)
      },
      Error::IpsError(ips_err) => write!(f, "{ips_err}"),
      Error::IoError(io_err) => write!(f, "{io_err}"),
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
      Error::IpsError(ips_err) => Some(ips_err),
      Error::IoError(io_err) => Some(io_err),
      _ => None,
    }
  }
}

impl From<ips::Error> for Error {
  fn from(e: ips::Error) -> Self {
    Error::IpsError(e)
  }
}

impl From<io::Error> for Error {
  fn from(e: io::Error) -> Self {
    Error::IoError(e)
  }
}

struct Args {
  patch: String,
  rom: String,
  target: String,
}

impl Args {
  fn help(f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "
Utility for patching ROM files with IPS patch files

Usage:
  ./<executable> --patch my-patch-file.ips --rom my-rom-file.rom --target my-patched-rom.rom

Arguments:
  -p, --patch    Path to an IPS encoded patch file
  -r, --rom      Path to a ROM file to patch
  -t, --target   Path to write the patched ROM file to
    ")
  }
}

impl TryFrom<env::Args> for Args {
  type Error = Error;

  fn try_from(env_args: env::Args) -> Result<Self, Self::Error> {
    let mut patch = None;
    let mut rom = None;
    let mut target = None;
    
    let mut raw_args = env_args.skip(1).take(6);

    while let Some(raw_arg) = raw_args.next() {
      let mut split: Vec<_> = raw_arg
        .split('=')
        .map(str::to_string)
        .collect();

      if split.len() == 1 {
        split.push(raw_args.next().ok_or(Error::ExpectedArg)?);
      }

      let split = split.split_at(1);
      let arg_name = split.0[0].as_str();
      let arg_value = split.1[0].as_str();

      match (arg_name, arg_value) {
        ("-p" | "--patch", value) => patch.replace(value.to_owned()),
        ("-r" | "--rom", value) => rom.replace(value.to_owned()),
        ("-t" | "--target", value) => target.replace(value.to_owned()),
        _ => continue,
      };
    }

    Ok(Args {
      patch: patch.ok_or(Error::ExpectedArg)?,
      rom: rom.ok_or(Error::ExpectedArg)?,
      target: target.ok_or(Error::ExpectedArg)?,
    })
  }
}
