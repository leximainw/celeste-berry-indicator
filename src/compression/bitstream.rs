use std::io::{
    Read,
    Result,
};

pub struct BitReader<R: Read> {
    base: R,
    bits: u8,
    buffer: u64,
}

impl<R: Read> BitReader<R> {
    pub fn new(base: R) -> BitReader<R> {
        BitReader{
            base,
            bits: 0,
            buffer: 0,
        }
    }

    fn fill(&mut self, bytes: usize) -> Result<()> {
        if bytes == 0 {
            return Ok(());
        }
        let mut buffer: [u8; 8] = [0; 8];
        let bytes = usize::min(bytes, self.base.read(&mut buffer[0..bytes])?);
        let mut offset = 64 - (self.bits + 8);
        for i in 0..bytes {
            self.buffer |= (buffer[i] as u64) << offset;
            self.bits += 8;
            offset -= 8;
        }
        Ok(())
    }

    pub fn read_bits(&mut self, bits: usize) -> Result<Option<u64>> {
        if bits == 0 {
            return Ok(None);
        } else if bits > 64 {
            panic!("expect bits <= 64, got bits = {bits}");
        } else if bits > 56 {
            let shift = bits - 56;
            let left = self.read_bits(56)?;
            let right = self.read_bits(shift)?;
            return Ok(left.and_then(|x| right.map(|y| (x, y)).map(|x| x.0 << shift | x.1)));
        }
        let bits = bits as u8;
        if self.bits < bits {
            self.fill(((bits + 7 - self.bits) / 8).into())?;
            if self.bits < bits {
                return Ok(None);
            }
        }
        let shift = 64 - bits;
        let value = self.buffer >> shift;
        self.buffer <<= shift;
        self.bits -= bits;
        Ok(Some(value))
    }

    pub fn read_bool(&mut self) -> Result<Option<bool>> {
        if self.bits == 0 {
            self.fill(1)?;
            if self.bits == 0 {
                return Ok(None);
            }
        }
        let value = (self.buffer >> 63) == 1;
        self.buffer <<= 1;
        self.bits -= 1;
        Ok(Some(value))
    }

    pub fn read_u8(&mut self) -> Result<Option<u8>> {
        if self.bits < 8 {
            self.fill(1)?;
            if self.bits < 8 {
                return Ok(None);
            }
        }
        let value = (self.buffer >> 56) as u8;
        self.buffer <<= 8;
        self.bits -= 8;
        Ok(Some(value))
    }

    pub fn read_u16(&mut self) -> Result<Option<u16>> {
        if self.bits < 16 {
            self.fill(2)?;
            if self.bits < 16 {
                return Ok(None);
            }
        }
        let value = (self.buffer >> 48) as u16;
        self.buffer <<= 16;
        self.bits -= 16;
        Ok(Some(value))
    }
}
