use crate::{
    ExprToken, Range, Statement, parser_ast::ParserTypeName, scope_manager::VariableKind,
    statement_building::BlockStmtBuilder,
};

impl BlockStmtBuilder<'_> {
    pub fn build_local_var(
        &mut self,
        name: &str,
        value_type: &Option<ParserTypeName>,
        def_val: &[ExprToken],
        stmt_range: Range,
    ) -> Option<Statement> {
        // Build and register the scope variable
        let var_id = self.build_and_register_scope_var(
            name,
            value_type,
            def_val,
            stmt_range,
            VariableKind::LocalVar,
        )?;

        // Return the local var statement
        let local_var_stmt = Statement::LocalVar { var_id };
        Some(local_var_stmt)
    }

    pub fn build_local_const(
        &mut self,
        name: &str,
        value_type: &Option<ParserTypeName>,
        def_val: &[ExprToken],
        stmt_range: Range,
    ) -> Option<Statement> {
        // Build and register the scope variable
        let var_id = self.build_and_register_scope_var(
            name,
            value_type,
            def_val,
            stmt_range,
            VariableKind::LocalConst,
        )?;

        // Return the local const statement
        let local_const_stmt = Statement::LocalConst { var_id };
        Some(local_const_stmt)
    }
}
