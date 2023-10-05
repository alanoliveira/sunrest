mod address_increment;
mod nametable;
mod pattern_table;
mod scroll;
mod spr_height;
mod vram_address;

pub use address_increment::AddressIncrement;
pub use nametable::Nametable;
pub use pattern_table::PatternTable;
pub use scroll::Scroll;
pub use spr_height::SprHeight;
pub use vram_address::VramAddress;

#[derive(Debug, Default, Clone)]
pub struct Registers {
    pub latch: Option<u8>,

    pub vram_addr: VramAddress,
    pub vram_data: u8,
    pub addres_increment: AddressIncrement,
    pub scroll: Scroll,
    pub spr_pattern_table: PatternTable,
    pub bg_pattern_table: PatternTable,
    pub spr_height: SprHeight,
    pub nametable: Nametable,
    pub oam_addr: u8,

    pub clip_bg: bool,
    pub clip_spr: bool,
    pub show_bg: bool,
    pub show_spr: bool,

    pub spr0_hit: bool,
    pub spr0_found: bool,
    pub spr_overflow: bool,
    pub vblank_occurred: Option<()>,
    pub nmi_enabled: bool,
    pub nmi_suppressed: bool,
}

impl Registers {
    pub fn set_scroll(&mut self, x: u8, y: u8) {
        self.scroll.set_x(x);
        self.scroll.set_y(y);
    }

    pub fn set_vram_address(&mut self, val: u16) {
        self.vram_addr.set(val);
        self.scroll.x.set_coarse(self.vram_addr.coarse_x());
        self.scroll.y.set_coarse(self.vram_addr.coarse_y());
        self.scroll.y.set_fine(self.vram_addr.fine_y());
        self.nametable = self.vram_addr.nametable().into();
    }

    pub fn update_vram_address_x(&mut self) {
        self.vram_addr.set_coarse_x(self.scroll.x.coarse());
        self.vram_addr.set_nametable_h(self.nametable.h());
    }

    pub fn update_vram_address_y(&mut self) {
        self.vram_addr.set_coarse_y(self.scroll.y.coarse());
        self.vram_addr.set_fine_y(self.scroll.y.fine());
        self.vram_addr.set_nametable_v(self.nametable.v());
    }

    pub fn increment_vram_address(&mut self) {
        self.vram_addr.increment(self.addres_increment as u16);
    }

    pub fn render_enabled(&self) -> bool {
        self.show_bg || self.show_spr
    }
}
