use pc_keyboard::{
    DecodedKey, HandleControl, KeyState, PS2Keyboard, ScancodeSet1, layouts::Us104Key
};
use spin::{Mutex, LazyLock};
use x86_64::{
    instructions::port::Port, 
    structures::{idt::InterruptStackFrame, idt::InterruptDescriptorTable}
};
use pic8259::ChainedPics;
use crate::terminal::TERMINAL;


static KEYBOARD_IDT: LazyLock<InterruptDescriptorTable> = LazyLock::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    idt[33].set_handler_fn(interrup_keyboard);
    idt
});
static PICS: Mutex<ChainedPics> = Mutex::new(unsafe{ChainedPics::new(32, 40)});
static KEYBOARD: Mutex<PS2Keyboard<Us104Key, ScancodeSet1>> = Mutex::new(
    PS2Keyboard::new(ScancodeSet1::new(), Us104Key, HandleControl::Ignore));


pub fn init_keyboard() {
    KEYBOARD_IDT.load();
    
    unsafe {
        crate::keyboard::PICS.lock().initialize();
    }

    unsafe {
        let mut master_mask_port = Port::<u8>::new(0x21);
        let mut mask = master_mask_port.read();
        mask &= !(1 << 1); // ALLOW IRQ Mask Bit for keyboard interrupts
        master_mask_port.write(mask); 
    }
}


fn handle_keyboard_input(key: DecodedKey) {
    let mut t_guard = TERMINAL.lock();
    let Some(terminal) = t_guard.as_mut() else { return; };
    match key {
        DecodedKey::Unicode('\u{8}') => terminal.clear_char(),
        DecodedKey::Unicode(ch) => terminal.draw_char(ch),
        _ => ()
    }
}

extern "x86-interrupt" fn interrup_keyboard(
    _frame: InterruptStackFrame
) {
    let mut port = Port::<u8>::new(0x60);
    let scancode: u8 = unsafe{ port.read() };
    let mut keyboard = KEYBOARD.lock();

    if let Ok(Some(ev)) = keyboard.add_byte(scancode) {
        if ev.state == KeyState::Down {
            if let Some(key) = keyboard.process_keyevent(ev) {
                handle_keyboard_input(key);    
            }
        }
    }
    unsafe{
        PICS.lock().notify_end_of_interrupt(33 as u8)
    };
}
