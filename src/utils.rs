use crate::arguments::config::Config;
use image::{DynamicImage, GenericImageView, GrayImage, RgbaImage};

/**
图片转ASCII艺术: https://github.com/lecepin/blog/blob/main/Rust%20Wasm%20%E5%9B%BE%E7%89%87%E8%BD%AC%20ASCII%20%E8%89%BA%E6%9C%AF.md
实现原理:
1.RGB 图片转成灰度图片。
2.准备一些不同密度的 ASCII 字符。
3.遍历灰度图片像素，根据亮度值替换相应的 ASCII 字符。

灰度处理:
1.灰度和彩色图片的区别就是 R=G=B。
2.关于灰度值的计算，有3种主流方式：
  (1)最大值法：Max(R, G, B)。
  (2)平均值法：(R + G + B) / 3。
  (3)加权平均值法：
    0.2126 * R + 0.7152 * G + 0.0722 * B
    0.299 * R + 0.587 * G + 0.114 * B
    Math.sqrt( (0.299 * R) ** 2 + (0.587 * G) ** 2 + (0.114 * B) ** 2 )
*/

// luminance formula credits: https://stackoverflow.com/a/596243
// >>> Luminance = 0.2126*R + 0.7152*G + 0.0722*B <<<
// 加权平均值法计算灰度：calculate RGB values to get luminance of the pixel
pub fn get_luminance(r: u8, g: u8, b: u8) -> f32 {
    let r = 0.2126 * (r as f32);
    let g = 0.7152 * (g as f32);
    let b = 0.0722 * (b as f32);
    r + g + b
}

// 根据亮度值替换相应的 ASCII 字符 colorize a character by surrounding it with true  term colors
pub fn colorize(rgb: &[u8; 3], ch: char, bg_fg: u8) -> String {
    let prefix = format!("\x1B[{};2;{};{};{}m", bg_fg, rgb[0], rgb[1], rgb[2]);
    let postfix = "\x1B[0m";
    format!("{}{}{}", prefix, ch, postfix)
}

// 打开并且缩放换为图像 rescale the image and convert to image buffer
pub fn open_and_resize(config: &Config) -> Option<RgbaImage> {
    let img = if let Ok(image) = image::open(&config.image_file) {
        image
    } else {
        eprintln!("Image path is not correct, OR image format is not supported!\n try -h | --help");
        return None;
    };
    let width = match config.original_size {
        true => img.width(),
        false => ((img.width() / config.scale) / 2) as u32,
    };
    let height = match config.original_size {
        true => img.height(),
        false => ((img.height() / config.scale) / 4) as u32,
    };
    let img = img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);
    let img = if config.colored {
        img.into_rgba8()
    } else {
        img.grayscale().into_rgba8()
    };
    Some(img)
}

pub fn resize(img: DynamicImage, config: &Config) -> RgbaImage {
    let (width, height) = match config.original_size {
        false => {
            let width = ((img.width() / config.scale) / 2) as u32;
            let height = ((img.height() / config.scale) / 4) as u32;
            (width, height)
        }
        true => (img.width(), img.height()),
    };
    img.resize(width, height, image::imageops::FilterType::Lanczos3)
        .to_rgba8()
}

// this will open the image path,
// and resize the image and turn it into image buffer;
// 图片转化为单个字符艺术图： 通过大津算法，将一个灰度图像退化为二值图像
pub fn get_luma_buffer(config: &Config) -> Option<GrayImage> {
    let img = if let Ok(image) = image::open(&config.image_file) {
        image
    } else {
        eprintln!("Image path is not correct, OR image format is not supported!\n try -h | --help");
        return None;
    };
    let width = match config.original_size {
        true => img.width(),
        false => ((img.width() / config.scale) / 2) as u32,
    };
    let height = match config.original_size {
        true => img.height(),
        false => ((img.height() / config.scale) / 4) as u32,
    };
    let img = img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);
    let img = img.to_luma8();
    Some(img)
}