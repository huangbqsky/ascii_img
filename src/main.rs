mod arguments;
mod operations;
mod utils;

use arguments::config::{Config, Style};
use operations::{ascii::img_to_ascii, braille::img_to_braille, onechar::img_to_onechar};
use std::env;

/*
                                 _
                              _ooOoo_
                             o8888888o
                             88" . "88
                             (| -_- |)
                             O\  =  /O
                          ____/`---'\____
                        .'  \\|     |//  `.
                       /  \\|||  :  |||//  \
                      /  _||||| -:- |||||_  \
                      |   | \\\  -  /'| |   |
                      | \_|  `\`---'//  |_/ |
                      \  .-\__ `-. -'__/-.  /
                    ___`. .'  /--.--\  `. .'___
                 ."" '<  `.___\_<|>_/___.' _> \"".
                | | :  `- \`. ;`. _/; .'/ /  .' ; |    
                \  \ `-.   \_\_`. _.'_/_/  -' _.' /
  ================-.`___`-.__\ \___  /__.-'_.'_.-'================
                              `=--=-'                  

                   佛祖保佑    永无BUG    永不宕机
*/

fn main() {
    let mut args = env::args();

    // parse args and return a valid config with defaults
    let config = match Config::new(&mut args) {
        Some(val) => val,
        None => return,
    };

    // matching the style givin to decide which operation to apply.
    match config.style {
        Style::OneChar => {
            img_to_onechar(config);
        }
        Style::Braille => {
            img_to_braille(config);
        }
        Style::Ascii => {
            let table = if config.table.is_empty() {
                vec![
                    ' ', ' ', ' ', ' ', '.', '.', '.', ',', ',', ',', '\'', ';', ':', '<', '>',
                    'l', 'o', 'b', 'd', 'x', 'k', 'O', '0', 'K', 'X', 'N', 'W', 'M',
                ]
            } else {
                config.table.clone()
            };
            img_to_ascii(config, &table);
        }
        Style::Numbers => {
            let table = vec![
                ' ', ' ', ' ', ' ', '0', '1', '7', '6', '9', '4', '2', '3', '8',
            ];
            img_to_ascii(config, &table);
        }
        Style::Blocks => {
            let table = vec![' ', ' ', ' ', ' ', '░', '▒', '▓', '█'];
            img_to_ascii(config, &table);
        }
    };
}