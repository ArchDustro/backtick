use std::path::PathBuf;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, BufWriter, Result};

pub struct FileLogger {
    writer: BufWriter<tokio::fs::File>,
}

impl FileLogger {
    pub async fn open(path: PathBuf) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true) // change to .truncate(true) for overwrite mode
            .open(path)
            .await?;

        Ok(Self {
            writer: BufWriter::new(file),
        })
    }

    pub async fn write(&mut self, contents: &str) -> Result<()> {
        self.writer.write_all(contents.as_bytes()).await?;
        self.writer.write_all(b"\n").await?;
        Ok(())
    }

    pub async fn close(mut self) -> Result<()> {
        self.writer.flush().await
    }
}
