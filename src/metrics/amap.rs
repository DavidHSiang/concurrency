use anyhow::Result;
use std::{
    collections::HashMap,
    fmt::Display,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
};

#[derive(Debug, Clone)]
pub struct AmapMetrics {
    pub data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AmapMetrics {
    pub fn new(metric_names: &[&'static str]) -> Self {
        let data: Arc<HashMap<&str, AtomicI64>> = Arc::new(
            metric_names
                .iter()
                .map(|&name| (name, AtomicI64::new(0)))
                .collect(),
        );
        Self { data }
    }

    pub fn increment(&self, key: impl AsRef<str>) -> Result<()> {
        let counter = self
            .data
            .get(key.as_ref())
            .ok_or_else(|| anyhow::anyhow!("key not found"))?;
        counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    // pub fn get(&self, key: &'static str) -> i64 {
    //     let data = self.data.as_ref();
    //     let counter = data.get(key).unwrap();
    //     counter.load(std::sync::atomic::Ordering::Relaxed)
    // }
}

impl Display for AmapMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.data.as_ref();
        let mut s = String::new();
        for (key, value) in data.iter() {
            s.push_str(&format!("{}: {}, ", key, value.load(Ordering::Relaxed)));
        }
        write!(f, "{}", s)
    }
}
