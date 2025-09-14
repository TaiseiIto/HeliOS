pub mod apic;
pub mod descriptor;
pub mod non_maskable;

pub use descriptor::Descriptor;

use {
    crate::{com2_println, memory, processor, task, timer, x64},
    alloc::collections::VecDeque,
};

static mut EVENTS: VecDeque<Event> = VecDeque::new();

pub enum Event {
    ApicTimer,
    Hpet,
    Interprocessor {
        sender_local_apic_id: u8,
        message: processor::message::Content,
    },
    Pit,
    Rtc,
}

impl Event {
    pub fn interprocessor(
        controller: &processor::Controller,
        message: processor::message::Content,
    ) -> Self {
        let sender_local_apic_id: u8 = controller.local_apic_id();
        Self::Interprocessor {
            sender_local_apic_id,
            message,
        }
    }

    pub fn pop() -> Option<Event> {
        task::Controller::get_current_mut().unwrap().cli();
        let event: Option<Event> = unsafe { EVENTS.pop_back() };
        task::Controller::get_current_mut().unwrap().sti();
        event
    }

    pub fn process(self) {
        match self {
            Self::ApicTimer => com2_println!("APIC timer event."),
            Self::Hpet => {
                com2_println!("HPET event.");
                processor::Controller::get_mut_all()
                    .filter(|processor| processor.is_initialized())
                    .for_each(|processor| {
                        processor.send(processor::message::Content::HpetInterrupt)
                    });
            }
            Self::Interprocessor {
                sender_local_apic_id,
                message,
            } => {
                let processor: &mut processor::Controller = processor::Controller::get_mut_all()
                    .find(|processor| processor.local_apic_id() == sender_local_apic_id)
                    .unwrap();
                message.process(processor);
            }
            Self::Pit => {
                com2_println!("PIT event.");
                processor::Controller::get_mut_all()
                    .filter(|processor| processor.is_initialized())
                    .for_each(|processor| {
                        processor.send(processor::message::Content::PitInterrupt)
                    });
            }
            Self::Rtc => {
                com2_println!("RTC event.");
                processor::Controller::get_mut_all()
                    .filter(|processor| processor.is_initialized())
                    .for_each(|processor| {
                        processor.send(processor::message::Content::RtcInterrupt)
                    });
            }
        }
    }

    pub fn push(event: Event) {
        task::Controller::get_current_mut().unwrap().cli();
        unsafe {
            EVENTS.push_front(event);
        }
        task::Controller::get_current_mut().unwrap().sti();
    }
}

pub const APIC_TIMER_INTERRUPT: u8 = 0x98;
pub const HPET_INTERRUPT: u8 = 0x22;
pub const PIT_INTERRUPT: u8 = 0x20;
pub const RTC_INTERRUPT: u8 = 0x28;
pub const INTERPROCESSOR_INTERRUPT: u8 = 0x99;
pub const SPURIOUS_INTERRUPT: u8 = 0x9f;

pub enum Handler {
    WithErrorCode(extern "x86-interrupt" fn(StackFrameAndErrorCode)),
    WithoutErrorCode(extern "x86-interrupt" fn(StackFrame)),
}

impl From<&Handler> for usize {
    fn from(handler: &Handler) -> Self {
        match handler {
            Handler::WithErrorCode(handler) => *handler as Self,
            Handler::WithoutErrorCode(handler) => *handler as Self,
        }
    }
}

impl From<extern "x86-interrupt" fn(StackFrameAndErrorCode)> for Handler {
    fn from(handler: extern "x86-interrupt" fn(StackFrameAndErrorCode)) -> Self {
        Self::WithErrorCode(handler)
    }
}

impl From<extern "x86-interrupt" fn(StackFrame)> for Handler {
    fn from(handler: extern "x86-interrupt" fn(StackFrame)) -> Self {
        Self::WithoutErrorCode(handler)
    }
}

/// # Interrupt Stack Frame
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.14.4 Figure 6-9. IA-32e Mode Stack Usage After Privilege Level Change
#[derive(Debug)]
#[repr(C)]
pub struct StackFrame {
    rip: u64,
    cs: memory::segment::Selector,
    rflags: x64::Rflags,
    rsp: u64,
    ss: memory::segment::Selector,
}

/// # Interrupt Stack Frame and Error Code
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.14.4 Figure 6-9. IA-32e Mode Stack Usage After Privilege Level Change
#[derive(Debug)]
#[repr(C)]
pub struct StackFrameAndErrorCode {
    error_code: u64,
    stack_frame: StackFrame,
}

pub fn register_handlers(idt: &mut descriptor::Table) {
    let handlers: [Handler; 0x100] = [
        (handler_0x00 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x01 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x02 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x03 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x04 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x05 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x06 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x07 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x08 as extern "x86-interrupt" fn(StackFrameAndErrorCode)).into(),
        (handler_0x09 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x0a as extern "x86-interrupt" fn(StackFrameAndErrorCode)).into(),
        (handler_0x0b as extern "x86-interrupt" fn(StackFrameAndErrorCode)).into(),
        (handler_0x0c as extern "x86-interrupt" fn(StackFrameAndErrorCode)).into(),
        (handler_0x0d as extern "x86-interrupt" fn(StackFrameAndErrorCode)).into(),
        (handler_0x0e as extern "x86-interrupt" fn(StackFrameAndErrorCode)).into(),
        (handler_0x0f as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x10 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x11 as extern "x86-interrupt" fn(StackFrameAndErrorCode)).into(),
        (handler_0x12 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x13 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x14 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x15 as extern "x86-interrupt" fn(StackFrameAndErrorCode)).into(),
        (handler_0x16 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x17 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x18 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x19 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x1a as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x1b as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x1c as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x1d as extern "x86-interrupt" fn(StackFrameAndErrorCode)).into(),
        (handler_0x1e as extern "x86-interrupt" fn(StackFrameAndErrorCode)).into(),
        (handler_0x1f as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x20 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x21 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x22 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x23 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x24 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x25 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x26 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x27 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x28 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x29 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x2a as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x2b as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x2c as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x2d as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x2e as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x2f as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x30 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x31 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x32 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x33 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x34 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x35 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x36 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x37 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x38 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x39 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x3a as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x3b as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x3c as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x3d as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x3e as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x3f as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x40 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x41 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x42 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x43 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x44 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x45 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x46 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x47 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x48 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x49 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x4a as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x4b as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x4c as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x4d as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x4e as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x4f as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x50 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x51 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x52 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x53 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x54 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x55 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x56 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x57 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x58 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x59 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x5a as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x5b as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x5c as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x5d as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x5e as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x5f as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x60 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x61 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x62 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x63 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x64 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x65 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x66 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x67 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x68 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x69 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x6a as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x6b as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x6c as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x6d as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x6e as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x6f as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x70 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x71 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x72 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x73 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x74 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x75 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x76 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x77 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x78 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x79 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x7a as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x7b as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x7c as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x7d as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x7e as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x7f as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x80 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x81 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x82 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x83 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x84 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x85 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x86 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x87 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x88 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x89 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x8a as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x8b as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x8c as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x8d as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x8e as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x8f as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x90 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x91 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x92 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x93 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x94 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x95 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x96 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x97 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x98 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x99 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x9a as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x9b as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x9c as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x9d as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x9e as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x9f as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xa0 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xa1 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xa2 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xa3 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xa4 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xa5 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xa6 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xa7 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xa8 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xa9 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xaa as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xab as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xac as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xad as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xae as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xaf as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xb0 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xb1 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xb2 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xb3 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xb4 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xb5 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xb6 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xb7 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xb8 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xb9 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xba as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xbb as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xbc as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xbd as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xbe as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xbf as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xc0 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xc1 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xc2 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xc3 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xc4 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xc5 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xc6 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xc7 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xc8 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xc9 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xca as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xcb as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xcc as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xcd as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xce as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xcf as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xd0 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xd1 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xd2 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xd3 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xd4 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xd5 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xd6 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xd7 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xd8 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xd9 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xda as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xdb as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xdc as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xdd as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xde as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xdf as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xe0 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xe1 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xe2 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xe3 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xe4 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xe5 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xe6 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xe7 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xe8 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xe9 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xea as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xeb as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xec as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xed as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xee as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xef as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xf0 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xf1 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xf2 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xf3 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xf4 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xf5 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xf6 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xf7 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xf8 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xf9 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xfa as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xfb as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xfc as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xfd as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xfe as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xff as extern "x86-interrupt" fn(StackFrame)).into(),
    ];
    let interrupt_stack_table: [u8; 0x100] = [
        2, // int 0x00 Divide Error Exception (\#DE)
        2, // int 0x01 Debug Exception (\#DB)
        2, // int 0x02 NMI Interrupt
        2, // int 0x03 Breakpoint Exception (\#BP)
        2, // int 0x04 Overflow Exception (\#OF)
        2, // int 0x05 BOUND Range Exceeded Exception (\#BR)
        2, // int 0x06 Invalid Opcode Exception (\#UD)
        2, // int 0x07 Device Not Available Exception (\#NM)
        3, // int 0x08 Double Fault Exception (\#DF)
        2, // int 0x09 Coprocessor Segment Overrun
        2, // int 0x0a Invalid TSS Exception (\#TS)
        2, // int 0x0b Segment Not Present (\#NP)
        2, // int 0x0c Stack Fault Exception (\#SS)
        2, // int 0x0d General Protection Exception (\#GP)
        2, // int 0x0e Page-Fault Exception (\#PF)
        2, // int 0x0f Reserved Exception 0
        2, // int 0x10 x87 Floating-Point Error (\#MF)
        2, // int 0x11 Alignment Check Exception (\#AC)
        2, // int 0x12 Machine Check Exception (\#MC)
        2, // int 0x13 SIMD Floating-Point Exception (\#XM)
        2, // int 0x14 Virtualization Exception (\#VE)
        2, // int 0x15 Control Protection Exception (\#CP)
        2, // int 0x16 Reserved Exception 1
        2, // int 0x17 Reserved Exception 2
        2, // int 0x18 Reserved Exception 3
        2, // int 0x19 Reserved Exception 4
        2, // int 0x1a Reserved Exception 5
        2, // int 0x1b Reserved Exception 6
        2, // int 0x1c Hypervisor Injection Exception (\#HV)
        2, // int 0x1d VMM Communication Exception (\#VC)
        2, // int 0x1e Security Exception (\#SX)
        2, // int 0x1f Reserved Exception 7
        1, // int 0x20 IRQ 0x00 PIT
        1, // int 0x21 IRQ 0x01
        1, // int 0x22 IRQ 0x02 HPET
        1, // int 0x23 IRQ 0x03
        1, // int 0x24 IRQ 0x04
        1, // int 0x25 IRQ 0x05
        1, // int 0x26 IRQ 0x06
        1, // int 0x27 IRQ 0x07
        1, // int 0x28 IRQ 0x08 RIC
        1, // int 0x29 IRQ 0x09
        1, // int 0x2a IRQ 0x0a
        1, // int 0x2b IRQ 0x0b
        1, // int 0x2c IRQ 0x0c
        1, // int 0x2d IRQ 0x0d
        1, // int 0x2e IRQ 0x0e
        1, // int 0x2f IRQ 0x0f
        1, // int 0x30 IRQ 0x10
        1, // int 0x31 IRQ 0x11
        1, // int 0x32 IRQ 0x12
        1, // int 0x33 IRQ 0x13
        1, // int 0x34 IRQ 0x14
        1, // int 0x35 IRQ 0x15
        1, // int 0x36 IRQ 0x16
        1, // int 0x37 IRQ 0x17
        1, // int 0x38 IRQ 0x18
        1, // int 0x39 IRQ 0x19
        1, // int 0x3a IRQ 0x1a
        1, // int 0x3b IRQ 0x1b
        1, // int 0x3c IRQ 0x1c
        1, // int 0x3d IRQ 0x1d
        1, // int 0x3e IRQ 0x1e
        1, // int 0x3f IRQ 0x1f
        1, // int 0x40 IRQ 0x20
        1, // int 0x41 IRQ 0x21
        1, // int 0x42 IRQ 0x22
        1, // int 0x43 IRQ 0x23
        1, // int 0x44 IRQ 0x24
        1, // int 0x45 IRQ 0x25
        1, // int 0x46 IRQ 0x26
        1, // int 0x47 IRQ 0x27
        1, // int 0x48 IRQ 0x28
        1, // int 0x49 IRQ 0x29
        1, // int 0x4a IRQ 0x2a
        1, // int 0x4b IRQ 0x2b
        1, // int 0x4c IRQ 0x2c
        1, // int 0x4d IRQ 0x2d
        1, // int 0x4e IRQ 0x2e
        1, // int 0x4f IRQ 0x2f
        1, // int 0x50 IRQ 0x30
        1, // int 0x51 IRQ 0x31
        1, // int 0x52 IRQ 0x32
        1, // int 0x53 IRQ 0x33
        1, // int 0x54 IRQ 0x34
        1, // int 0x55 IRQ 0x35
        1, // int 0x56 IRQ 0x36
        1, // int 0x57 IRQ 0x37
        1, // int 0x58 IRQ 0x38
        1, // int 0x59 IRQ 0x39
        1, // int 0x5a IRQ 0x3a
        1, // int 0x5b IRQ 0x3b
        1, // int 0x5c IRQ 0x3c
        1, // int 0x5d IRQ 0x3d
        1, // int 0x5e IRQ 0x3e
        1, // int 0x5f IRQ 0x3f
        1, // int 0x60 IRQ 0x40
        1, // int 0x61 IRQ 0x41
        1, // int 0x62 IRQ 0x42
        1, // int 0x63 IRQ 0x43
        1, // int 0x64 IRQ 0x44
        1, // int 0x65 IRQ 0x45
        1, // int 0x66 IRQ 0x46
        1, // int 0x67 IRQ 0x47
        1, // int 0x68 IRQ 0x48
        1, // int 0x69 IRQ 0x49
        1, // int 0x6a IRQ 0x4a
        1, // int 0x6b IRQ 0x4b
        1, // int 0x6c IRQ 0x4c
        1, // int 0x6d IRQ 0x4d
        1, // int 0x6e IRQ 0x4e
        1, // int 0x6f IRQ 0x4f
        1, // int 0x70 IRQ 0x50
        1, // int 0x71 IRQ 0x51
        1, // int 0x72 IRQ 0x52
        1, // int 0x73 IRQ 0x53
        1, // int 0x74 IRQ 0x54
        1, // int 0x75 IRQ 0x55
        1, // int 0x76 IRQ 0x56
        1, // int 0x77 IRQ 0x57
        1, // int 0x78 IRQ 0x58
        1, // int 0x79 IRQ 0x59
        1, // int 0x7a IRQ 0x5a
        1, // int 0x7b IRQ 0x5b
        1, // int 0x7c IRQ 0x5c
        1, // int 0x7d IRQ 0x5d
        1, // int 0x7e IRQ 0x5e
        1, // int 0x7f IRQ 0x5f
        1, // int 0x80 IRQ 0x60
        1, // int 0x81 IRQ 0x61
        1, // int 0x82 IRQ 0x62
        1, // int 0x83 IRQ 0x63
        1, // int 0x84 IRQ 0x64
        1, // int 0x85 IRQ 0x65
        1, // int 0x86 IRQ 0x66
        1, // int 0x87 IRQ 0x67
        1, // int 0x88 IRQ 0x68
        1, // int 0x89 IRQ 0x69
        1, // int 0x8a IRQ 0x6a
        1, // int 0x8b IRQ 0x6b
        1, // int 0x8c IRQ 0x6c
        1, // int 0x8d IRQ 0x6d
        1, // int 0x8e IRQ 0x6e
        1, // int 0x8f IRQ 0x6f
        1, // int 0x90 IRQ 0x70
        1, // int 0x91 IRQ 0x71
        1, // int 0x92 IRQ 0x72
        1, // int 0x93 IRQ 0x73
        1, // int 0x94 IRQ 0x74
        1, // int 0x95 IRQ 0x75
        1, // int 0x96 IRQ 0x76
        1, // int 0x97 IRQ 0x77
        1, // int 0x98 APIC timer interrupt
        1, // int 0x99 Interprocessor interrupt
        1, // int 0x9a
        1, // int 0x9b
        1, // int 0x9c
        1, // int 0x9d
        1, // int 0x9e
        1, // int 0x9f Spurious interrupt
        1, // int 0xa0
        1, // int 0xa1
        1, // int 0xa2
        1, // int 0xa3
        1, // int 0xa4
        1, // int 0xa5
        1, // int 0xa6
        1, // int 0xa7
        1, // int 0xa8
        1, // int 0xa9
        1, // int 0xaa
        1, // int 0xab
        1, // int 0xac
        1, // int 0xad
        1, // int 0xae
        1, // int 0xaf
        1, // int 0xb0
        1, // int 0xb1
        1, // int 0xb2
        1, // int 0xb3
        1, // int 0xb4
        1, // int 0xb5
        1, // int 0xb6
        1, // int 0xb7
        1, // int 0xb8
        1, // int 0xb9
        1, // int 0xba
        1, // int 0xbb
        1, // int 0xbc
        1, // int 0xbd
        1, // int 0xbe
        1, // int 0xbf
        1, // int 0xc0
        1, // int 0xc1
        1, // int 0xc2
        1, // int 0xc3
        1, // int 0xc4
        1, // int 0xc5
        1, // int 0xc6
        1, // int 0xc7
        1, // int 0xc8
        1, // int 0xc9
        1, // int 0xca
        1, // int 0xcb
        1, // int 0xcc
        1, // int 0xcd
        1, // int 0xce
        1, // int 0xcf
        1, // int 0xd0
        1, // int 0xd1
        1, // int 0xd2
        1, // int 0xd3
        1, // int 0xd4
        1, // int 0xd5
        1, // int 0xd6
        1, // int 0xd7
        1, // int 0xd8
        1, // int 0xd9
        1, // int 0xda
        1, // int 0xdb
        1, // int 0xdc
        1, // int 0xdd
        1, // int 0xde
        1, // int 0xdf
        1, // int 0xe0
        1, // int 0xe1
        1, // int 0xe2
        1, // int 0xe3
        1, // int 0xe4
        1, // int 0xe5
        1, // int 0xe6
        1, // int 0xe7
        1, // int 0xe8
        1, // int 0xe9
        1, // int 0xea
        1, // int 0xeb
        1, // int 0xec
        1, // int 0xed
        1, // int 0xee
        1, // int 0xef
        1, // int 0xf0
        1, // int 0xf1
        1, // int 0xf2
        1, // int 0xf3
        1, // int 0xf4
        1, // int 0xf5
        1, // int 0xf6
        1, // int 0xf7
        1, // int 0xf8
        1, // int 0xf9
        1, // int 0xfa
        1, // int 0xfb
        1, // int 0xfc
        1, // int 0xfd
        1, // int 0xfe
        1, // int 0xff
    ];
    idt.iter_mut()
        .zip(handlers.as_slice().iter())
        .zip(interrupt_stack_table.as_slice().iter())
        .map(|((descriptor, handler), interrupt_stack_table)| {
            (descriptor, handler, interrupt_stack_table)
        })
        .for_each(|(descriptor, handler, interrupt_stack_table)| {
            let interface = descriptor::Interface::new(handler, *interrupt_stack_table);
            *descriptor = (&interface).into();
        });
}

/// # Divide Error Exception (\#DE)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x00(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x00;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Divide Error Exception (#DE)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Divide Error Exception");
}

/// # Debug Exception (\#DB)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x01(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x01;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Debug Exception (#DB)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Debug Exception");
}

/// # NMI Interrupt
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x02(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x02;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("NMI Interrupt");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("NMI Interrupt");
}

/// # Breakpoint Exception (\#BP)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x03(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x03;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Breakpoint Exception (#BP)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Breakpoint Exception");
}

/// # Overflow Exception (\#OF)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x04(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x04;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Overflow Exception (#OF)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Overflow Exception");
}

/// # BOUND Range Exceeded Exception (\#BR)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x05(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x05;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("BOUND Range Exceeded Exception (#BR)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("BOUND Range Exceeded Exception");
}

/// # Invalid Opcode Exception (\#UD)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x06(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x06;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Invalid Opcode Exception (#UD)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Invalid Opcode Exception");
}

/// # Device Not Available Exception (\#NM)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x07(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x07;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Device Not Available Exception (#NM)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Device Not Available Exception");
}

/// # Double Fault Exception (\#DF)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x08(stack_frame_and_error_code: StackFrameAndErrorCode) {
    let interrupt_number: u8 = 0x08;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Double Fault Exception (#DF)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!(
        "stack_frame_and_error_code = {:#x?}",
        stack_frame_and_error_code
    );
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Double Fault Exception");
}

/// # Coprocessor Segment Overrun
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x09(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x09;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Coprocessor Segment Overrun");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Coprocessor Segment Overrun");
}

/// # Invalid TSS Exception (\#TS)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x0a(stack_frame_and_error_code: StackFrameAndErrorCode) {
    let interrupt_number: u8 = 0x0a;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Invalid TSS Exception (#TS)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!(
        "stack_frame_and_error_code = {:#x?}",
        stack_frame_and_error_code
    );
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Invalid TSS Exception");
}

/// # Segment Not Present (\#NP)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x0b(stack_frame_and_error_code: StackFrameAndErrorCode) {
    let interrupt_number: u8 = 0x0b;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Segment Not Present (#NP)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!(
        "stack_frame_and_error_code = {:#x?}",
        stack_frame_and_error_code
    );
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Segment Not Present");
}

/// # Stack Fault Exception (\#SS)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x0c(stack_frame_and_error_code: StackFrameAndErrorCode) {
    let interrupt_number: u8 = 0x0c;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Stack Fault Exception (#SS)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!(
        "stack_frame_and_error_code = {:#x?}",
        stack_frame_and_error_code
    );
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Stack Fault Exception");
}

/// # General Protection Exception (\#GP)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x0d(stack_frame_and_error_code: StackFrameAndErrorCode) {
    let interrupt_number: u8 = 0x0d;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("General Protection Exception (#GP)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!(
        "stack_frame_and_error_code = {:#x?}",
        stack_frame_and_error_code
    );
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("General Protection Exception");
}

/// # Page-Fault Exception (\#PF)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x0e(stack_frame_and_error_code: StackFrameAndErrorCode) {
    let interrupt_number: u8 = 0x0e;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Page-Fault Exception (#PF)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!(
        "stack_frame_and_error_code = {:#x?}",
        stack_frame_and_error_code
    );
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Page-Fault Exception");
}

/// # Reserved Exception 0
/// ## References
/// * [Exceptions - OSDev Wiki](https://wiki.osdev.org/Exceptions)
extern "x86-interrupt" fn handler_0x0f(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x0f;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Reserved Exception 0");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Reserved Exception 0");
}

/// # x87 Floating-Point Error (\#MF)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x10(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x10;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("x87 Floating-Point Error (#MF)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("x87 Floating-Point Error");
}

/// # Alignment Check Exception (\#AC)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x11(stack_frame_and_error_code: StackFrameAndErrorCode) {
    let interrupt_number: u8 = 0x11;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Alignment Check Exception (#AC)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!(
        "stack_frame_and_error_code = {:#x?}",
        stack_frame_and_error_code
    );
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Alignment Check Exception");
}

/// # Machine Check Exception (\#MC)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x12(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x12;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Machine Check Exception (#MC)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Machine Check Exception");
}

/// # SIMD Floating-Point Exception (\#XM)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x13(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x13;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("SIMD Floating-Point Exception (#XM)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("SIMD Floating-Point Exception");
}

/// # Virtualization Exception (\#VE)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x14(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x14;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Virtualization Exception (#VE)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Virtualization Exception");
}

/// # Control Protection Exception (\#CP)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x15(stack_frame_and_error_code: StackFrameAndErrorCode) {
    let interrupt_number: u8 = 0x15;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Control Protection Exception (#CP)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!(
        "stack_frame_and_error_code = {:#x?}",
        stack_frame_and_error_code
    );
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Control Protection Exception");
}

/// # Reserved Exception 1
/// ## References
/// * [Exceptions - OSDev Wiki](https://wiki.osdev.org/Exceptions)
extern "x86-interrupt" fn handler_0x16(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x16;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Reserved Exception 1");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Reserved Exception 1");
}

/// # Reserved Exception 2
/// ## References
/// * [Exceptions - OSDev Wiki](https://wiki.osdev.org/Exceptions)
extern "x86-interrupt" fn handler_0x17(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x17;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Reserved Exception 2");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Reserved Exception 2");
}

/// # Reserved Exception 3
/// ## References
/// * [Exceptions - OSDev Wiki](https://wiki.osdev.org/Exceptions)
extern "x86-interrupt" fn handler_0x18(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x18;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Reserved Exception 3");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Reserved Exception 3");
}

/// # Reserved Exception 4
/// ## References
/// * [Exceptions - OSDev Wiki](https://wiki.osdev.org/Exceptions)
extern "x86-interrupt" fn handler_0x19(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x19;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Reserved Exception 4");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Reserved Exception 4");
}

/// # Reserved Exception 5
/// ## References
/// * [Exceptions - OSDev Wiki](https://wiki.osdev.org/Exceptions)
extern "x86-interrupt" fn handler_0x1a(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x1a;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Reserved Exception 5");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Reserved Exception 5");
}

/// # Reserved Exception 6
/// ## References
/// * [Exceptions - OSDev Wiki](https://wiki.osdev.org/Exceptions)
extern "x86-interrupt" fn handler_0x1b(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x1b;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Reserved Exception 6");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Reserved Exception 6");
}

/// # Hypervisor Injection Exception (\#HV)
/// ## References
/// * [Exceptions - OSDev Wiki](https://wiki.osdev.org/Exceptions)
extern "x86-interrupt" fn handler_0x1c(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x1c;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Hypervisor Injection Exception (#HV)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Hypervisor Injection Exception");
}

/// # VMM Communication Exception (\#VC)
/// ## References
/// * [Exceptions - OSDev Wiki](https://wiki.osdev.org/Exceptions)
extern "x86-interrupt" fn handler_0x1d(stack_frame_and_error_code: StackFrameAndErrorCode) {
    let interrupt_number: u8 = 0x1d;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("VMM Communication Exception (#VC)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!(
        "stack_frame_and_error_code = {:#x?}",
        stack_frame_and_error_code
    );
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("VMM Communication Exception");
}

/// # Security Exception (\#SX)
/// ## References
/// * [Exceptions - OSDev Wiki](https://wiki.osdev.org/Exceptions)
extern "x86-interrupt" fn handler_0x1e(stack_frame_and_error_code: StackFrameAndErrorCode) {
    let interrupt_number: u8 = 0x1e;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Security Exception (#SX)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!(
        "stack_frame_and_error_code = {:#x?}",
        stack_frame_and_error_code
    );
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Security Exception");
}

/// # Reserved Exception 7
/// ## References
/// * [Exceptions - OSDev Wiki](https://wiki.osdev.org/Exceptions)
extern "x86-interrupt" fn handler_0x1f(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x1f;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Reserved Exception 7");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
    panic!("Reserved Exception 7");
}

/// # IRQ 0x00 PIT interrupt
extern "x86-interrupt" fn handler_0x20(_stack_frame: StackFrame) {
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    x64::msr::ia32::ApicBase::get()
        .unwrap()
        .registers_mut()
        .end_interruption();
    Event::push(Event::Pit);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x01
extern "x86-interrupt" fn handler_0x21(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x21;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x02 HPET interrupt
extern "x86-interrupt" fn handler_0x22(_stack_frame: StackFrame) {
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    x64::msr::ia32::ApicBase::get()
        .unwrap()
        .registers_mut()
        .end_interruption();
    Event::push(Event::Hpet);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x03
extern "x86-interrupt" fn handler_0x23(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x23;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x04
extern "x86-interrupt" fn handler_0x24(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x24;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x05
extern "x86-interrupt" fn handler_0x25(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x25;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x06
extern "x86-interrupt" fn handler_0x26(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x26;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x07
extern "x86-interrupt" fn handler_0x27(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x27;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x08 RIC
extern "x86-interrupt" fn handler_0x28(_stack_frame: StackFrame) {
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    x64::msr::ia32::ApicBase::get()
        .unwrap()
        .registers_mut()
        .end_interruption();
    timer::rtc::end_interruption();
    Event::push(Event::Rtc);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x09
extern "x86-interrupt" fn handler_0x29(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x29;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x0a
extern "x86-interrupt" fn handler_0x2a(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x2a;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x0b
extern "x86-interrupt" fn handler_0x2b(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x2b;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x0c
extern "x86-interrupt" fn handler_0x2c(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x2c;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x0d
extern "x86-interrupt" fn handler_0x2d(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x2d;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x0e
extern "x86-interrupt" fn handler_0x2e(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x2e;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x0f
extern "x86-interrupt" fn handler_0x2f(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x2f;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x10
extern "x86-interrupt" fn handler_0x30(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x30;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x11
extern "x86-interrupt" fn handler_0x31(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x31;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x12
extern "x86-interrupt" fn handler_0x32(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x32;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x13
extern "x86-interrupt" fn handler_0x33(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x33;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x14
extern "x86-interrupt" fn handler_0x34(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x34;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x15
extern "x86-interrupt" fn handler_0x35(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x35;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x16
extern "x86-interrupt" fn handler_0x36(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x36;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x17
extern "x86-interrupt" fn handler_0x37(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x37;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x18
extern "x86-interrupt" fn handler_0x38(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x38;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x19
extern "x86-interrupt" fn handler_0x39(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x39;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x1a
extern "x86-interrupt" fn handler_0x3a(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x3a;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x1b
extern "x86-interrupt" fn handler_0x3b(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x3b;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x1c
extern "x86-interrupt" fn handler_0x3c(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x3c;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x1d
extern "x86-interrupt" fn handler_0x3d(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x3d;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x1e
extern "x86-interrupt" fn handler_0x3e(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x3e;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x1f
extern "x86-interrupt" fn handler_0x3f(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x3f;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x20
extern "x86-interrupt" fn handler_0x40(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x40;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x21
extern "x86-interrupt" fn handler_0x41(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x41;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x22
extern "x86-interrupt" fn handler_0x42(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x42;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x23
extern "x86-interrupt" fn handler_0x43(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x43;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x24
extern "x86-interrupt" fn handler_0x44(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x44;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x25
extern "x86-interrupt" fn handler_0x45(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x45;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x26
extern "x86-interrupt" fn handler_0x46(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x46;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x27
extern "x86-interrupt" fn handler_0x47(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x47;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x28
extern "x86-interrupt" fn handler_0x48(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x48;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x29
extern "x86-interrupt" fn handler_0x49(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x49;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x2a
extern "x86-interrupt" fn handler_0x4a(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x4a;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x2b
extern "x86-interrupt" fn handler_0x4b(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x4b;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x2c
extern "x86-interrupt" fn handler_0x4c(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x4c;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x2d
extern "x86-interrupt" fn handler_0x4d(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x4d;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x2e
extern "x86-interrupt" fn handler_0x4e(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x4e;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x2f
extern "x86-interrupt" fn handler_0x4f(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x4f;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x30
extern "x86-interrupt" fn handler_0x50(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x50;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x31
extern "x86-interrupt" fn handler_0x51(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x51;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x32
extern "x86-interrupt" fn handler_0x52(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x52;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x33
extern "x86-interrupt" fn handler_0x53(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x53;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x34
extern "x86-interrupt" fn handler_0x54(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x54;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x35
extern "x86-interrupt" fn handler_0x55(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x55;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x36
extern "x86-interrupt" fn handler_0x56(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x56;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x37
extern "x86-interrupt" fn handler_0x57(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x57;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x38
extern "x86-interrupt" fn handler_0x58(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x58;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x39
extern "x86-interrupt" fn handler_0x59(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x59;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x3a
extern "x86-interrupt" fn handler_0x5a(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x5a;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x3b
extern "x86-interrupt" fn handler_0x5b(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x5b;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x3c
extern "x86-interrupt" fn handler_0x5c(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x5c;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x3d
extern "x86-interrupt" fn handler_0x5d(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x5d;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x3e
extern "x86-interrupt" fn handler_0x5e(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x5e;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x3f
extern "x86-interrupt" fn handler_0x5f(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x5f;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x40
extern "x86-interrupt" fn handler_0x60(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x60;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x41
extern "x86-interrupt" fn handler_0x61(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x61;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x42
extern "x86-interrupt" fn handler_0x62(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x62;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x43
extern "x86-interrupt" fn handler_0x63(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x63;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x44
extern "x86-interrupt" fn handler_0x64(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x64;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x45
extern "x86-interrupt" fn handler_0x65(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x65;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x46
extern "x86-interrupt" fn handler_0x66(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x66;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x47
extern "x86-interrupt" fn handler_0x67(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x67;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x48
extern "x86-interrupt" fn handler_0x68(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x68;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x49
extern "x86-interrupt" fn handler_0x69(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x69;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x4a
extern "x86-interrupt" fn handler_0x6a(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x6a;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x4b
extern "x86-interrupt" fn handler_0x6b(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x6b;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x4c
extern "x86-interrupt" fn handler_0x6c(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x6c;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x4d
extern "x86-interrupt" fn handler_0x6d(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x6d;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x4e
extern "x86-interrupt" fn handler_0x6e(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x6e;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x4f
extern "x86-interrupt" fn handler_0x6f(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x6f;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x50
extern "x86-interrupt" fn handler_0x70(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x70;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x51
extern "x86-interrupt" fn handler_0x71(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x71;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x52
extern "x86-interrupt" fn handler_0x72(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x72;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x53
extern "x86-interrupt" fn handler_0x73(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x73;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x54
extern "x86-interrupt" fn handler_0x74(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x74;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x55
extern "x86-interrupt" fn handler_0x75(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x75;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x56
extern "x86-interrupt" fn handler_0x76(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x76;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x57
extern "x86-interrupt" fn handler_0x77(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x77;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x58
extern "x86-interrupt" fn handler_0x78(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x78;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x59
extern "x86-interrupt" fn handler_0x79(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x79;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x5a
extern "x86-interrupt" fn handler_0x7a(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x7a;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x5b
extern "x86-interrupt" fn handler_0x7b(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x7b;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x5c
extern "x86-interrupt" fn handler_0x7c(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x7c;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x5d
extern "x86-interrupt" fn handler_0x7d(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x7d;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x5e
extern "x86-interrupt" fn handler_0x7e(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x7e;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x5f
extern "x86-interrupt" fn handler_0x7f(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x7f;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x60
extern "x86-interrupt" fn handler_0x80(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x80;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x61
extern "x86-interrupt" fn handler_0x81(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x81;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x62
extern "x86-interrupt" fn handler_0x82(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x82;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x63
extern "x86-interrupt" fn handler_0x83(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x83;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x64
extern "x86-interrupt" fn handler_0x84(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x84;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x65
extern "x86-interrupt" fn handler_0x85(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x85;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x66
extern "x86-interrupt" fn handler_0x86(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x86;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x67
extern "x86-interrupt" fn handler_0x87(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x87;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x68
extern "x86-interrupt" fn handler_0x88(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x88;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x69
extern "x86-interrupt" fn handler_0x89(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x89;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x6a
extern "x86-interrupt" fn handler_0x8a(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x8a;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x6b
extern "x86-interrupt" fn handler_0x8b(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x8b;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x6c
extern "x86-interrupt" fn handler_0x8c(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x8c;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x6d
extern "x86-interrupt" fn handler_0x8d(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x8d;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x6e
extern "x86-interrupt" fn handler_0x8e(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x8e;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x6f
extern "x86-interrupt" fn handler_0x8f(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x8f;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x70
extern "x86-interrupt" fn handler_0x90(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x90;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x71
extern "x86-interrupt" fn handler_0x91(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x91;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x72
extern "x86-interrupt" fn handler_0x92(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x92;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x73
extern "x86-interrupt" fn handler_0x93(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x93;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x74
extern "x86-interrupt" fn handler_0x94(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x94;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x75
extern "x86-interrupt" fn handler_0x95(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x95;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x76
extern "x86-interrupt" fn handler_0x96(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x96;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # IRQ 0x77
extern "x86-interrupt" fn handler_0x97(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x97;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # APIC timer interrupt
extern "x86-interrupt" fn handler_0x98(_stack_frame: StackFrame) {
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    x64::msr::ia32::ApicBase::get()
        .unwrap()
        .registers_mut()
        .end_interruption();
    Event::push(Event::ApicTimer);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # Interprocessor interrupt
extern "x86-interrupt" fn handler_0x99(_stack_frame: StackFrame) {
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    processor::Controller::save_received_messages();
    x64::msr::ia32::ApicBase::get()
        .unwrap()
        .registers_mut()
        .end_interruption();
    processor::Controller::delete_received_messages();
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0x9a(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x9a;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0x9b(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x9b;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0x9c(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x9c;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0x9d(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x9d;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0x9e(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x9e;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

/// # Spurious Interrupt
extern "x86-interrupt" fn handler_0x9f(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x9f;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("Spurious Interrupt");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xa0(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xa0;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xa1(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xa1;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xa2(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xa2;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xa3(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xa3;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xa4(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xa4;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xa5(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xa5;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xa6(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xa6;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xa7(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xa7;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xa8(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xa8;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xa9(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xa9;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xaa(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xaa;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xab(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xab;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xac(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xac;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xad(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xad;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xae(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xae;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xaf(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xaf;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xb0(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xb0;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xb1(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xb1;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xb2(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xb2;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xb3(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xb3;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xb4(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xb4;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xb5(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xb5;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xb6(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xb6;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xb7(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xb7;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xb8(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xb8;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xb9(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xb9;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xba(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xba;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xbb(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xbb;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xbc(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xbc;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xbd(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xbd;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xbe(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xbe;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xbf(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xbf;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xc0(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xc0;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xc1(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xc1;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xc2(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xc2;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xc3(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xc3;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xc4(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xc4;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xc5(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xc5;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xc6(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xc6;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xc7(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xc7;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xc8(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xc8;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xc9(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xc9;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xca(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xca;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xcb(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xcb;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xcc(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xcc;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xcd(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xcd;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xce(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xce;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xcf(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xcf;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xd0(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xd0;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xd1(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xd1;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xd2(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xd2;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xd3(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xd3;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xd4(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xd4;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xd5(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xd5;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xd6(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xd6;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xd7(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xd7;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xd8(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xd8;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xd9(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xd9;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xda(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xda;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xdb(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xdb;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xdc(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xdc;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xdd(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xdd;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xde(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xde;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xdf(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xdf;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xe0(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xe0;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xe1(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xe1;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xe2(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xe2;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xe3(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xe3;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xe4(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xe4;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xe5(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xe5;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xe6(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xe6;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xe7(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xe7;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xe8(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xe8;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xe9(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xe9;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xea(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xea;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xeb(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xeb;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xec(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xec;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xed(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xed;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xee(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xee;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xef(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xef;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xf0(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xf0;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xf1(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xf1;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xf2(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xf2;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xf3(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xf3;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xf4(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xf4;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xf5(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xf5;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xf6(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xf6;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xf7(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xf7;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xf8(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xf8;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xf9(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xf9;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xfa(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xfa;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xfb(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xfb;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xfc(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xfc;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xfd(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xfd;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xfe(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xfe;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}

extern "x86-interrupt" fn handler_0xff(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xff;
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.start_interrupt();
    }
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
    if let Some(current_task) = task::Controller::get_current_mut() {
        current_task.end_interrupt();
    }
}
