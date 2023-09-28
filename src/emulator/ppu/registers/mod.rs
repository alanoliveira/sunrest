mod address_increment;
mod name_table;
mod pattern_table;
mod scroll;
mod spr_height;
mod vblank;
mod vram_address;

pub use address_increment::AddressIncrement;
pub use name_table::NameTable;
pub use pattern_table::PatternTable;
pub use scroll::Scroll;
pub use spr_height::SprHeight;
pub use vblank::Vblank;
pub use vram_address::VramAddress;

#[derive(Debug, Default)]
pub struct Registers {
    pub latch: Option<u8>,

    pub vram_addr: VramAddress,
    pub vram_data: u8,
    pub addres_increment: AddressIncrement,
    pub scroll: Scroll,
    pub spr_pattern_table: PatternTable,
    pub bg_pattern_table: PatternTable,
    pub spr_height: SprHeight,
    pub name_table: NameTable,
    pub oam_addr: u8,

    pub clip_bg: bool,
    pub clip_spr: bool,
    pub show_bg: bool,
    pub show_spr: bool,

    pub spr0_hit: bool,
    pub spr_overflow: bool,
    pub vblank: Vblank,
    pub nmi_enabled: bool,
    pub nmi: Option<()>,
}

impl Registers {
    pub fn start_vblank(&mut self) {
        self.vblank.set(true);
        if self.nmi_enabled {
            self.nmi = Some(());
        }
    }

    pub fn stop_vblank(&mut self) {
        self.vblank.set(false);
    }

    pub fn set_scroll(&mut self, x: u8, y: u8) {
        self.scroll.set_x(x);
        self.scroll.set_y(y);
    }

    pub fn set_vram_address(&mut self, val: u16) {
        self.vram_addr.set(val);
        self.scroll.x.set_coarse(self.vram_addr.coarse_x());
        self.scroll.y.set_coarse(self.vram_addr.coarse_y());
        self.scroll.y.set_fine(self.vram_addr.fine_y());
        self.name_table = self.vram_addr.name_table().into();
    }

    pub fn update_vram_address_x(&mut self) {
        self.vram_addr.set_coarse_x(self.scroll.x.coarse());
        self.vram_addr.set_name_table_h(self.name_table.h());
    }

    pub fn update_vram_address_y(&mut self) {
        self.vram_addr.set_coarse_y(self.scroll.y.coarse());
        self.vram_addr.set_fine_y(self.scroll.y.fine());
        self.vram_addr.set_name_table_v(self.name_table.v());
    }

    pub fn increment_vram_address(&mut self) {
        self.vram_addr.increment(self.addres_increment as u16);
    }

    pub fn render_enabled(&self) -> bool {
        self.show_bg || self.show_spr
    }
}
