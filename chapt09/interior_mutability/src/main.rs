use std::cell::{/*Cell,*/ RefCell};

/*
use std::rc::Rc;
use std::cell::{Cell, RefCell};
pub struct SpiderRobot {
    species: String,
    web_enabled: bool,
    leg_devices: [fd::FileDesc; 8],
    hardware_error_count: Cell<u32>,
    log_file: RefCell<File>,
}

impl SpiderRobot {
    /// Write a line to the log file.
    pub fn log(&self, message: &str) {
        let mut file = self.log_file.borrow_mut();
        // `writeln!` is like `println!`, but sends
        // output to the given file.
        writeln!(file, "{}", message).unwrap();
    }
}


impl SpiderRobot {
    /// Increase the error count by 1.
    pub fn add_hardware_error(&self) {
        let n = self.hardware_error_count.get();
        self.hardware_error_count.set(n + 1);
    }

    /// True if any hardware errors have been reported.
    pub fn has_hardware_errors(&self) -> bool {
        self.hardware_error_count.get() > 0
    }
}


pub struct SpiderSenses {
    robot: Rc<SpiderRobot>, // <-- pointer to settings and I/O
    eyes: [Camera; 32],
    motion: Accelerometer,
}
*/


fn main() {
    let ref_cell: RefCell<String> = RefCell::new("hello".to_string());

    {
        let r = ref_cell.borrow(); // ok, returns a Ref<String>
        let count = r.len(); // ok, returns "hell".len();
        assert_eq!(count, 5);

    } // drop r, count

    let mut w = ref_cell.borrow_mut(); // panic: already borrowed
    w.push_str(" world");
}
