use super::sbi_call;

const EID_CONSOLE_PUTCHAR: usize = 0x1;
const EID_SHUTDOWN: usize = 0x8;

/// put a character to console, sbi interface
pub fn console_putchar(c: usize) {
    sbi_call(EID_CONSOLE_PUTCHAR, 0, c, 0, 0);
}

/// shutdown the system, sbi interface
pub fn shutdown() {
    sbi_call(EID_SHUTDOWN, 0, 0, 0, 0);
}
