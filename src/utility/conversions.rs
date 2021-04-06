use std::convert::TryInto;

/// Converts `num` to `i32`, clamping it to `i32`'s max value if `num` is too big
pub fn as_i32(num: usize) -> i32 {
    match num.try_into() {
        Ok(result) => result,
        Err(_) => i32::MAX
    }
}

/// Converts `num` to `usize`, clamping it between 0 and `usize::MAX` if `num` is outside range
pub fn as_usize(num: i32) -> usize {
    match num.try_into() {
        Ok(result) => result,
        Err(_) => {
            if num < 0 {
                0
            } else {
                usize::MAX
            }
        }
    }
}

/// Converts `num` to `usize`, clamping it between 0 and `u16;:MAX`
pub fn u16_to_usize(num: u16) -> usize {
    match num.try_into() {
        Ok(result) => result,
        Err(_) => u16::MAX.into()
    }
}
