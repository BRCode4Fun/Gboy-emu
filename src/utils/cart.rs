#![allow(dead_code, unused)]

#[derive(Debug)]
struct RomHeader {
    
    entry : [u8; 4],
    logo  : [u8; 0x30],

    title : [char; 16],
    new_lic_code : u16,
    sgb_flag : u8,
    r#type : u8,
    rom_size : u8,
    ram_size : u8,
    dest_code : u8,
    lic_code : u8,
    version : u8,
    checksum : u8,
    global_checksum : u16
}

impl RomHeader {
    pub fn load(bytes : &str) -> Result<(), ()> {
        todo!();
    }
}