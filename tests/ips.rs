use std::io::{Cursor};

use ips_rs::{Error, apply_patch};

#[test]
fn test_blank_patch_file() {
  let rom = Vec::new();
  let patch = Vec::new(); // Blank patch file.
  let mut out = Cursor::new(Vec::new());

  let result = apply_patch(
    &mut rom.as_slice(),
    &mut patch.as_slice(),
    &mut out,
  );

  assert!(matches!(result, Err(Error::ExpectedHeader)));
}

#[test]
fn test_incomplete_patch_file() {
  let rom = Vec::new();
  let mut out = Cursor::new(Vec::new());

  let patch: Vec<u8> = vec![
    // Header
    b'P', b'A', b'T', b'C', b'H',

    0, 0, 0, // Record offset
    // Missing record size and data

    // EOF
    b'E', b'O', b'F',
  ];

  let result = apply_patch(
    &mut patch.as_slice(),
    &mut rom.as_slice(),
    &mut out,
  );

  assert!(matches!(result, Err(Error::UnexpectedEof)));
}

#[test]
fn test_simple_patch() {
  let rom = vec![0, 0, 0, 0];
  let mut out = Cursor::new(Vec::new());

  let patch = vec![
    // Header
    b'P', b'A', b'T', b'C', b'H',

    0, 0, 0, // Record offset
    0, 1, // Record size
    1, // Record data

    // EOF
    b'E', b'O', b'F',
  ];

  let result = apply_patch(
    &mut patch.as_slice(),
    &mut rom.as_slice(),
    &mut out,
  );

  match result {
    Ok(()) => {
      assert_eq!(out.into_inner(), vec![1, 0, 0, 0]);
    },

    Err(e) => {
      panic!("{e}");
    },
  }
}

#[test]
fn test_patch_with_run_length_encoded_record() {
  let rom = vec![0, 0, 0, 0];
  let mut out = Cursor::new(Vec::new());

  let patch = vec![
    // Header
    b'P', b'A', b'T', b'C', b'H',

    0, 0, 1, // Record offset
    0, 0, // Record size, 0 = RLE
    0, 2, // RLE size
    1, // Data to repeat

    // EOF
    b'E', b'O', b'F',
  ];

  let result = apply_patch(
    &mut patch.as_slice(),
    &mut rom.as_slice(),
    &mut out,
  );

  match result {
    Ok(()) => {
      assert_eq!(out.into_inner(), vec![0, 1, 1, 0]);
    },

    Err(e) => {
      panic!("{e}");
    },
  }
}

#[test]
fn test_patch_with_multiple_records() {
  let rom = vec![0, 0, 0, 0];
  let mut out = Cursor::new(Vec::new());

  let patch = vec![
    // Header
    b'P', b'A', b'T', b'C', b'H',

    // Record 1
    0, 0, 0,
    0, 1,
    1,

    // Record 2 - RLE
    0, 0, 1,
    0, 0,
    0, 2,
    1,

    // Record 3
    0, 0, 3,
    0, 1,
    1,

    // EOF
    b'E', b'O', b'F',
  ];

  let result = apply_patch(
    &mut patch.as_slice(),
    &mut rom.as_slice(),
    &mut out,
  );

  match result {
    Ok(()) => {
      assert_eq!(out.into_inner(), vec![1, 1, 1, 1]);
    },

    Err(e) => {
      panic!("{e}");
    },
  }
}
