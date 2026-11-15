use anyhow::Result;
use erp_core::ErpApp;

#[tokio::main]
async fn main() -> Result<()> {
    let app = ErpApp::new("sqlite://erp_poc.db".to_string());

    #[cfg(feature = "mod_inventory")]
    app.install_module(erp_inventory::InventoryModule);

    #[cfg(feature = "mod_sales")]
    app.install_module(erp_sales::SalesModule);

    println!("--- เริ่มจำลองการทำงาน ---");

    #[cfg(feature = "mod_sales")]
    {
        erp_sales::SaleService::confirm_order(&app, "SO-001", "Laptop Pro", 2).await?;
    }

    Ok(())
}
