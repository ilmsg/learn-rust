use anyhow::Result;
use erp_core::{ErpApp, ErpModule, HookRegistry};
use serde_json::json;

pub struct SalesModule;

impl ErpModule for SalesModule {
    fn name(&self) -> &'static str {
        "sales"
    }

    fn register(&self, registry: &mut HookRegistry) {
        // โมดูลนี้เป็นผู้ส่ง Event จึงไม่ต้อง Subscribe อะไร
    }
}

pub struct SaleService;

impl SaleService {
    pub async fn confirm_order(
        app: &ErpApp,
        order_id: &str,
        product: &str,
        qty: i64,
    ) -> Result<()> {
        println!("[Sales] ยืนยันใบสั่งขายเลขที่: {}", order_id);

        app.trigger(
            "sale.order.confirmed",
            json!({
                "order_id": order_id,
                "product": product,
                "qty": qty
            }),
        )
        .await?;

        Ok(())
    }
}
