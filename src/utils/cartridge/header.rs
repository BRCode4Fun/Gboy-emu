use std::string::String;

#[derive(Debug, Default)]
pub struct RomHeader {

    pub entry : Vec<u8>,
    pub logo  : Vec<u8>,
    pub title : String,
    
    pub new_lic_code : String,
    pub sgb_flag : u8,
    pub cart_type : u8,
    pub rom_size : u8,
    pub ram_size : u8,
    pub dest_code : u8,
    pub lic_code : u8,
    pub version : u8,
    pub checksum : u8,
    pub global_checksum : u16
}

impl RomHeader {
    pub fn new() -> Self {
        RomHeader::default()
    }
    pub fn load(&mut self, header : &[u8]) -> Result<(), ()> {

        self.entry = (&header[0x00..=0x03]).to_vec();
        self.logo = (&header[0x04..=0x33]).to_vec();

        self.title = String::from_utf8_lossy(&header[0x034..0x43]).into_owned();
        
        self.new_lic_code = String::from_utf8_lossy(&header[0x044..=0x45]).into_owned();

        self.sgb_flag = header[0x46];

        self.cart_type = header[0x47];

        self.rom_size = header[0x48];
        self.ram_size = header[0x49];

        self.dest_code = header[0x4A];

        self.lic_code = header[0x4B];

        self.version = header[0x4C];

        self.checksum = header[0x4D];

        self.global_checksum = (header[0x4F] as u16) << 8 | header[0x4E] as u16;

        Ok(())
    }
}