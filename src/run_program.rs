//
//  Copyright 2025-2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

/// Runs the given program once with the provided inputs, outputs, and states.
/// This function is unsafe because this function can cause an undefined behavior
/// if any of the pointers passed to the function are null, or if the lengths are incorrect.
///
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
) {
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
}

/// Runs the given program given times, with the provided inputs, outputs, and states.
/// This function is unsafe because this function can cause an undefined behavior
/// if any of the pointers passed to the function are null, or if the lengths are incorrect.
///
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
) {
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
}
