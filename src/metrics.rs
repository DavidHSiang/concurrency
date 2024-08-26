// metrics data structure
// 基本功能：increment, decrement, snapshot, clear

use core::fmt;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<RwLock<HashMap<String, i64>>>,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn increment(&self, key: impl Into<String>) -> Result<()> {
        let mut binding = self
            .data
            .write()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let count = binding.entry(key.into()).or_insert(0);
        *count += 1;
        Ok(())
    }

    pub fn decrement(&self, key: impl Into<String>) -> Result<()> {
        let mut binding = self
            .data
            .write()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let count = binding.entry(key.into()).or_insert(0);
        *count -= 1;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        self.data
            .read()
            .map(|data| data.clone())
            .map_err(|e| anyhow::anyhow!(e.to_string()))
    }

    pub fn clear(&mut self) -> Result<()> {
        self.data
            .write()
            .map(|mut data| data.clear())
            .map_err(|e| anyhow::anyhow!(e.to_string()))
    }
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data = self.data.read().map_err(|_| fmt::Error {})?;
        write!(f, "{:?}", data)
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_metrics() -> Result<(), anyhow::Error> {
        let metrics = super::Metrics::new();
        metrics.increment("foo")?;
        metrics.increment("foo")?;
        metrics.increment("bar")?;
        metrics.increment("foo")?;
        metrics.decrement("foo")?;
        metrics.increment("baz")?;
        assert_eq!(metrics.snapshot()?, {
            let mut map = std::collections::HashMap::new();
            map.insert("foo".to_string(), 2);
            map.insert("bar".to_string(), 1);
            map.insert("baz".to_string(), 1);
            map
        });
        Ok(())
    }
}
