pub struct Control {
   pub nmi_enabled: bool,
}

impl From<u8> for Control {
    fn from(val: u8) -> Self {
        Self {
            nmi_enabled: val & 0b1000_0000 != 0,
        }
    }
}

impl From<Control> for u8 {
    fn from(val: Control) -> Self {
        let mut ret = 0;
        if val.nmi_enabled {
            ret |= 0b1000_0000;
        }
        ret
    }
}
