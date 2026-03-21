use crate::{
    FuncParam, ParserScopeStmt, PostfixOperator, PostfixOperatorProperties, Range,
    error::{EK, Ph},
    global_decl_collection::GlobalDeclCollector,
    symbol_table::Block,
    type_registry::ResolvedType,
};

impl GlobalDeclCollector<'_> {
    pub fn resolve_postfix_define(&mut self, symbol: &str, props: &PostfixOperatorProperties) {
        if let Err(error_kind) = self
            .prog_ctx
            .op_ctx
            .register_postfix_properties(symbol.to_string(), props.clone())
            && error_kind == EK::DuplicatePostfixDefine
        {
            self.ec
                .duplicate_postfix_define(props.range, Ph::GlobalDeclCollection, symbol);
        }
    }

    pub fn register_postfix_func(
        &mut self,
        symbol: &str,
        params: Vec<FuncParam>,
        return_type: ResolvedType,
        body: &[ParserScopeStmt],
        block: Block,
        decl_range: Range,
    ) {
        if params.len() != 1 {
            self.ec.wrong_param_count_for_postfix(
                decl_range,
                Ph::GlobalDeclCollection,
                symbol,
                params.len(),
            );
            return;
        }

        // Construct postfix operator
        let op = PostfixOperator {
            symbol: symbol.to_string(),
            namespace_id: self.current_namespace,
            operand: params[0].clone(),
            return_type,
            block,
            range: decl_range,
        };

        // Register the operator
        let Ok(op_id) = self.prog_ctx.op_ctx.register_postfix_func(op) else {
            self.ec.duplicate_postfix_func(
                decl_range,
                Ph::GlobalDeclCollection,
                symbol,
                self.prog_ctx
                    .type_registry
                    .format_type(&params[0].value_type),
            );
            return;
        };

        // Register the function body to the function body map
        self.comp_data.op_body_map.register(op_id, body.to_vec());
    }
}
