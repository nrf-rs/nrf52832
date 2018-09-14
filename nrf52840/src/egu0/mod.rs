#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Description collection[n]: Trigger n for triggering the corresponding TRIGGERED[n] event"]
    pub tasks_trigger: [TASKS_TRIGGER; 16],
    _reserved1: [u8; 192usize],
    #[doc = "0x100 - Description collection[n]: Event number n generated by triggering the corresponding TRIGGER[n] task"]
    pub events_triggered: [EVENTS_TRIGGERED; 16],
    _reserved2: [u8; 448usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
}
#[doc = "Description collection[n]: Trigger n for triggering the corresponding TRIGGERED[n] event"]
pub struct TASKS_TRIGGER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[n]: Trigger n for triggering the corresponding TRIGGERED[n] event"]
pub mod tasks_trigger;
#[doc = "Description collection[n]: Event number n generated by triggering the corresponding TRIGGER[n] task"]
pub struct EVENTS_TRIGGERED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[n]: Event number n generated by triggering the corresponding TRIGGER[n] task"]
pub mod events_triggered;
#[doc = "Enable or disable interrupt"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "Enable interrupt"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable interrupt"]
pub mod intenclr;
