use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct AppContext {
    pub db_url: String,
}

#[async_trait]
pub trait ErpModule: Send + Sync {
    fn name(&self) -> &'static str;
    fn register(&self, registry: &mut HookRegistry);
}

#[async_trait]
pub trait EventHook: Send + Sync {
    async fn handle(&self, payload: &Value, ctx: &AppContext) -> Result<()>;
}

#[derive(Default)]
pub struct HookRegistry {
    hooks: HashMap<String, Vec<Arc<dyn EventHook>>>,
}

impl HookRegistry {
    pub fn new() -> Self {
        Self {
            hooks: HashMap::new(),
        }
    }

    pub fn subscribe<H: EventHook + 'static>(&mut self, event_name: &str, hook: H) {
        self.hooks
            .entry(event_name.to_string())
            .or_insert_with(Vec::new)
            .push(Arc::new(hook));
    }

    pub async fn emit(&self, event_name: &str, payload: Value, ctx: &AppContext) -> Result<()> {
        if let Some(listeners) = self.hooks.get(event_name) {
            for hook in listeners {
                hook.handle(&payload, ctx).await?;
            }
        }
        Ok(())
    }
}

pub struct ErpApp {
    pub context: Arc<AppContext>,
    pub registry: Arc<RwLock<HookRegistry>>,
}

impl ErpApp {
    pub fn new(db_url: String) -> Self {
        Self {
            context: Arc::new(AppContext { db_url }),
            registry: Arc::new(RwLock::new(HookRegistry::new())),
        }
    }

    pub fn install_module<M: ErpModule>(&self, module: M) {
        println!("[Core] กำลังติดตั้งโมดูล: {}", module.name());
        let mut reg = self.registry.write().unwrap();
        module.register(&mut reg);
    }

    pub async fn trigger(&self, event_name: &str, payload: Value) -> Result<()> {
        let listeners = {
            let reg = self.registry.read().unwrap();
            reg.hooks.get(event_name).cloned().unwrap_or_default()
        };

        for hook in listeners {
            hook.handle(&payload, &self.context).await?;
        }

        Ok(())
    }
}
