#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_extern_crates)]
#![feature(test)]

extern crate bytebuffer;
extern crate test;

#[macro_use]
mod macros;
use macros::*;

use std::collections::HashMap;
use std::sync::mpsc::{Sender};
use bytebuffer::{ByteBuffer};
#[cfg(test)]
mod tests {
    use test::Bencher;
    use super::*;

    #[bench]
    fn bench_colorbg(b: &mut Bencher) {
        b.iter(|| colorBg!(255, 255, 255));
    }
    #[bench]
    fn bench_colorfg(b: &mut Bencher) {
        b.iter(|| colorFg!(30));
    }
    #[bench]
    fn bench_clear(b: &mut Bencher) {
        b.iter(|| colorFg!());
    }
    #[bench]
    fn print_pic(b: &mut Bencher) {
        b.iter( || {
        let testb: String = String::from("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");

        imgcat(testb.as_bytes(), 12, 12);
        });
    }

    #[test]
    fn text_box() {
        let mut attr = TermAttr {cols: 76, lines: 82, cursor: [0, 0]};
        let mut tbox = TextBox::new(1, 1, 12, 12);
        for c in String::from("hello!World!hello!World!").into_bytes() {
            tbox.listen(c);
        }
        for c in &[8 as u8; 15] {
            tbox.listen(*c);
        }
        use std::io::{self, Write};
        use std::ops::Deref;
        loop {
            let out = tbox.serialize(&mut attr);
            if out.is_empty() {
                break;
            }
            io::stdout().write(Box::deref(&out)).unwrap();
        }
        io::stdout().write(&tbox.buf).unwrap();
    }

    #[test]
    fn it_works() {
        println!("{}{}xddd{}xd", String::from_utf8_lossy(&colorBg!(100, 100, 0)),
                                 String::from_utf8_lossy(&colorFg!(
                                         255, 255, 0)),
                                 String::from_utf8_lossy(&normal!()));
    }
}


pub struct TermAttr {
    cols: u16,
    lines: u16,
    cursor: [u16; 2]
}

impl TermAttr {
    pub fn new() -> TermAttr {
        TermAttr {..Default::default()}
    }
}

impl Default for TermAttr {
    fn default() -> TermAttr {
        TermAttr {
            cols: 80,
            lines: 25,
            cursor: [0, 0],
        }
    }
}

trait Element {
    fn serialize(&mut self, attr: &mut TermAttr) -> Box<[u8]>;
    fn position(&mut self) -> [u16; 2];
}

pub struct TextBox {
    pub buf: Vec<u8>,
    pub x: u16,
    pub y: u16,
    pub width: usize,
    pub height: usize,
    pub cursorX: u16,
    pub cursorY: u16,
    //color: TermColor,
    //fontColor: TermColor,
    pub serialized: bool,
    outbuf: ByteBuffer,
}


impl TextBox {
    pub fn new(x: u16, y: u16, width: usize, height: usize) -> TextBox {
        return TextBox{
            buf: Vec::new(),
            x: x,
            y: y,
            width: width,
            height: height,
            cursorX: x,
            cursorY: y,
            serialized: false,
            outbuf: ByteBuffer::new(),
        };
    }
    pub fn listen(&mut self, c: u8) {
        // TODO: Handle color
        if self.cursorY + 1 - self.y <= self.height as u16 {
            match c {
                BACKSPACE => {
                    if self.cursorX == self.x {
                        if self.cursorY > self.y {
                            self.cursorX = self.x + (self.width as u16) - 1;
                            self.cursorY -= 1;
                            self.outbuf.write_bytes(&cursorUp!());
                            self.outbuf.write_bytes(&right!(
                                    (self.width as u16)));
                        } else {
                            return;
                        }
                    } else {
                        self.cursorX -= 1;
                    }
                    self.buf.pop();
                    self.outbuf.write_u8(c);
                    self.outbuf.write_u8(32);
                    self.outbuf.write_bytes(&cursorLeft!());
                },
                _ => {
                    self.buf.push(c);
                    if self.cursorX + 1 - self.x > self.width as u16 {
                        self.cursorX -= self.width as u16;
                        self.cursorY += 1;
                        self.outbuf.write_bytes(&cursorDown!());
                        self.outbuf.write_bytes(&left!((self.width as u16)));
                    }
                    self.cursorX += 1;
                    self.outbuf.write_u8(c);
                }
            }
        }
    }
}

impl Iterator for TextBox {
    type Item = Box<[u8]>;
    fn next(&mut self) -> Option<Box<[u8]>> {
        if !self.serialized {
            self.serialized = true;
            return Some(move2!((self.x+1), (self.y+1)));
        }
        let mut output = Vec::with_capacity(self.outbuf.len());
        loop {
            if self.outbuf.get_rpos() >= self.outbuf.len() {
                break;
            }
            output.push(self.outbuf.read_u8());
        }
        return Some(output.into_boxed_slice());
    }
}

impl Element for TextBox {
    fn serialize(&mut self, _attr: &mut TermAttr) -> Box<[u8]> {
        return self.next().unwrap();
    }
    fn position(&mut self) -> [u16; 2] {
        return [self.x, self.y];
    }
}

pub struct TermBox {
    attr: TermAttr,
    events: HashMap<u8, Box<FnMut(&mut TermAttr)>>
}

impl TermBox {
    pub fn new() -> TermBox {
        TermBox {events: HashMap::new(), attr: TermAttr::new()}
    }
    pub fn addEvent(&mut self, key: u8, value: Box<FnMut(&mut TermAttr)>) {
        self.events.insert(key, value);
    }
    pub fn call(&mut self, key: &u8) {
        self.events.get_mut(key).unwrap()(&mut self.attr);
    }
}

