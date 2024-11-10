pub struct Bootrom {
    rom: Box<[u8]>,
    active: bool,
}

impl Bootrom {
    pub fn new(rom: Box<[u8]>) -> Self {
        Self { rom, active: true }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    // 0 以外を書き込むと Boot ROM が無効化される
    pub fn write(&mut self, _: u16, val: u8) {
        self.active &= val == 0;
    }
}