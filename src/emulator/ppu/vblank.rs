#[derive(Debug, Default)]
pub struct VblankHandler {
    nmi_enabled: bool,
    is_running: bool,
    nmi: Option<()>,
}

impl VblankHandler {
    pub fn get(&self) -> bool {
        self.is_running
    }

    pub fn enable_nmi(&mut self, enable: bool) {
        self.nmi_enabled = enable;
    }

    pub fn start(&mut self) {
        self.is_running = true;
        if self.nmi_enabled {
            self.nmi = Some(());
        }
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn take_nmi(&mut self) -> bool {
        self.nmi.take().is_some()
    }
}
