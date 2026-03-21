use crate::{
    FuncParam, InfixOperator, InfixOperatorProperties, ParserScopeStmt, Range,
    error::{EK, Ph},
    global_decl_collection::GlobalDeclCollector,
    symbol_table::Block,
    type_registry::ResolvedType,
};

impl GlobalDeclCollector<'_> {
    pub fn resolve_infix_define(&mut self, symbol: &str, props: &InfixOperatorProperties) {
        if let Err(error_kind) = self
            .prog_ctx
            .op_ctx
            .register_infix_properties(symbol.to_string(), props.clone())
            && error_kind == EK::DuplicateInfixDefine
        {
            self.ec
                .duplicate_infix_define(props.range, Ph::GlobalDeclCollection, symbol);
        }
    }

    pub fn register_infix_func(
        &mut self,
        symbol: &str,
        params: Vec<FuncParam>,
        return_type: ResolvedType,
        body: &[ParserScopeStmt],
        block: Block,
        decl_range: Range,
    ) {
        if params.len() != 2 {
            self.ec.wrong_param_count_for_infix(
                decl_range,
                Ph::GlobalDeclCollection,
                symbol,
                params.len(),
            );
            return;
        }

        // Construct infix operator
        let op = InfixOperator {
            symbol: symbol.to_string(),
            namespace_id: self.current_namespace,
            lhs: params[0].clone(),
            rhs: params[1].clone(),
            return_type,
            block,
            range: decl_range,
        };

        // Register the operator
        let Ok(op_id) = self.prog_ctx.op_ctx.register_infix_func(op) else {
            self.ec.duplicate_infix_func(
                decl_range,
                Ph::GlobalDeclCollection,
                symbol,
                self.prog_ctx
                    .type_registry
                    .format_type(&params[0].value_type),
                self.prog_ctx
                    .type_registry
                    .format_type(&params[1].value_type),
            );
            return;
        };

        // Register the function body to the function body map
        self.comp_data.op_body_map.register(op_id, body.to_vec());
    }
}
