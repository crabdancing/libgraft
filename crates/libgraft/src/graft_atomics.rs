use std::sync::atomic::AtomicBool;

pub trait GraftAtomicBool {
    fn get_relaxed(&self) -> bool;
    fn get_cst(&self) -> bool;
    fn set_relaxed(&self, value: bool);
    fn set_cst(&self, value: bool);
}

impl GraftAtomicBool for AtomicBool {
    fn get_relaxed(&self) -> bool {
        self.load(std::sync::atomic::Ordering::Relaxed)
    }

    fn get_cst(&self) -> bool {
        self.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn set_relaxed(&self, value: bool) {
        self.store(value, std::sync::atomic::Ordering::Relaxed);
    }

    fn set_cst(&self, value: bool) {
        self.store(value, std::sync::atomic::Ordering::SeqCst);
    }
}
