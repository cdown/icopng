use std::convert::TryFrom;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{Read, Write};

use byteorder::{LittleEndian, WriteBytesExt};

use anyhow::{bail, Result};

// All of these are sized based on valid ico output, not png input
#[derive(Debug)]
struct PngMetadata {
    depth: u16,
    length: u32,
    width: u8,
    height: u8,
    data: Vec<u8>,
}

fn get_input_metadata(filename: &str) -> Result<PngMetadata> {
    let mut input = File::open(filename)?;
    let size = input.metadata()?.len();
    let mut png_data = vec![0; size as usize];
    input.read_exact(&mut png_data)?;
    input.rewind()?;

    let decoder = png::Decoder::new(&input);
    let mut reader = decoder.read_info()?;
    let mut decomp_data = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut decomp_data)?;

    let bit_depth = u16::try_from(info.bit_depth as usize * info.color_type.samples())?;

    Ok(PngMetadata {
        depth: bit_depth,
        // nb: not the output buffer size, which is post-decode
        length: u32::try_from(size)?,
        width: u8::try_from(get_ico_dimension(info.width)?)?,
        height: u8::try_from(get_ico_dimension(info.height)?)?,
        data: png_data,
    })
}

/// ico files encode dimensions with 0 as 256
fn get_ico_dimension(dim: u32) -> Result<u32> {
    if dim == 256 {
        Ok(0)
    } else if dim == 0 || dim > 256 {
        bail!(
            "Input image has dimension {}, but ico files only support from 1-256",
            dim
        );
    } else {
        Ok(dim)
    }
}

// https://en.wikipedia.org/w/index.php?title=ICO_(file_format)&oldid=1048679157#Outline
fn write_ico(filename: &str, meta: &PngMetadata) -> Result<()> {
    let mut data: Vec<u8> = vec![];
    data.write_u16::<LittleEndian>(0)?; // Reserved
    data.write_u16::<LittleEndian>(1)?; // Type ICO
    data.write_u16::<LittleEndian>(1)?; // 1 image
    data.write_u8(meta.width)?;
    data.write_u8(meta.height)?;
    data.write_u8(0)?; // Colour palette
    data.write_u8(0)?; // Reserved
    data.write_u16::<LittleEndian>(1)?; // Colour planes
    data.write_u16::<LittleEndian>(meta.depth)?;
    data.write_u32::<LittleEndian>(meta.length)?;
    data.write_u32::<LittleEndian>(data.len() as u32 + 4)?; // PNG offset

    let mut output = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(filename)?;
    output.write_all(&data)?;
    output.write_all(&meta.data)?;

    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        bail!("Usage: {} [input] [output]", args[0]);
    }

    let meta = get_input_metadata(&args[1])?;
    write_ico(&args[2], &meta).map_err(anyhow::Error::from)
}
