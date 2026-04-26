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

use crate::{
    DIR_ROOT_FILE_NAME,
    ast_nodes::{Range, compilation_data::CompilerState, namespace_registry::ImportPath},
    error::{ErrorCollector, Ph},
    semantic_analysis::{
        decl_collection::GlobalDeclCollector, namespace_constructor::NameSpaceConstructor,
    },
};
use std::{
    collections::HashSet,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

impl GlobalDeclCollector<'_> {
    pub fn resolve_import(&mut self, import_path: &ImportPath, decl_range: Range) {
        let Some((program, full_path)) = self.search_progam(import_path) else {
            self.ec.import_not_found(
                decl_range,
                Ph::GlobalDeclCollection,
                import_path.to_string(),
            );
            return;
        };

        // If the name is already used, throw an error
        if let Some(last_component) = import_path.path.last() {
            if self.is_name_used(last_component) {
                self.ec
                    .duplicate_name(decl_range, Ph::GlobalDeclCollection, last_component);
            } else {
                // Mark the module name as used
                self.mark_name_used(last_component);
            }
        }

        // If the path is already in the set of imported paths, skip it and throw an error
        if self.comp_state.imported_paths.contains(&full_path) {
            self.ec.cyclic_dependency(
                decl_range,
                Ph::GlobalDeclCollection,
                import_path.to_string(),
            );
            return;
        }

        // Change the ranges of the errors to the import path range and insert them into the error collector
        let child_ec = self.compile_imported_program(program, full_path, import_path);

        for mut error in child_ec.records.values().cloned() {
            error.ranges = HashSet::from([decl_range]);
            self.ec.push_error(error);
        }
    }

    fn compile_imported_program(
        &mut self,
        program: String,
        full_path: PathBuf,
        import_path: &ImportPath,
    ) -> ErrorCollector {
        // Add the imported path to the set of paths to search for imports
        let mut imported_paths = self.comp_state.imported_paths.clone();
        imported_paths.insert(full_path.clone());

        // Get the parent directory of the imported path and add it to the child search paths
        let mut child_search_paths = self.comp_state.child_search_paths.clone();
        if let Some(parent_path) = full_path.parent() {
            child_search_paths.push(parent_path.to_path_buf());
        }

        // Generate a new namespace for the imported program
        let namespace_id = self.prog_ctx.namespace_registry.register_namespace(
            import_path.path.last().cloned().unwrap(),
            Some(self.current_namespace),
        );
        self.prog_ctx
            .scope_registry
            .create_global_scope(namespace_id);

        // Create a new error collector for the imported program
        let mut ec = ErrorCollector::default();

        // Create a new compiler state
        let comp_state = CompilerState {
            child_search_paths,
            imported_paths,
            virtual_files: self.comp_state.virtual_files.clone(),
        };

        // Create a constructor and pass the program to it
        let mut constructor = NameSpaceConstructor::new(
            &mut ec,
            self.prog_ctx,
            self.comp_data,
            comp_state,
            self.builtin_registry,
            namespace_id,
        );
        constructor.set_code(&program);

        // Parse and construct the program
        if let Err(parse_error) = constructor.parse() {
            let offset = parse_error.location.offset;
            self.ec.parse_error(
                Range::n(offset, offset),
                Ph::GlobalDeclCollection,
                parse_error.expected,
            );
            return ec;
        }
        constructor.collect_global_decls();

        ec
    }

    fn search_progam(&mut self, import_path: &ImportPath) -> Option<(String, PathBuf)> {
        for base_path in &self.comp_state.child_search_paths {
            let file_path = base_path.join(import_path.to_path()).with_extension("kasl");

            if let Some(content) = self.get_file_content(&file_path) {
                return Some((content, file_path));
            } else {
                // If the file with .kasl extension is not found, try to find a directory with the same name and look for a root.kasl file in it
                let root_path = base_path
                    .join(import_path.to_path())
                    .with_file_name(DIR_ROOT_FILE_NAME);
                if let Some(content) = self.get_file_content(&root_path) {
                    return Some((content, root_path));
                }
            }
        }
        None
    }

    fn get_file_content(&self, path: &Path) -> Option<String> {
        if path.is_file() {
            // Open the file
            let mut file = match File::open(path) {
                Err(why) => panic!("couldn't open {}: {}", path.display(), why),
                Ok(file) => file,
            };

            // Get the content string of the file
            let mut str = String::new();
            match file.read_to_string(&mut str) {
                Err(why) => panic!("couldn't read {}: {}", path.display(), why),
                Ok(_) => Some(str),
            }
        } else {
            self.comp_state.virtual_files.get(path).cloned()
        }
    }
}
