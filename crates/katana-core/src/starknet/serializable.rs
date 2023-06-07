use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

use super::StarknetWrapper;

pub trait SerializableState {
    fn dump_state(&self, path: &PathBuf) -> std::io::Result<()>;

    fn load_state(&mut self, path: &PathBuf) -> std::io::Result<()>;
}

impl SerializableState for StarknetWrapper {
    fn dump_state(&self, path: &PathBuf) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, self)?;

        writer.flush()?;
        Ok(())
    }

    fn load_state(&mut self, path: &PathBuf) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // decode buffer content
        let decoded: StarknetWrapper = serde_json::from_reader(reader)?;
        *self = decoded;
        Ok(())
    }
}
