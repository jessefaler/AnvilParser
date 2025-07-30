mod region;

use std::borrow::Cow;
use std::io::{Cursor};
use region::Region;

use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = r"C:\r.0.0.mca";
    let region = Region::create_region(path)?;

    let start = Instant::now();  // Start total timer

    // Region files contain 32x32 chunks
    for x in 0..32 {
        for z in 0..32 {
            match region.get_chunk(x, z)? {
                Some(chunk) => {
                    extract_block_palettes(&chunk);
                }
                None => {
                }
            }
        }
    }

    let elapsed = start.elapsed();  // End total timer

    println!(
        "Total time to process all chunks: {}.{:03} seconds",
        elapsed.as_secs(),
        elapsed.subsec_millis()
    );

    Ok(())
}

fn example(chunk_bytes: &[u8]) {
    let nbt = simdnbt::borrow::read(&mut Cursor::new(chunk_bytes))
        .unwrap()
        .unwrap();

    // Extract DataVersion from the root compound
    let data_version = nbt
        .int("DataVersion")
        .unwrap_or(-1); // fallback if not found

    println!("DataVersion: {}", data_version);
}

fn extract_block_palettes(chunk_bytes: &[u8]) {
    let root = simdnbt::borrow::read(&mut Cursor::new(chunk_bytes)).unwrap().unwrap();

    if let Some(sections) = root.list("sections") {
        for section in sections.compounds().unwrap_or_default() {
            let y = section.byte("Y").unwrap_or(-99);

            if let Some(block_states) = section.compound("block_states") {
                if let Some(palette_list) = block_states.list("palette") {
                    println!("Section Y = {:?}", y);
                    for block in palette_list.compounds().unwrap_or_default() {
                        let name: Cow<str> = block.string("Name")
                            .map(|m| m.to_str())
                            .unwrap_or_else(|| Cow::Borrowed("<unknown>"));
                        println!("  Block: {}", name);

                        if let Some(props) = block.compound("Properties") {
                            for (key, val) in props.iter() {
                                println!("    Property key: {}, value: {:?}", key, val);
                            }
                        }
                    }
                }
            }
        }
    }
}