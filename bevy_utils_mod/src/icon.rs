use crate::BevyProjectConfig;
use icns_rs::{IcnsEncoder, IconFormats};
use image::imageops::FilterType;
use image::DynamicImage;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, ErrorKind, Write};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CreateIconsError {
    #[error("could not find the template icon `icon_1024x1024.png` in the build directory")]
    TemplateIconMissing(String),
    #[error("io operation failed")]
    UnknownIoError(std::io::Error),
}

struct Icons(HashMap<(u32, u32), DynamicImage>);

pub fn create_icons(config: BevyProjectConfig) -> Result<(), CreateIconsError> {
    let template_image_path = config.build_data_directory.join("icon_1024x1024.png");

    let img = image::open(&template_image_path).unwrap();
    let required_sizes: Vec<(u32, u32)> = vec![
        (1024, 1024),
        (512, 512),
        (256, 256),
        (128, 128),
        (64, 64),
        (32, 32),
        (16, 16),
    ];
    let mut icons = Icons(Default::default());
    for (width, height) in required_sizes {
        icons.0.insert(
            (width, height),
            img.resize(width, height, FilterType::Nearest),
        );
    }

    // icns for macOS
    let mut encoder = IcnsEncoder::new();

    encoder.data(img);
    encoder.formats(IconFormats::recommended());

    // Encode the image
    let data = match encoder.build() {
        Ok(data) => data,
        Err(e) => {
            println!("Error encoding image: {}", e);
            return Ok(());
        }
    };

    let mut file = BufWriter::new(
        File::create(
            &config
                .build_data_directory
                .join("macos/src/Game.app/Contents/Resources/AppIcon.icns"),
        )
        .unwrap(),
    );
    file.write_all(&data)?;

    Ok(())
}

impl From<std::io::Error> for CreateIconsError {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            ErrorKind::NotFound => CreateIconsError::TemplateIconMissing("bla".to_string()),
            _ => CreateIconsError::UnknownIoError(value),
        }
    }
}
