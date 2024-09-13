use image::imageops::FilterType;
use image::{DynamicImage, ImageReader};
use std::io::Cursor;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

pub struct Size {
    width: u32,
    height: u32,
}
impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Size { width, height }
    }
}
#[async_trait::async_trait]
pub trait IImage {
    async fn save_to_file(&self, file_path: &str, img: DynamicImage) -> anyhow::Result<()>;

    fn resize(&self, data: &[u8], size: Size) -> anyhow::Result<DynamicImage>;
}

fn resize_image(data: &[u8], size: Size) -> anyhow::Result<DynamicImage> {
    let img = ImageReader::new(Cursor::new(data))
        .with_guessed_format()?
        .decode()?;
    let w = img.width();
    let h = img.height();
    if w <= size.width && h <= size.height {
        return Ok(img);
    }
    Ok(img.resize(size.width, size.height, FilterType::CatmullRom))
}

#[derive(Debug)]
pub struct FakeImageUtil {
    save_file_error: bool,
    files: Mutex<Vec<String>>,
}

#[cfg(test)]
impl FakeImageUtil {
    pub fn new() -> Self {
        Self {
            save_file_error: false,
            files: Mutex::new(Vec::new()),
        }
    }

    pub fn with_save_file_error(self) -> Self {
        Self {
            save_file_error: true,
            ..self
        }
    }
}

#[async_trait::async_trait]
impl IImage for FakeImageUtil {
    async fn save_to_file(&self, file_path: &str, _: DynamicImage) -> anyhow::Result<()> {
        if self.save_file_error {
            return Err(anyhow::anyhow!("Failed to save file"));
        }
        let mut lock = self.files.lock().await;
        lock.push(file_path.to_string());
        Ok(())
    }

    fn resize(&self, data: &[u8], size: Size) -> anyhow::Result<DynamicImage> {
        resize_image(data, size)
    }
}

#[derive(Debug, Default)]
pub struct ImageUtil {}

#[async_trait::async_trait]
impl IImage for ImageUtil {
    async fn save_to_file(&self, file_path: &str, img: DynamicImage) -> anyhow::Result<()> {
        let mut output = File::create(file_path).await?;
        let mut buffer = Vec::new();
        img.write_to(&mut Cursor::new(&mut buffer), image::ImageFormat::Png)?;
        Ok(output.write_all(&buffer).await?)
    }

    fn resize(&self, data: &[u8], size: Size) -> anyhow::Result<DynamicImage> {
        resize_image(data, size)
    }
}
