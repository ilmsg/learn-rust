use anyhow::Result;
use async_trait::async_trait;
use erp_core::{AppContext, ErpModule, EventHook, HookRegistry};
use serde_json::Value;

pub struct InventoryModule;

impl ErpModule for InventoryModule {
    fn name(&self) -> &'static str {
        "inventory"
    }

    fn register(&self, registry: &mut HookRegistry) {
        registry.subscribe("sale.order.confirmed", DeductStockHook);
    }
}

struct DeductStockHook;

#[async_trait]
impl EventHook for DeductStockHook {
    async fn handle(&self, payload: &Value, ctx: &AppContext) -> Result<()> {
        let order_id = payload["order_id]"].as_str().unwrap_or("UNKNOWN");
        let product = payload["product"].as_str().unwrap_or("UNKNOWN");
        let qty = payload["qty"].as_i64().unwrap_or(0);

        println!(
            "  - > [Inventory Hook] ตัดสต๊อกสินค้า '{}' จำนวน {} ชิ้น (อ้างอิงใบสั่งขาย) {}",
            product, qty, order_id
        );

        Ok(())
    }
}
