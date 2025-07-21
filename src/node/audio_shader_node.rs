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

use std::error::Error;

use crate::{Compiler, Parser, Program, SemanticAnalyzer, SymbolInfo, SymbolKind, SyntaxError};
use knodiq_engine::{Node, NodeId, Value, error::TrackError};

pub struct AudioShaderNode {
    id: NodeId,
    name: String,
    pub input: Vec<SymbolInfo>,
    pub output: Vec<SymbolInfo>,
    pub shader: String,
    pub program: Option<Program>,
}

impl AudioShaderNode {
    /// Creates a new AudioShaderNode instance.
    pub fn new() -> Self {
        AudioShaderNode {
            id: NodeId::new_v4(),
            name: "Audio Shader Node".to_string(),
            input: Vec::new(),
            output: Vec::new(),
            shader: "".to_string(),
            program: None,
        }
    }

    /// Sets the shader code for the node.
    pub fn set_shader(&mut self, shader: String) -> Result<(), Box<dyn Error>> {
        self.shader = shader;

        // Compile the shader code into a program.
        let parser = Parser::new();
        let program = match parser.parse(&self.shader) {
            Ok(program) => program,
            Err(_) => return Err(Box::new(SyntaxError::new())),
        };

        // Check for errors in the program.
        let mut analyzer = SemanticAnalyzer::new();
        match analyzer.analyze(&program) {
            Ok(program) => {
                self.program = Some(program);
                self.input = analyzer.get_inputs();
                self.output = analyzer.get_outputs();
                return Ok(());
            }
            Err(error) => return Err(Box::new(error)),
        }
    }

    /// Gets the shader code of the node.
    pub fn get_shader(&self) -> &str {
        &self.shader
    }
}

impl Node for AudioShaderNode {
    fn process(
        &mut self,
        _sample_rate: usize,
        _samples_per_beat: f32,
        _channels: usize,
        _chunk_start: usize,
        _chunk_end: usize,
        _track_id: u32,
    ) -> Result<(), Box<dyn TrackError>> {
        // let program = match self.program.as_ref() {
        //     Some(program) => program,
        //     None => return Ok(()),
        // };

        // let mut interpreter = Interpreter::new(
        //     program.clone(),
        //     sample_rate,
        //     samples_per_beat,
        //     channels,
        //     chunk_start,
        //     chunk_end,
        // );

        // let output_table = interpreter
        //     .execute(self.input.clone())
        //     .map_err(|e| Box::new(e) as Box<dyn TrackError>)?;
        // self.output = output_table;

        let mut input_vec = Vec::new();
        for symbol_info in &self.input {
            input_vec.push(match &symbol_info.value {
                Some(v) => v.clone(),
                None => return Ok(()),
            });
        }

        let mut compiler = match Compiler::new() {
            Ok(c) => c,
            Err(_) => return Ok(()),
        };

        let mut exec = match compiler.compile(&self.shader) {
            Ok(e) => e,
            Err(_) => return Ok(()),
        };

        let output = match exec.run(
            input_vec,
            self.output
                .iter()
                .map(|info| info.value_type.clone())
                .collect(),
        ) {
            Ok(r) => r,
            Err(_) => return Ok(()),
        };

        let output_iter = self.output.iter().enumerate();
        let mut new_output = Vec::new();
        for (i, symbol_info) in output_iter {
            if let Some(value) = output.get(i) {
                new_output.push(SymbolInfo {
                    value: Some(value.clone()),
                    ..symbol_info.clone()
                });
            } else {
                new_output.push(symbol_info.clone());
            }
        }
        self.output = new_output;

        Ok(())
    }

    fn get_input(&self, key: &str) -> Option<Value> {
        self.input.iter().find_map(|info| {
            if info.name == key {
                info.value.clone()
            } else {
                None
            }
        })
    }

    fn set_input(&mut self, key: &str, value: Value) {
        if let Some(info) = self.input.iter_mut().find(|info| info.name == key) {
            info.value = Some(value);
        } else {
            let value_type = value.get_type();
            self.input.push(SymbolInfo {
                name: key.to_string(),
                value: Some(value),
                value_type,
                kind: SymbolKind::Input,
            });
        }
    }

    fn get_input_list(&self) -> Vec<String> {
        self.input.iter().map(|info| info.name.clone()).collect()
    }

    fn get_output(&self, key: &str) -> Option<Value> {
        self.output.iter().find_map(|info| {
            if info.name == key {
                info.value.clone()
            } else {
                None
            }
        })
    }

    fn get_output_list(&self) -> Vec<String> {
        self.output.iter().map(|info| info.name.clone()).collect()
    }

    fn get_type(&self) -> String {
        "AudioShaderNode".to_string()
    }

    fn set_id(&mut self, id: NodeId) {
        self.id = id;
    }

    fn get_id(&self) -> NodeId {
        self.id.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn is_input(&self) -> bool {
        false
    }

    fn is_output(&self) -> bool {
        false
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Clone for AudioShaderNode {
    fn clone(&self) -> Self {
        AudioShaderNode {
            id: self.id.clone(),
            name: self.name.clone(),
            input: self.input.clone(),
            output: self.output.clone(),
            shader: self.shader.clone(),
            program: self.program.clone(),
        }
    }
}
