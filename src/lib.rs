// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-printer-sharing: printer sharing component model.
//!
//! Real implementation: sharings, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sharing {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrinterSharingError {
    EmptySharingName,
    DuplicateSharing(String),
    EmptyEventSubject,
}

impl fmt::Display for PrinterSharingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySharingName => write!(f, "sharing name must not be empty"),
            Self::DuplicateSharing(id) => write!(f, "duplicate sharing {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for PrinterSharingError {}

pub struct PrinterSharing {
    pub name: String,
    sharings: BTreeMap<String, Sharing>,
    events: BTreeMap<String, Event>,
}

impl PrinterSharing {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            sharings: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_sharing(&mut self, item: Sharing) -> Result<(), PrinterSharingError> {
        if item.name.trim().is_empty() {
            return Err(PrinterSharingError::EmptySharingName);
        }
        let id = item.id.clone();
        if self.sharings.contains_key(&id) {
            return Err(PrinterSharingError::DuplicateSharing(id.clone()));
        }
        self.sharings.insert(id, item);
        Ok(())
    }

    pub fn sharing_count(&self) -> usize {
        self.sharings.len()
    }

    pub fn get_sharing(&self, id: &str) -> Option<&Sharing> {
        self.sharings.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), PrinterSharingError> {
        if event.subject.trim().is_empty() {
            return Err(PrinterSharingError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_sharing(id: &str, name: &str) -> Sharing {
        Sharing {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = PrinterSharing::new("PrinterSharing");
        assert_eq!(m.name, "PrinterSharing");
        assert_eq!(m.sharing_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_sharing() {
        let mut m = PrinterSharing::new("PrinterSharing");
        m.add_sharing(sample_sharing("a1", "Alpha")).unwrap();
        assert_eq!(m.sharing_count(), 1);
        assert!(m.get_sharing("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = PrinterSharing::new("PrinterSharing");
        assert!(m.add_sharing(sample_sharing("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = PrinterSharing::new("PrinterSharing");
        m.add_sharing(sample_sharing("x", "One")).unwrap();
        assert!(m.add_sharing(sample_sharing("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = PrinterSharing::new("PrinterSharing");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
