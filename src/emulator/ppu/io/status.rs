pub struct Status {
    pub vertical_blank: bool,
}

impl From<u8> for Status {
    fn from(val: u8) -> Self {
        Self {
            vertical_blank: val & 0b1000_0000 != 0,
        }
    }
}

impl From<Status> for u8 {
    fn from(val: Status) -> Self {
        let mut ret = 0;
        if val.vertical_blank {
            ret |= 0b1000_0000;
        }
        ret
    }
}
