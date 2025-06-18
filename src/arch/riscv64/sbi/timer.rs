use sbi_rt::SbiRet;
/// use sbi call to set timer
pub fn set_timer(timer: usize) {
    sbi_rt::set_timer(timer as _);
}
