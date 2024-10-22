pub struct BytesReader<'a> {
    bytes: &'a [u8],
}

impl<'a> BytesReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_at_end(&self) -> bool {
        self.len() == 0
    }

    pub fn skip(&mut self) {
        self.read();
    }

    pub fn read(&mut self) -> u8 {
        self.read_n(1)[0]
    }

    pub fn read_n(&mut self, len: usize) -> &'a [u8] {
        let res = &self.bytes[..len];
        self.bytes = &self.bytes[len..];
        res
    }

    pub fn read_all(&mut self) -> &'a [u8] {
        self.read_n(self.len())
    }

    pub fn read_until(&mut self, byte: u8) -> &'a [u8] {
        let pos = self.bytes.iter().position(|x| *x == byte).unwrap();
        let (front, back) = self.bytes.split_at(pos);
        self.bytes = back;
        front
    }
}
