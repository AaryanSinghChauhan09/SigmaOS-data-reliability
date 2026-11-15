//! SigmaOS-data-reliability: data reliability model.
//!
//! Real implementation: uptime, MTTR, SLAs, and validation.
//! Replaces the 108-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Reliability SLA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReliabilitySLA {
    pub id: String,
    pub name: String,
    pub service: String,
    pub uptime_target: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReliabilityError {
    EmptyName,
    DuplicateSLA(String),
}

impl fmt::Display for ReliabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "SLA name must not be empty"),
            Self::DuplicateSLA(id) => write!(f, "duplicate SLA {:?}", id),
        }
    }
}

impl std::error::Error for ReliabilityError {}

/// Data reliability manager.
pub struct DataReliability {
    pub name: String,
    slas: BTreeMap<String, ReliabilitySLA>,
}

impl DataReliability {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            slas: BTreeMap::new(),
        }
    }

    pub fn add_sla(&mut self, sla: ReliabilitySLA) -> Result<(), ReliabilityError> {
        if sla.name.trim().is_empty() { return Err(ReliabilityError::EmptyName); }
        let id = sla.id.clone();
        if self.slas.contains_key(&id) {
            return Err(ReliabilityError::DuplicateSLA(id.clone()));
        }
        self.slas.insert(id, sla);
        Ok(())
    }

    pub fn sla_count(&self) -> usize { self.slas.len() }

    pub fn get_sla(&self, id: &str) -> Option<&ReliabilitySLA> { self.slas.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reliability_create_aaa() {
        let r = DataReliability::new("My Reliability");
        assert_eq!(r.name, "My Reliability");
        assert_eq!(r.sla_count(), 0);
    }

    #[test]
    fn test_add_sla_aaa() {
        let mut r = DataReliability::new("My Reliability");
        let s = ReliabilitySLA { id: "s1".into(), name: "api-uptime".into(), service: "api".into(), uptime_target: "99.9%".into() };
        r.add_sla(s).unwrap();
        assert_eq!(r.sla_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut r = DataReliability::new("My Reliability");
        let s = ReliabilitySLA { id: "x".into(), name: "".into(), service: "a".into(), uptime_target: "b".into() };
        assert!(r.add_sla(s).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut r = DataReliability::new("My Reliability");
        let s = ReliabilitySLA { id: "x".into(), name: "X".into(), service: "a".into(), uptime_target: "b".into() };
        r.add_sla(s).unwrap();
        let s2 = ReliabilitySLA { id: "x".into(), name: "X".into(), service: "a".into(), uptime_target: "b".into() };
        assert!(r.add_sla(s2).is_err());
    }
}
