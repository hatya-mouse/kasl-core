use crate::{
    FuncParam, ParserScopeStmt, PrefixOperator, PrefixOperatorProperties, Range,
    error::{EK, Ph},
    global_decl_collection::GlobalDeclCollector,
    symbol_table::Block,
    type_registry::ResolvedType,
};

impl GlobalDeclCollector<'_> {
    pub fn resolve_prefix_define(&mut self, symbol: &str, props: &PrefixOperatorProperties) {
        if let Err(error_kind) = self
            .prog_ctx
            .op_ctx
            .register_prefix_properties(symbol.to_string(), props.clone())
            && error_kind == EK::DuplicatePrefixDefine
        {
            self.ec
                .duplicate_prefix_define(props.range, Ph::GlobalDeclCollection, symbol);
        }
    }

    pub fn register_prefix_func(
        &mut self,
        symbol: &str,
        params: Vec<FuncParam>,
        return_type: ResolvedType,
        body: &[ParserScopeStmt],
        block: Block,
        decl_range: Range,
    ) {
        if params.len() != 1 {
            self.ec.wrong_param_count_for_prefix(
                decl_range,
                Ph::GlobalDeclCollection,
                symbol,
                params.len(),
            );
            return;
        }

        // Construct prefix operator
        let op = PrefixOperator {
            symbol: symbol.to_string(),
            namespace_id: self.current_namespace,
            operand: params[0].clone(),
            return_type,
            block,
            range: decl_range,
        };

        // Register the operator
        let Ok(op_id) = self.prog_ctx.op_ctx.register_prefix_func(op) else {
            self.ec.duplicate_prefix_func(
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
