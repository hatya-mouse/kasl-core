/// # Safety
///
/// This function can cause an undefined behavior in these cases:
/// - Passing a null pointer as a program, inputs, outputs, or states.
/// - Passing a pointer with not enough length for the inputs, outputs or states.
pub unsafe fn run_once(
    program: *const u8,
    inputs: &[*const ()],
    outputs: &[*mut ()],
    states: &[*mut ()],
    should_init: i8,
) -> Result<(), String> {
    unsafe {
        let code_fn: fn(*const *const (), *const *mut (), *const *mut (), i8) =
            std::mem::transmute(program);
        code_fn(
            inputs.as_ptr(),
            outputs.as_ptr(),
            states.as_ptr(),
            should_init,
        );
    }

    Ok(())
}

/// # Safety
///
/// This function can cause an undefined behavior in these cases:
/// - Passing a null pointer as a program, inputs, outputs, or states.
/// - Passing a pointer with not enough length for the inputs, outputs or states.
pub unsafe fn run_buffer(
    program: *const u8,
    inputs: &[*const ()],
    outputs: &[*mut ()],
    states: &[*mut ()],
    should_init: i8,
    buffer_size: i32,
) -> Result<(), String> {
    unsafe {
        let code_fn: fn(*const *const (), *const *mut (), *const *mut (), i8, i32) =
            std::mem::transmute(program);
        code_fn(
            inputs.as_ptr(),
            outputs.as_ptr(),
            states.as_ptr(),
            should_init,
            buffer_size,
        );
    }

    Ok(())
}
