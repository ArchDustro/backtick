use std::path::PathBuf;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, BufWriter, Result};

pub struct FileLogger {
    writer: BufWriter<tokio::fs::File>,
}

impl FileLogger {
    pub fn open(path: PathBuf) -> std::io::Result<Self> {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;

        Ok(Self {
            writer: BufWriter::new(tokio::fs::File::from_std(file)),
        })
    }


    pub async fn write(&mut self, contents: &str) -> Result<()> {
        self.writer.write_all(contents.as_bytes()).await?;
        self.writer.write_all(b"\n").await?;
        self.writer.flush().await?;
        Ok(())
    }

    pub async fn close(mut self) -> Result<()> {
        self.writer.flush().await
    }
}
