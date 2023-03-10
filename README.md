# ascii_img

Terminal Ascii Images

A tool to convert images to ascii art written in Rust 🦀

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




### 参考

Tai : 命令行图片转 ASCII 艺术 https://github.com/MustafaSalih1993/tai

Rust wasm image to ascii：Rust Wasm 图片转 ASCII 艺术 https://github.com/lecepin/rust-wasm-image-ascii


### Building & Testing

you can clone ascii_img repo and build it locally

```
➜ git clone https://github.com/huangbqsky/ascii_img
➜ cd ascii_img
➜ cargo build --quiet && target/debug/ascii_img -h  
                                   
USAGE: target/debug/ascii_img [Options] IMAGE

Options:
    -h, --help          Show this help message
    -b, --background    Will apply the colors on the "background" of the
                        characters instead of coloring the foreground
    -c, --colored       Will return true colored(RGB) art
    -d, --dither        enables image dithering
    -o, --onechar CHARACTER
                        Followed by a character, This will modify the default
                        character used by (-S onechar)
    -N, --no-scale      will keep the original size of the image, default to
                        false
    -D, --dither-scale NUMBER
                        used with "-d" option, controls the scale number for
                        the dithering, default to 16
    -O, --once          Will play the image's animation only once (no looping)
    -S, --style STYLE   Followed by one of: {{ascii, numbers, blocks, onechar,
                        braille}}, default to "braille"
        --sleep MILLI_SECONDS
                        Followed by number, controls the sleep delay(milli
                        seconds) between animation frames. default to 100
    -s, --scale NUMBER  Followed by a number to Resize the output (lower
                        number means bigger output) default to 2
    -t, --table TABLE_OF_CHARACTERS
                        Make a custom ascii table,(works only with "ascii"
                        Style) seperated by ','
                        ex: ascii_img -S ascii --table " ,.,:,x,@" image.png
    -v, --version       Print tai's Version and exit!
```

### Example

```
➜ cargo build --quiet && target/debug/ascii_img -b '/xxxx/rustdocs.png'
➜ cargo build --quiet && target/debug/ascii_img -c '/xxxx/rustdocs.png'
➜ cargo build --quiet && target/debug/ascii_img -d '/xxxx/rustdocs.png'
➜ cargo build --quiet && target/debug/ascii_img -o '=' -s 5 '/xxxx/rustdocs.png' 

➜ cargo build --quiet && target/debug/ascii_img -D 50 '/xxxx/rustdocs.png' 

➜ cargo build --quiet && target/debug/ascii_img -S ascii '/xxxx/rustdocs.png'
➜ cargo build --quiet && target/debug/ascii_img -S numbers '/xxxx/rustdocs.png'
➜ cargo build --quiet && target/debug/ascii_img -S blocks '/xxxx/rustdocs.png'
➜ cargo build --quiet && target/debug/ascii_img -S onechar '/xxxx/rustdocs.png'
➜ cargo build --quiet && target/debug/ascii_img -S braille '/xxxx/rustdocs.png'

➜ cargo build --quiet && target/debug/ascii_img -S ascii -s 5 '/xxxx/rustdocs.png'
➜ cargo build --quiet && target/debug/ascii_img -S ascii --table " ,.,:,x,@" '/xxxx/rustdocs.png' 

```
