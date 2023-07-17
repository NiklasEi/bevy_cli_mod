use crate::{BevyProjectConfig, Platform};
use icns_rs::{IcnsEncoder, IconFormats};
use image::imageops::FilterType;
use image::{DynamicImage, ImageError};
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, ErrorKind, Write};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CreateIconsError {
    #[error("could not find the template icon `icon_1024x1024.png` in the build directory")]
    TemplateIconMissing,
    #[error("could not open the template icon `icon_1024x1024.png` in the build directory: {0}")]
    FailedOpeningTemplateIcon(#[from] ImageError),
    #[error("io operation failed: {0}")]
    IoError(std::io::Error),
    #[error("Failed to build icns file: {0}")]
    IcnsConversionError(String),
}

struct Icons(HashMap<(u32, u32), DynamicImage>);

pub fn create_icons(config: BevyProjectConfig) -> Result<(), CreateIconsError> {
    let template_image_path = config.build_data_directory.join("icon_1024x1024.png");
    let img = image::open(&template_image_path).map_err(|error| {
        if let ImageError::IoError(io_error) = &error {
            if io_error.kind() == ErrorKind::NotFound {
                return CreateIconsError::TemplateIconMissing;
            }
        }
        error.into()
    })?;
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

    for platform in &config.platforms {
        platform.create_icons(&config, &img)?;
    }

    Ok(())
}

impl Platform {
    pub fn create_icons(
        &self,
        config: &BevyProjectConfig,
        template_image: &DynamicImage,
    ) -> Result<(), CreateIconsError> {
        match self {
            Platform::Windows => todo!("implement icon creation"),
            Platform::Linux => todo!("implement icon creation"),
            Platform::Mac => create_icons_mac(config, template_image),
            Platform::Web => todo!("implement icon creation"),
            Platform::Android => todo!("implement icon creation"),
            Platform::Ios => todo!("implement icon creation"),
        }
    }
}

fn create_icons_mac(
    config: &BevyProjectConfig,
    template_image: &DynamicImage,
) -> Result<(), CreateIconsError> {
    let mut encoder = IcnsEncoder::new();

    encoder.data(template_image.clone());
    encoder.formats(IconFormats::recommended());

    let data = encoder.build()?;
    let icns_path = config
        .build_data_directory
        .join("macos/src/Game.app/Contents/Resources/AppIcon.icns");

    // Todo: do want to complain about the platform not being initialized here instead?
    // Missing directories would also mean that other build files like the info.plist could be missing.
    create_dir_all(
        &icns_path
            .parent()
            .expect("icns file needs to have a parent directory"),
    )?;
    let mut file = BufWriter::new(File::create(&icns_path)?);
    file.write_all(&data)?;

    Ok(())
}

impl From<std::io::Error> for CreateIconsError {
    fn from(value: std::io::Error) -> Self {
        println!("{value:?}");
        match value.kind() {
            ErrorKind::NotFound => CreateIconsError::TemplateIconMissing,
            _ => CreateIconsError::IoError(value),
        }
    }
}

impl From<String> for CreateIconsError {
    fn from(value: String) -> Self {
        CreateIconsError::IcnsConversionError(value)
    }
}
