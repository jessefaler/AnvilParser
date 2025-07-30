use std::error::Error;
use std::fs;
use mca::RegionReader;
use ouroboros::self_referencing;

#[self_referencing]
pub struct Region {
    data: Vec<u8>,

    #[borrows(data)]
    #[covariant]
    region: RegionReader<'this>,
}
impl Region {
    pub fn create_region(path: &str) -> Result<Region, Box<dyn Error>> {
        let data = fs::read(path)?;
        RegionTryBuilder {
            data,
            region_builder: |data| RegionReader::new(data),
        }
            .try_build()
            .map_err(|e| e.into())
    }
    pub fn get_chunk(&self, x: u32, z: u32) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
        self.with_region(|region| {
            let x_usize = x.try_into()?;
            let z_usize = z.try_into()?;
            if let Some(compressed_chunk) = region.get_chunk(x_usize, z_usize)? {
                let decompressed = compressed_chunk.decompress()?;
                Ok(Some(decompressed))
            } else {
                Ok(None)
            }
        })
    }
}
