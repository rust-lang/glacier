use self::Ref::*;

use std::mem::swap;

struct Int {
    ch: [Ref; 2],
    off: usize,
    bit: u8,
}

enum Ref {
    N(Box<Int>),
    K(String),
}

pub struct Crash {
    root: Option<Ref>,
    length: usize,
}

impl Int {
    fn dir(&self, key: String) -> usize {
        if self.off < key.len() && key.as_bytes()[self.off] & self.bit != 0 {
            1
        } else {
            0
        }
    }
}

impl Crash {
    pub fn new() -> Crash {
        Crash {
            root: None,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn co(&self, key: &str) -> bool {
        match self.root {
            None => false,
            Some(ref r) => {
                let mut p = r;
                loop {
                    match p {
                        &N(ref n) => {
                            p = &n.ch[n.dir(key.to_string())];
                        }
                        &K(ref s) => {
                            return key == s.to_string();
                        }
                    }
                }
            }
        }
    }

    pub fn inn(&mut self, key: String) -> bool {
        match self.root {
            None => {
                self.root = Some(K(key));
                self.length += 1;
                true
            }
            Some(ref mut r) => {
                let mut p = r;
                loop {
                    match p {
                        &mut N(ref n) => {
                            p = &mut n.ch[n.dir(key.to_string())];
                        }
                        &mut K(ref s) => {
                            let mut off: usize = 0;
                            let mut ch: u8 = 0;
                            let mut bit: u8 = 0;
                            let keyb = key.as_bytes();
                            for off in 0..keyb.len() {
                                if ch == 0 && off < s.len() {
                                    ch = s.as_bytes()[off];
                                }
                                let keych = keyb[off];
                                if ch != keych {
                                    bit = ch ^ keych;
                                    // goto FOUND
                                }
                            }
                            if off < s.len() {
                                ch = s.as_bytes()[off];
                                bit = ch;
                                // goto FOUND
                            }
                            break
                        }
                    }
                }
                false
            }
        }
    }

    #[inline]
    fn fo(&mut self, key: String, off: usize, mut bit: u8, ch: u8) {
        bit |= bit >> 1;
        bit |= bit >> 2;
        bit |= bit >> 4;
        bit &= (255 - (bit >> 1));

        match self.root {
            None => (),
            Some(ref mut r) => {
                let mut wp = r;
                loop {
                    match wp {
                        &mut N(ref mut n) => {
                            if n.off > off || n.off == off && n.bit < bit {
                                break
                            }
                            wp = &mut n.ch[n.dir(key.to_string())];
                        }
                        &mut K(ref s) => break
                    }
                }
                let mut n = if ch & bit > 1 {
                    Int{ch: [K(key), *wp], off: off, bit: bit}
                } else {
                    Int{ch: [*wp, K(key)], off: off, bit: bit}
                };
                let mut nn = N(Box::new(n));
                *wp = nn;
                self.length += 1
            }
        }
    }
}

fn main() {
}
