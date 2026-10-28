use pc_keyboard::{DecodedKey, HandleControl, KeyEvent, KeyState, PS2Keyboard, ScancodeSet1, layouts::Us104Key};
use spin::Mutex;
use x86_64::{instructions::port::Port, structures::idt::InterruptStackFrame};
use pic8259::ChainedPics;
use spin::LazyLock;
use x86_64::structures::idt::InterruptDescriptorTable;

static IDT: LazyLock<InterruptDescriptorTable> = LazyLock::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    idt[33].set_handler_fn(interrup_keyboard);
    idt
});

pub fn init_idt() {
    IDT.load();
}

pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe{ChainedPics::new(32, 40)});
pub static KEYBOARD: Mutex<PS2Keyboard<Us104Key, ScancodeSet1>> = Mutex::new(
    PS2Keyboard::new(ScancodeSet1::new(), Us104Key, HandleControl::Ignore));

extern "x86-interrupt" fn interrup_keyboard(
    _frame: InterruptStackFrame
) {
    let mut port = Port::<u8>::new(0x60);
    let scancode: u8 = unsafe{ port.read() };
    
    let mut keyboard = KEYBOARD.lock();

    if let Ok(Some(ev)) = keyboard.add_byte(scancode) {
        if ev.state == KeyState::Down {
            if let Some(key) = keyboard.process_keyevent(ev) {
                match key {
                    DecodedKey::Unicode(ch) => crate::terminal::print_char(ch),
                    _ => ()
                }
            }
        }
    }
    unsafe{
        PICS.lock().notify_end_of_interrupt(33 as u8)
    };
}
