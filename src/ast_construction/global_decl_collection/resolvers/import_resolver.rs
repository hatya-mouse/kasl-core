//
// © 2025-2026 Shuntaro Kasatani
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

use crate::{
    Range, compilation_data::CompilerState, error::Ph, global_decl_collection::GlobalDeclCollector,
    namespace_constructor::NameSpaceConstructor, namespace_registry::ImportPath,
};
use std::{fs::File, io::Read, path::PathBuf};

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
        if let Some(last_component) = import_path.path.last()
            && self
                .prog_ctx
                .namespace_registry
                .is_name_used(&self.current_namespace, last_component)
        {
            self.ec
                .duplicate_name(decl_range, Ph::GlobalDeclCollection, last_component);
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

        // Add the imported path to the set of paths to search for imports
        let mut imported_paths = self.comp_state.imported_paths.clone();
        imported_paths.insert(full_path.clone());

        // Get the parent directory of the imported path and add it to the child search paths
        let mut child_search_paths = self.comp_state.child_search_paths.clone();
        if let Some(parent_path) = full_path.parent() {
            child_search_paths.push(parent_path.to_path_buf());
        }

        // Create a new compiler state
        let comp_state = CompilerState {
            child_search_paths,
            imported_paths,
        };

        // Generate a new namespace for the imported program
        let namespace_id = self.prog_ctx.namespace_registry.register_namespace(
            import_path.path.last().cloned().unwrap(),
            Some(self.current_namespace),
        );
        self.prog_ctx
            .scope_registry
            .create_global_scope(namespace_id);

        // Create a constructor and pass the program to it
        println!("Program {}: {}", import_path, &program);
        let mut constructor = NameSpaceConstructor::new(
            self.ec,
            self.prog_ctx,
            self.comp_data,
            comp_state,
            self.builtin_registry,
            namespace_id,
        );
        constructor.set_code(&program);

        // Construct the program
        constructor.collect_global_decls();
    }

    fn search_progam(&mut self, import_path: &ImportPath) -> Option<(String, PathBuf)> {
        for base_path in &self.comp_state.child_search_paths {
            // Create a full path by joining the base path with the import path
            let full_path = base_path.join(import_path.to_path()).with_extension("kasl");

            if full_path.is_file() {
                // Open the file
                let mut file = match File::open(&full_path) {
                    Err(why) => panic!("couldn't open {}: {}", full_path.display(), why),
                    Ok(file) => file,
                };

                // Get the content string of the file
                let mut str = String::new();
                match file.read_to_string(&mut str) {
                    Err(why) => panic!("couldn't read {}: {}", full_path.display(), why),
                    Ok(_) => return Some((str, full_path)),
                }
            }
        }

        None
    }
}
