pub mod header;
use super::cartridge::header::RomHeader;

use std::io::Read;

fn human_readable(size : usize) -> String {

    // auxiliar function to convert from byte size to human readable representation

    let mut sz = size as f32; 
    let units = vec!["B","KiB","MiB","GiB","TiB"];
    let mut idx = 0;
    
    let result = loop {
        if sz < 1024.0 {
            break format!("{:.1} {}", sz, units[idx]);
        }
        sz /= 1024.0;
        idx += 1;
    };
    result
}

#[derive(Debug, Default)]
pub struct CartContext {
    pub header   : RomHeader,
    pub rom_size : usize,
    pub rom_data : Vec<u8>,
}

impl CartContext {
    pub fn new() -> Self {
        CartContext {
            header   : RomHeader::new(),
            rom_data : vec![0u8; 0x8000],
            rom_size : 0x8000,
        }
    }
    pub fn load(&mut self, filename : &str) -> Result<(), ()> {

        let mut file = std::fs::File::open(filename).expect("Run open failed");

        file.read_to_end(&mut self.rom_data).expect("Run read failed");

        self.rom_size = self.rom_data.len();

        self.header.load(&self.rom_data[0x100..=0x14F]);

        let cart_type = match self.header.cart_type {
            0x00    => "ROM ONLY",
            0x01    => "MBC1",
            0x02    => "MBC1+RAM",
            0x03    => "MBC1+RAM+BATTERY",
            0x05    => "MBC2",
            0x06    => "MBC2+BATTERY",
            0x08    => "ROM+RAM",
            0x09    => "ROM+RAM+BATTERY",
            0x0B    => "MMM01",
            0x0C    => "MMM01+RAM",
            0x0D    => "MMM01+RAM+BATTERY",
            0x0F    => "MBC3+TIMER+BATTERY",
            0x10    => "MBC3+TIMER+RAM+BATTERY",
            0x11    => "MBC3",
            0x12    => "MBC3+RAM",
            0x13    => "MBC3+RAM+BATTERY",
            0x19    => "MBC5",
            0x1A    => "MBC5+RAM",
            0x1B    => "MBC5+RAM+BATTERY",
            0x1C    => "MBC5+RUMBLE",
            0x1D    => "MBC5+RUMBLE+RAM",
            0x1E    => "MBC5+RUMBLE+RAM+BATTERY",
            0x20    => "MBC6",
            0x22    => "MBC7+SENSOR+RUMBLE+RAM+BATTERY",
            0xFC    => "POCKET CAMERA",
            0xFD    => "BANDAI TAMA5",
            0xFE    => "HuC3",
            0xFF    => "HuC1+RAM+BATTERY",
            _       => "Unknown"
        };

        let rom_size = human_readable(match self.header.rom_size {
            0x00..=0x08 => (1 << (15 + self.header.rom_size)),
            0x52        => ((1.1 * ((1 << 20) as f64)) as usize),
            0x53        => ((1.2 * ((1 << 20) as f64)) as usize),
            0x54        => ((1.5 * ((1 << 20) as f64)) as usize),
            _           => 0
        });

        let ram_size = match self.header.ram_size {
            0x00    => "No RAM",
            0x01    => "Unused",
            0x02    => "1 bank",
            0x03    => "4 banks of 8 KiB each",
            0x04    => "16 banks of 8 KiB each",
            0x05    => "8 banks of 8 KiB each",
            _       => "Unknown"
        };

        let lic_name = match self.header.lic_code {
            0x33 => {
                match self.header.new_lic_code.as_str() {
                    "00"    => "None",
                    "01"    => "Nintendo R&D1",
                    "08"    => "Capcom",
                    "13"    => "Electronic Arts",
                    "18"    => "Hudson Soft",
                    "19"    => "b-ai",
                    "20"    => "kss",
                    "22"    => "pow",
                    "24"    => "PCM Complete",
                    "25"    => "san-x",
                    "28"    => "Kemco Japan",
                    "29"    => "seta",
                    "30"    => "Viacom",
                    "31"    => "Nintendo",
                    "32"    => "Bandai",
                    "33"    => "Ocean/Acclaim",
                    "34"    => "Konami",
                    "35"    => "Hector",
                    "37"    => "Taito",
                    "38"    => "Hudson",
                    "39"    => "Banpresto",
                    "41"    => "Ubi Soft",
                    "42"    => "Atlus",
                    "44"    => "Malibu",
                    "46"    => "angel",
                    "47"    => "Bullet-Proof",
                    "49"    => "irem",
                    "50"    => "Absolute",
                    "51"    => "Acclaim",
                    "52"    => "Activision",
                    "53"    => "American sammy",
                    "54"    => "Konami",
                    "55"    => "Hi tech entertainment",
                    "56"    => "LJN",
                    "57"    => "Matchbox",
                    "58"    => "Mattel",
                    "59"    => "Milton Bradley",
                    "60"    => "Titus",
                    "61"    => "Virgin",
                    "64"    => "LucasArts",
                    "67"    => "Ocean",
                    "69"    => "Electronic Arts",
                    "70"    => "Infogrames",
                    "71"    => "Interplay",
                    "72"    => "Broderbund",
                    "73"    => "sculptured",
                    "75"    => "sci",
                    "78"    => "THQ",
                    "79"    => "Accolade",
                    "80"    => "misawa",
                    "83"    => "lozc",
                    "86"    => "Tokuma Shoten Intermedia",
                    "87"    => "Tsukuda Original",
                    "91"    => "Chunsoft",
                    "92"    => "Video system",
                    "93"    => "Ocean/Acclaim",
                    "95"    => "Varie",
                    "96"    => "Yonezawa/s'pal",
                    "97"    => "Kaneko",
                    "99"    => "Pack in soft",
                    "A4"    => "Konami (Yu-Gi-Oh!)",
                    _       => "Unknown"
                }
            }
            _ => "Unknown"
        };
        println!("Cartridge Loaded");
        println!("\t Title          : {}", self.header.title);
        println!("\t Cartridge Type : {:#04X} ({})", self.header.cart_type, cart_type);
        println!("\t SGB Support    : {}", if self.header.sgb_flag == 0x03 {"Yes"} else {"No"});
        println!("\t ROM Size       : {:#04X} ({})", self.header.rom_size, rom_size);
        println!("\t RAM Size       : {:#04X} ({})", self.header.ram_size, ram_size);
        println!("\t LIC Code       : {:#04X} ({})", self.header.lic_code, lic_name);

        let mut checksum : u8 = 0;
        for address in 0x134..=0x14C {
            checksum = checksum.wrapping_sub(self.rom_data[address]).wrapping_sub(1);
        }
        // If the byte at $014D does not match the lower 8 bits of checksum, 
        // the boot ROM will lock up and the program in the cartridge won’t run.
        assert_eq!(self.header.checksum, (checksum & 0xFF), "Checksum FAILED"); // will panic if false
        println!("Checksum PASSED");

        //println!("{0:?}", self.header);

        Ok(())
    }
    pub fn read(&self, address : u16)  -> u8 {
        self.rom_data[address as usize]
    }
    pub fn write(&mut self, address : u16, value : u8) {
        self.rom_data[address as usize] = value
    }
}
