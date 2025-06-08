use std::{collections::HashMap, error::Error};

use crate::{Interpreter, Lexer, Parser, Program, SemanticAnalyzer, SymbolInfo};
use knodiq_engine::{Node, Value};

pub struct AudioShaderNode {
    input: HashMap<String, SymbolInfo>,
    output: HashMap<String, SymbolInfo>,
    shader: String,
    program: Option<Program>,
}

impl AudioShaderNode {
    /// Creates a new AudioShaderNode instance.
    pub fn new() -> Self {
        AudioShaderNode {
            input: HashMap::new(),
            output: HashMap::new(),
            shader: "".to_string(),
            program: None,
        }
    }

    /// Sets the shader code for the node.
    pub fn set_shader(&mut self, shader: String) -> Result<(), Vec<String>> {
        self.shader = shader;

        // Compile the shader code into a program.
        let lexer = Lexer::new(self.shader.clone());
        let tokens = lexer.tokenize();
        let parser = Parser::new(tokens);
        let program = match parser.parse() {
            Ok(program) => program,
            Err(err) => return Err(vec![err.to_string()]),
        };

        // Check for errors in the program.
        let mut analyzer = SemanticAnalyzer::new();
        match analyzer.analyze(&program) {
            Ok(_) => {
                self.program = Some(program);
                self.input = analyzer.get_input_table();
                self.output = analyzer.get_output_table();
                Ok(())
            }
            Err(errors) => Err(errors),
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
        sample_rate: usize,
        channels: usize,
        chunk_start: usize,
        chunk_end: usize,
    ) -> Result<(), Box<dyn Error>> {
        let program = self.program.as_ref().unwrap();
        let mut interpreter = Interpreter::new(
            program.clone(),
            sample_rate,
            channels,
            chunk_start,
            chunk_end,
        );
        let output_table = interpreter.execute(self.input.clone())?;
        self.output = output_table;

        println!("{:?}", self.get_output("out_buffer"));

        Ok(())
    }

    fn prepare(&mut self, _chunk_size: usize) {}

    fn get_input(&self, key: &str) -> Option<Value> {
        self.input.get(key).and_then(|info| info.value.clone())
    }

    fn set_input(&mut self, key: &str, value: Value) {
        if self.input.contains_key(key) {
            self.input.get_mut(key).unwrap().value = Some(value);
        }
    }

    fn get_input_list(&self) -> Vec<String> {
        self.input.keys().cloned().collect()
    }

    fn get_output(&self, key: &str) -> Option<Value> {
        self.output.get(key).and_then(|info| info.value.clone())
    }

    fn get_output_list(&self) -> Vec<String> {
        self.output.keys().cloned().collect()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Clone for AudioShaderNode {
    fn clone(&self) -> Self {
        AudioShaderNode {
            input: self.input.clone(),
            output: self.output.clone(),
            shader: self.shader.clone(),
            program: self.program.clone(),
        }
    }
}
