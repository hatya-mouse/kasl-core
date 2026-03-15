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
    Range, error::Ph, global_decl_collection::GlobalDeclCollector,
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

        // If the path is already in the set of imported paths, skip it and throw an error
        if self.constructor_state.imported_paths.contains(&full_path) {
            self.ec.cyclic_dependency(
                decl_range,
                Ph::GlobalDeclCollection,
                import_path.to_string(),
            );
            return;
        }

        // Add the imported path to the set of paths to search for imports
        let mut imported_paths = self.constructor_state.imported_paths.clone();
        imported_paths.insert(full_path);

        // Create a constructor and pass the program to it
        let mut constructor = NameSpaceConstructor::new(self.comp_config.clone(), imported_paths);
        constructor.set_code(&program);

        // Construct the program
        constructor.collect_global_decls();
        constructor.analyze_struct_graph();
        constructor.build_statements();
        constructor.analyze_scope_graph();

        // Register the namespace with the import path
        self.namespace.namespace_registry.register_namespace(
            import_path.path.last().cloned().unwrap(),
            constructor.namespace,
        );
    }

    fn search_progam(&mut self, import_path: &ImportPath) -> Option<(String, PathBuf)> {
        for base_path in &self.comp_config.search_paths {
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
