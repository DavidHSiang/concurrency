// metrics data structure
// 基本功能：increment, decrement, snapshot, clear

use core::fmt;
use std::sync::Arc;

use anyhow::Result;
use dashmap::DashMap;

#[derive(Debug, Clone)]
pub struct CmapMetrics {
    data: Arc<DashMap<String, i64>>,
}

impl CmapMetrics {
    pub fn new() -> Self {
        CmapMetrics {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn increment(&self, key: impl Into<String>) -> Result<()> {
        let mut count = self.data.entry(key.into()).or_insert(0);
        *count += 1;
        Ok(())
    }

    pub fn decrement(&self, key: impl Into<String>) -> Result<()> {
        let mut count = self.data.entry(key.into()).or_insert(0);
        *count -= 1;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<DashMap<String, i64>> {
        let res = (*self.data).clone();
        Ok(res)
    }

    pub fn clear(&mut self) -> Result<()> {
        self.data.clear();
        Ok(())
    }
}

impl fmt::Display for CmapMetrics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl Default for CmapMetrics {
    fn default() -> Self {
        Self::new()
    }
}
