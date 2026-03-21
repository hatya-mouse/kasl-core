use crate::{
    Expr, ExprKind, backend::func_translator::FuncTranslator, namespace_registry::ArrayID,
};
use cranelift_codegen::ir::StackSlot;

impl FuncTranslator<'_> {
    pub fn store_array_to_slot(
        &mut self,
        expr: &Expr,
        array_id: &ArrayID,
        slot: StackSlot,
        base_offset: i32,
    ) {
        // Get the size of the items
        let array_decl = self
            .prog_ctx
            .type_registry
            .get_array_decl(array_id)
            .unwrap();
        let item_size = self
            .prog_ctx
            .type_registry
            .get_type_actual_size(array_decl.item_type())
            .unwrap() as i32;

        match &expr.kind {
            ExprKind::ArrayList(items) => {
                for (i, item) in (0i32..).zip(items.iter()) {
                    let offset = base_offset + item_size * i;
                    self.store_value_to_slot(item, slot, offset);
                }
            }
            ExprKind::ArraySpread { value, count } => {
                for i in 0..*count as i32 {
                    let offset = base_offset + item_size * i;
                    self.store_value_to_slot(value, slot, offset);
                }
            }
            _ => unreachable!(),
        }
    }
}
