use lazy_static::lazy_static;
use x86_64::{
    VirtAddr,
    instructions::tables::load_tss,
    registers::segmentation::{CS, Segment},
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        tss::TaskStateSegment,
    },
};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
const STACK_SIZE: usize = 4096 * 5;

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut global_descriptor_table = GlobalDescriptorTable::new();
        let code_selector = global_descriptor_table.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = global_descriptor_table.add_entry(Descriptor::tss_segment(&TSS));

        (
            global_descriptor_table,
            Selectors {
                code_selector,
                tss_selector,
            },
        )
    };
    static ref TSS: TaskStateSegment = {
        let mut task_state_segment = TaskStateSegment::new();
        task_state_segment.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            let stack_start = VirtAddr::from_ptr(&raw const STACK);

            stack_start + STACK_SIZE
        };

        task_state_segment
    };
}

pub fn init() {
    GDT.0.load();

    // reload code segment register and load TSS
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}
