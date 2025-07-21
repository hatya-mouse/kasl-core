//
// Copyright 2025 Shuntaro Kasatani
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use knodiq_engine::{Type, Value};

pub struct Executable {
    pub func: unsafe extern "C" fn(*const u8, *mut u8) -> (),
    pub outputs: Vec<u8>,
    pub output_types: Vec<Type>,
}

impl Executable {
    pub fn run(
        &mut self,
        inputs: Vec<Value>,
        output_types: Vec<Type>,
    ) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        let mut input_vals = Vec::new();
        for input in &inputs {
            get_bytes_repr(input, &mut input_vals);
        }

        // Execute the function
        unsafe {
            (self.func)(input_vals.as_ptr(), self.outputs.as_mut_ptr());
        }

        // Convert results to Value
        let result = output_types
            .iter()
            .fold(Vec::new(), |mut acc, target_type| {
                let (remaining_bytes, value) = bytes_as_value(&self.outputs, &target_type);
                self.outputs = remaining_bytes;
                acc.push(value);
                acc
            });

        Ok(result)
    }
}

fn get_bytes_repr(value: &Value, output: &mut Vec<u8>) {
    match value {
        Value::Float(f) => output.extend_from_slice(&f.to_ne_bytes()),
        Value::Int(i) => output.extend_from_slice(&i.to_ne_bytes()),
        Value::Array(arr) => {
            let ptr = arr.as_ptr() as usize;
            output.extend_from_slice(&ptr.to_ne_bytes());
        }
    }
}

fn bytes_as_value(bytes: &Vec<u8>, target_type: &Type) -> (Vec<u8>, Value) {
    println!(
        "bytes_as_value: {:?}, target_type: {:?}",
        bytes, target_type
    );
    match target_type {
        Type::Float => {
            if bytes.len() < 4 {
                panic!("Not enough bytes for float");
            }
            let mut arr = [0u8; 4];
            arr.copy_from_slice(&bytes[0..4]);
            (bytes[4..].to_vec(), Value::Float(f32::from_ne_bytes(arr)))
        }
        Type::Int => {
            if bytes.len() < 8 {
                panic!("Not enough bytes for int");
            }
            let mut arr = [0u8; 8];
            arr.copy_from_slice(&bytes[0..8]);
            (
                bytes[8..].to_vec(),
                Value::Int(i64::from_ne_bytes(arr) as i32),
            )
        }
        Type::Array(inside_type) => {
            let usize_bytes = std::mem::size_of::<usize>();

            if bytes.len() < usize_bytes {
                panic!("Not enough bytes for array pointer");
            }

            let mut ptr_bytes = vec![0u8; usize_bytes];
            ptr_bytes.copy_from_slice(&bytes[0..usize_bytes]);
            let ptr = usize::from_ne_bytes(ptr_bytes.try_into().unwrap()) as *const u8;

            if ptr.is_null() {
                return (bytes[usize_bytes..].to_vec(), Value::Array(Vec::new()));
            }

            let len = unsafe {
                let len_ptr = ptr as *const usize;
                *len_ptr
            };

            if len == 0 {
                return (bytes[usize_bytes..].to_vec(), Value::Array(Vec::new()));
            }

            let data_ptr = unsafe { ptr.add(usize_bytes) };
            let element_size = match &**inside_type {
                Type::Float => 4,
                Type::Int => 8,
                Type::Array(_) => usize_bytes,
                Type::None => {
                    panic!("Unsupported type for array: {:?}", inside_type);
                }
            };

            let total_data_size = len * element_size;
            let data = unsafe { std::slice::from_raw_parts(data_ptr, total_data_size) };

            let mut vec = Vec::new();

            match &**inside_type {
                Type::Float => {
                    for chunk in data.chunks(4) {
                        if chunk.len() == 4 {
                            let mut arr = [0u8; 4];
                            arr.copy_from_slice(chunk);
                            vec.push(Value::Float(f32::from_ne_bytes(arr)));
                        }
                    }
                }
                Type::Int => {
                    for chunk in data.chunks(8) {
                        if chunk.len() == 8 {
                            let mut arr = [0u8; 8];
                            arr.copy_from_slice(chunk);
                            vec.push(Value::Int(i64::from_ne_bytes(arr) as i32));
                        }
                    }
                }
                Type::Array(_) => {
                    for chunk in data.chunks(usize_bytes) {
                        if chunk.len() == usize_bytes {
                            let (_, arr_val) = bytes_as_value(&chunk.to_vec(), inside_type);
                            vec.push(arr_val);
                        }
                    }
                }
                _ => {
                    panic!("Unsupported array element type: {:?}", inside_type);
                }
            }

            (bytes[usize_bytes..].to_vec(), Value::Array(vec))
        }
        Type::None => {
            panic!("Unsupported type: {:?}", target_type);
        }
    }
}
