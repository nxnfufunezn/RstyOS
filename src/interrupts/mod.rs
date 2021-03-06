#[macro_use]
mod idt;

use vga;
use x86::{irq, segmentation};
use pic;
use keyboard::{Keyboard, STATE};
use cpuio::Port;
use core::intrinsics;

lazy_static! {
	static ref IDT: idt::Idt = {
		let mut idt = idt::Idt::new();
        idt.set_handler(0, make_idt_entry!(isr0, {
            unsafe { vga::print_error(format_args!("EXCEPTION: Divide By Zero")) };
            loop { } 
        }));

        idt.set_handler(1, make_idt_entry!(isr1, {
            unsafe { vga::print_error(format_args!("EXCEPTION: Debug")) };
            loop { } 
        }));

        idt.set_handler(2, make_idt_entry!(isr2, {
            unsafe { vga::print_error(format_args!("EXCEPTION: Non-maskable Interrupt")) };
            loop { } 
        }));

        idt.set_handler(3, make_idt_entry!(isr3, {
            unsafe { vga::print_error(format_args!("EXCEPTION: Breakpoint")) };
            loop { } 
        }));

        idt.set_handler(4, make_idt_entry!(isr4, {
            unsafe { vga::print_error(format_args!("EXCEPTION: Overflow")) };
            loop { } 
        }));

        idt.set_handler(5, make_idt_entry!(isr5, {
            unsafe { vga::print_error(format_args!("EXCEPTION: Bound Range Exceeded")) };
            loop { } 
        }));

        idt.set_handler(6, make_idt_entry!(isr6, {
            unsafe { vga::print_error(format_args!("EXCEPTION: Invalid Opcode")) };
            loop { } 
        }));

        idt.set_handler(7, make_idt_entry!(isr7, {
            unsafe { vga::print_error(format_args!("EXCEPTION: Device Not Available")) };
            loop { } 
        }));

        idt.set_handler(8, make_idt_entry!(isr8, {
            unsafe { vga::print_error(format_args!("EXCEPTION: Double Fault")) };
            loop { } 
        }));

        idt.set_handler(9, make_idt_entry!(isr9, {
            // do nothing for now
            pic::eoi_for(9);
            unsafe { irq::enable(); }
        }));

        idt.set_handler(10, make_idt_entry!(isr10, {
            unsafe { vga::print_error(format_args!("EXCEPTION: Invalid TSS")) };
            loop { } 
        }));

        idt.set_handler(11, make_idt_entry!(isr11, {
            unsafe { vga::print_error(format_args!("EXCEPTION: Segment Not Present")) };
            loop { } 
        }));

        idt.set_handler(12, make_idt_entry!(isr12, {
            unsafe { vga::print_error(format_args!("EXCEPTION: Stack-Segment Fault")) };
            loop { } 
        }));

        idt.set_handler(13, make_idt_entry!(isr13, {
            unsafe { vga::print_error(format_args!("EXCEPTION: General Protection Fault")) };
            loop { } 
        }));

        idt.set_handler(14, make_idt_entry!(isr14, {
            unsafe { vga::print_error(format_args!("EXCEPTION: PAGE FAULT")) };
            loop { } 
        }));

        idt.set_handler(15, make_idt_entry!(isr15, {
            // do nothing for now
            pic::eoi_for(15);
            unsafe { irq::enable(); } 
        }));

        idt.set_handler(16, make_idt_entry!(isr16, {
            unsafe { vga::print_error(format_args!("EXCEPTION: x87 Floating-Point Exception")) };
            loop { } 
        }));

        idt.set_handler(17, make_idt_entry!(isr17, {
            unsafe { vga::print_error(format_args!("EXCEPTION: Alignment Check")) };
            loop { } 
        }));

        idt.set_handler(18, make_idt_entry!(isr18, {
           unsafe { vga::print_error(format_args!("EXCEPTION: Machine Check")) };
            loop { } 
        }));

        idt.set_handler(19, make_idt_entry!(isr19, {
            unsafe { vga::print_error(format_args!("EXCEPTION: SIMD Floating-Point Exception")) };
            loop { } 
        }));

        idt.set_handler(20, make_idt_entry!(isr20, {
            unsafe { vga::print_error(format_args!("EXCEPTION: Virtualization Exception")) };
            loop { } 
        }));

        idt.set_handler(21, make_idt_entry!(isr21, {
            // do nothing for now
            pic::eoi_for(21);
            unsafe { irq::enable(); } 
        }));

        idt.set_handler(22, make_idt_entry!(isr22, {
            // do nothing for now
            pic::eoi_for(22);
            unsafe { irq::enable(); } 
        }));

        idt.set_handler(23, make_idt_entry!(isr23, {
            // do nothing for now
            pic::eoi_for(23);
            unsafe { irq::enable(); } 
        }));

        idt.set_handler(24, make_idt_entry!(isr24, {
            // do nothing for now
            pic::eoi_for(24);
            unsafe { irq::enable(); } 
        }));

        idt.set_handler(25, make_idt_entry!(isr25, {
            // do nothing for now
            pic::eoi_for(25);
            unsafe { irq::enable(); } 
        }));

        idt.set_handler(26, make_idt_entry!(isr26, {
            // do nothing for now
            pic::eoi_for(26);
            unsafe { irq::enable(); } 
        }));

        idt.set_handler(27, make_idt_entry!(isr27, {
            // do nothing for now
            pic::eoi_for(27);
            unsafe { irq::enable(); } 
        }));

        idt.set_handler(28, make_idt_entry!(isr28, {
            // do nothing for now
            pic::eoi_for(28);
            unsafe { irq::enable(); } 
        }));

        idt.set_handler(29, make_idt_entry!(isr29, {
            // do nothing for now
            pic::eoi_for(29);
            unsafe { irq::enable(); } 
        }));

        idt.set_handler(30, make_idt_entry!(isr30, {
           unsafe { vga::print_error(format_args!("EXCEPTION: Security Exception")) };
            loop { } 
        }));

        idt.set_handler(31, make_idt_entry!(isr31, {
            // do nothing for now
            pic::eoi_for(31);
            unsafe { irq::enable(); } 
        }));

        idt.set_handler(32, make_idt_entry!(isr32, {
            // timer, do nothing for now
            pic::eoi_for(32);
            unsafe { irq::enable(); } 
        }));

        idt.set_handler(33, make_idt_entry!(isr33, {
            let mut keyboard: Port<u8> = unsafe { Port::new(0x60) };
			let scancode = keyboard.read();
			STATE.lock().update_state(scancode);
            Keyboard.handle_keys(scancode as usize);
            pic::eoi_for(33);
            unsafe { irq::enable(); } 
        }));

        idt
    };
}

pub fn init() {
	IDT.load();
}