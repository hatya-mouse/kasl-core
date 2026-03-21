use crate::{
    ParserDeclStmt, ParserDeclStmtKind,
    error::Ph,
    global_decl_collection::{FuncDeclInfo, GlobalDeclCollector},
    symbol_table::FunctionType,
};

impl<'a> GlobalDeclCollector<'a> {
    pub fn process_stmt(&mut self, stmt: &'a ParserDeclStmt) {
        match &stmt.kind {
            ParserDeclStmtKind::Import { path } => self.resolve_import(path, stmt.range),
            ParserDeclStmtKind::Typealias { alias, target } => {
                self.resolve_typealias(alias, target, stmt.range)
            }
            ParserDeclStmtKind::Input {
                name,
                value_type,
                def_val,
                attrs,
            } => self.resolve_input(name, value_type, def_val, attrs, stmt.range),
            ParserDeclStmtKind::Output {
                name,
                value_type,
                def_val,
            } => self.resolve_output(name, value_type, def_val, stmt.range),
            ParserDeclStmtKind::StateVar {
                name,
                value_type,
                def_val,
            } => self.resolve_state_var(name, value_type, def_val, stmt.range),
            ParserDeclStmtKind::GlobalConst {
                name,
                value_type,
                def_val,
            } => self.resolve_global_const(name, value_type, def_val, stmt.range),

            ParserDeclStmtKind::StructDecl { name, body } => {
                self.resolve_struct_decl(name, body, stmt.range)
            }

            ParserDeclStmtKind::FuncDecl {
                is_static,
                name,
                params,
                return_type,
                body,
            } => {
                // Throw an error if the function is static
                if *is_static {
                    self.ec.global_func_cannot_be_static(
                        stmt.range,
                        Ph::GlobalDeclCollection,
                        name,
                    );
                }

                let info = FuncDeclInfo {
                    func_type: FunctionType::Global,
                    name,
                    params,
                    return_type,
                    body,
                };
                self.resolve_global_func_decl(info, stmt.range)
            }

            ParserDeclStmtKind::InfixDefine { symbol, props } => {
                self.resolve_infix_define(symbol, props)
            }
            ParserDeclStmtKind::PrefixDefine { symbol, props } => {
                self.resolve_prefix_define(symbol, props)
            }
            ParserDeclStmtKind::PostfixDefine { symbol, props } => {
                self.resolve_postfix_define(symbol, props)
            }

            ParserDeclStmtKind::OperatorFunc {
                op_type,
                symbol,
                params,
                return_type,
                body,
            } => self.resolve_operator_func(op_type, symbol, params, return_type, body, stmt.range),

            ParserDeclStmtKind::StructField { .. } => {
                self.ec.top_level_var(stmt.range, Ph::GlobalDeclCollection)
            }
        }
    }
}
