use crate::arguments::argument_parsing;

// 显示类型
#[derive(Debug)]
pub enum Style { 
    Ascii, // Ascii 文字
    Blocks,  // 块
    Braille, // 盲文
    Numbers, // 纯数字
    OneChar, // 一个字符
}

#[derive(Debug)]
pub struct Config {
    pub background: u8,
    pub colored: bool,
    pub dither: bool,
    pub dither_scale: u8,
    pub image_file: String,
    pub onechar: char,
    pub original_size: bool,
    pub scale: u32,
    pub sleep: u64,
    pub style: Style,
    pub table: Vec<char>,
    pub once: bool,
}

impl Default for Style {
    fn default() -> Self {
        Self::Braille
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            background: 38,
            colored: false,
            dither: false,
            dither_scale: 16,
            image_file: String::new(),
            onechar: '█',
            original_size: false,
            scale: 2,
            sleep: 100,
            style: Style::default(),
            table: vec![],
            once: false,
        }
    }
}

impl Config {
    // Parsing arguments and return a valid config
    pub fn new(args: &mut std::env::Args) -> Option<Self> {
        // converting from iterator to vector.
        let args: Vec<String> = args.collect();
        argument_parsing::parse(args)
    }
}