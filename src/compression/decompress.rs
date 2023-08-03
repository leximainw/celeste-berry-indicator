use std::io::{
    Error,
    ErrorKind,
    Read,
    Result,
};

use super::BitReader;

pub struct Decompress<R: Read> {
    base: BitReader<R>,
    curr_block: DecompressState,
}

enum DecompressState {
    None,
    LiteralBlock(Vec<u8>, usize),
}

impl<R: Read> Decompress<R> {
    pub fn new(base: R) -> Decompress<R> {
        Decompress{
            base: BitReader::new(base),
            curr_block: DecompressState::None,
        }
    }
}

impl<R: Read> Read for Decompress<R> {
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
        let mut ptr: usize = 0;
        macro_rules! read_bit {
            ($terminal:literal, $if_true:stmt, $if_false:stmt) => {
                if let Some(value) = self.base.read_bool()? {
                    if value {
                        $if_true
                    } else {
                        $if_false
                    }
                } else if $terminal {
                    return Ok(ptr);
                } else {
                    return Err(Error::new(ErrorKind::UnexpectedEof, "unexpected end of file"));
                }
            }
        }

        macro_rules! read_u8 {
            ($terminal:literal, $var:ident, $block:stmt) => {
                if let Some($var) = self.base.read_u8()? {
                    $block
                } else if $terminal {
                    return Ok(ptr);
                } else {
                    return Err(Error::new(ErrorKind::UnexpectedEof, "unexpected end of file"));
                }
            }
        }

        macro_rules! read_u16 {
            ($terminal:literal, $var:ident, $block:stmt) => {
                if let Some($var) = self.base.read_u16()? {
                    $block
                } else if $terminal {
                    return Ok(ptr);
                } else {
                    return Err(Error::new(ErrorKind::UnexpectedEof, "unexpected end of file"));
                }
            }
        }

        while ptr < buffer.len() {
            match self.curr_block {
                DecompressState::None => {
                    read_bit!(true, {   // literal block
                        read_bit!(false, {   // multiple bytes
                            let mut vec_size = 0;
                            read_bit!(false, {   // variable length
                                read_bit!(false, {   // u16 + 259
                                    read_u16!(false, value, {
                                        vec_size = Into::<usize>::into(value) + 259;
                                    });
                                }, {   // u8 + 2
                                    read_u8!(false, value, {
                                        vec_size = Into::<usize>::into(value) + 2;
                                    });
                                });
                            }, {   // fixed length
                                read_bit!(false, {   // 65795-byte block
                                    vec_size = 65795;
                                }, {   // 258-byte block
                                    vec_size = 258;
                                });
                            });
                            let mut vec = vec![0; vec_size];
                            for i in 0..vec_size {
                                read_u8!(false, value, {
                                    vec[i] = value;
                                    ptr += 1;
                                })
                            }
                            let mut vec_ptr = 0;
                            while ptr < buffer.len() && vec_ptr < vec.len() {
                                buffer[ptr] = vec[vec_ptr];
                                vec_ptr += 1;
                                ptr += 1;
                            }
                            if vec_ptr < vec.len() {
                                self.curr_block = DecompressState::LiteralBlock(vec, vec_ptr);
                            }
                        }, {   // single byte
                            read_u8!(false, value, {
                                buffer[ptr] = value;
                                ptr += 1;
                            });
                        });
                    }, {   // compressed block
                        todo!();
                    });
                },
                _ => todo!(),
            }
        }
        Ok(ptr)
    }
}
