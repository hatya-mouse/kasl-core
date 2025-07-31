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
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref ARRAY_STORAGE: Mutex<HashMap<usize, Vec<u8>>> = Mutex::new(HashMap::new());
}

pub struct Executable {
    pub func: unsafe extern "C" fn(*const u8, *mut u8),
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
            input_vals.extend_from_slice(&get_bytes_repr(input));
        }

        // Execute the function
        unsafe {
            (self.func)(input_vals.as_ptr(), self.outputs.as_mut_ptr());
        }

        // Convert results to Value
        // let result = output_types
        //     .iter()
        //     .fold(Vec::new(), |mut acc, target_type| {
        //         let (remaining_bytes, value) = bytes_as_value(&self.outputs, &target_type);
        //         self.outputs = remaining_bytes;
        //         acc.push(value);
        //         acc
        //     });
        let result = vec![];

        Ok(result)
    }
}

fn get_bytes_repr(value: &Value) -> Vec<u8> {
    match value {
        Value::Array(arr) => {
            println!("🎯 [RUNTIME] Creating input array");
            println!("🎯 [RUNTIME] Input: {:?}", arr);

            let mut array_data = Vec::new();
            let length = arr.len() as i32;
            array_data.extend_from_slice(&length.to_ne_bytes());

            println!(
                "🎯 [RUNTIME] Length: {} -> bytes: {:?}",
                length,
                length.to_ne_bytes()
            );

            for (i, elem) in arr.iter().enumerate() {
                match elem {
                    Value::Float(f) => {
                        let f_bytes = f.to_ne_bytes();
                        println!("🎯 [RUNTIME] Float[{}]: {} -> bytes: {:?}", i, f, f_bytes);
                        array_data.extend_from_slice(&f_bytes);
                    }
                    Value::Int(int_val) => {
                        array_data.extend_from_slice(&int_val.to_ne_bytes());
                    }
                    Value::Array(_) => {
                        array_data.extend_from_slice(&get_bytes_repr(elem));
                    }
                }
            }

            println!("🎯 [RUNTIME] Total array data: {:?}", array_data);
            println!("🎯 [RUNTIME] Data structure:");
            println!("🎯   Length (4 bytes): {:?}", &array_data[0..4]);
            if array_data.len() > 4 {
                println!("🎯   Element 0 (4 bytes): {:?}", &array_data[4..8]);
            }
            if array_data.len() > 8 {
                println!("🎯   Element 1 (4 bytes): {:?}", &array_data[8..12]);
            }

            let data_ptr = Box::into_raw(array_data.into_boxed_slice()) as *const u8;
            let ptr_value = data_ptr as usize;

            println!("🎯 [RUNTIME] Array pointer: 0x{:x}", ptr_value);
            ptr_value.to_ne_bytes().to_vec()
        }
        Value::Float(f) => f.to_ne_bytes().to_vec(),
        Value::Int(i) => i.to_ne_bytes().to_vec(),
    }
}

fn bytes_as_value(bytes: &Vec<u8>, target_type: &Type) -> (Vec<u8>, Value) {
    println!("🎯 [RUNTIME] Bytes: {:?}", bytes);
    match target_type {
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
                let len_ptr = ptr as *const i32;
                let len_val = *len_ptr;

                let raw_memory = std::slice::from_raw_parts(ptr, 16);
                println!("🎯 [RUNTIME] Raw memory: {:?}", raw_memory);

                len_val
            } as usize;

            if len == 0 {
                println!("❌ [RUNTIME] Zero length!");
                return (bytes[usize_bytes..].to_vec(), Value::Array(Vec::new()));
            }

            let data_ptr = unsafe { ptr.add(4) };
            let element_size = 4; // float
            let total_data_size = len * element_size;

            println!("🎯 [RUNTIME] Data pointer: 0x{:x}", data_ptr as usize);
            println!("🎯 [RUNTIME] Reading {} bytes", total_data_size);

            let data = unsafe { std::slice::from_raw_parts(data_ptr, total_data_size) };
            println!("🎯 [RUNTIME] Data bytes: {:?}", data);

            let mut vec = Vec::new();
            for (i, chunk) in data.chunks(4).enumerate() {
                if chunk.len() == 4 {
                    let mut arr = [0u8; 4];
                    arr.copy_from_slice(chunk);
                    let f = f32::from_ne_bytes(arr);

                    println!(
                        "🎯 [RUNTIME] Element[{}]: {} (from bytes: {:?})",
                        i, f, chunk
                    );

                    if f.is_nan() {
                        println!("❌ [RUNTIME] NaN detected at element {}!", i);
                        println!("❌ [RUNTIME] Bytes that produced NaN: {:?}", chunk);

                        // バイトパターンを解析
                        let bits = u32::from_ne_bytes(arr);
                        println!("❌ [RUNTIME] Bit pattern: 0x{:08x}", bits);

                        if bits == 0x7fc00000 {
                            println!("❌ [RUNTIME] This is a standard quiet NaN");
                        } else if (bits & 0x7f800000) == 0x7f800000 {
                            println!("❌ [RUNTIME] This is some form of NaN/Infinity");
                        } else {
                            println!("❌ [RUNTIME] Unexpected bit pattern for float");
                        }
                    }

                    vec.push(Value::Float(f));
                }
            }

            println!("🎯 [RUNTIME] Final result: {:?}", vec);
            (bytes[usize_bytes..].to_vec(), Value::Array(vec))
        }
        // 他の型...
        Type::Float => {
            if bytes.len() < 4 {
                panic!("Not enough bytes for float");
            }
            let mut arr = [0u8; 4];
            arr.copy_from_slice(&bytes[0..4]);
            (bytes[4..].to_vec(), Value::Float(f32::from_ne_bytes(arr)))
        }
        Type::Int => {
            if bytes.len() < 4 {
                panic!("Not enough bytes for int");
            }
            let mut arr = [0u8; 4];
            arr.copy_from_slice(&bytes[0..4]);
            (bytes[4..].to_vec(), Value::Int(i32::from_ne_bytes(arr)))
        }
        Type::None => {
            panic!("Unsupported type: {:?}", target_type);
        }
    }
}
