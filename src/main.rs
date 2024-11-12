use std::convert::TryFrom;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, Write};

use anyhow::{bail, Result};
use byteorder::{LittleEndian, WriteBytesExt};

const ICO_RESERVED: u8 = 0;
const ICO_TYPE: u16 = 1;
const ICO_COLOUR_PLANES: u16 = 1;
const ICO_COLOUR_PALETTE: u8 = 0;
const ICO_HEADER_SIZE: usize = 6;
const ICO_DIRECTORY_ENTRY_SIZE: usize = 16;
const MAX_ICO_DIMENSION: u32 = 256;

// All of these are sized based on valid ico output, not png input
#[derive(Debug)]
struct PngMetadata {
    depth: u16,
    length: u32,
    width: u8,
    height: u8,
    data: Vec<u8>,
}

fn get_png_metadata(filename: &str) -> Result<PngMetadata> {
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
    if dim == MAX_ICO_DIMENSION {
        Ok(0)
    } else if dim == 0 || dim > MAX_ICO_DIMENSION {
        bail!(
            "Input image has dimension {}, but ico files only support from 1-256",
            dim
        );
    } else {
        Ok(dim)
    }
}

// https://en.wikipedia.org/w/index.php?title=ICO_(file_format)&oldid=1048679157#Outline
fn write_ico(filename: &str, metas: &[PngMetadata]) -> Result<()> {
    let mut ico_hdr: Vec<u8> = vec![];
    ico_hdr.write_u16::<LittleEndian>(ICO_RESERVED.into())?;
    ico_hdr.write_u16::<LittleEndian>(ICO_TYPE)?;
    ico_hdr.write_u16::<LittleEndian>(metas.len() as u16)?;

    let mut image_offset = (ICO_HEADER_SIZE + (ICO_DIRECTORY_ENTRY_SIZE * metas.len())) as u32;

    for meta in metas {
        ico_hdr.write_u8(meta.width)?;
        ico_hdr.write_u8(meta.height)?;
        ico_hdr.write_u8(ICO_COLOUR_PALETTE)?;
        ico_hdr.write_u8(ICO_RESERVED)?;
        ico_hdr.write_u16::<LittleEndian>(ICO_COLOUR_PLANES)?;
        ico_hdr.write_u16::<LittleEndian>(meta.depth)?;
        ico_hdr.write_u32::<LittleEndian>(meta.length)?;
        ico_hdr.write_u32::<LittleEndian>(image_offset)?;
        image_offset += meta.length;
    }

    let mut output = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(filename)?;
    output.write_all(&ico_hdr)?;

    for meta in metas {
        output.write_all(&meta.data)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        bail!(
            "Usage: {} [input1.png] [input2.png] ... [output.ico]",
            args[0]
        );
    }

    let input_filenames = &args[1..args.len() - 1];
    let output_filename = &args[args.len() - 1];

    let metas = input_filenames
        .iter()
        .map(|file| get_png_metadata(file))
        .collect::<Result<Vec<_>>>()?;

    write_ico(output_filename, &metas).map_err(anyhow::Error::from)
}
