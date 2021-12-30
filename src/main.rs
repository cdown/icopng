use std::env;
use std::fs::File;
use std::process::exit;

use anyhow::Result;

// All of these are sized based on valid ico output, not png input
#[derive(Debug)]
struct PngMetadata {
    depth: u16,
    length: u64,
    width: u8,
    height: u8,
}

fn get_input_metadata(filename: &str) -> Result<PngMetadata> {
    let input = File::open(filename)?;
    let decoder = png::Decoder::new(&input);
    let mut reader = decoder.read_info()?;
    let mut img_data = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut img_data)?;

    Ok(PngMetadata {
        depth: info.bit_depth as u16,
        // nb: not the output buffer size, which is usually page size aligned
        length: input.metadata()?.len() as u64,
        width: info.width as u8,
        height: info.height as u8,
    })
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} [input] [output]", args[0]);
        exit(1);
    }

    let input = &args[1];
    let output = &args[2];

    println!("{:?}", get_input_metadata(input)?);

    Ok(())
}
