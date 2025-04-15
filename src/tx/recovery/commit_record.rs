use std::{any::Any, sync::Arc};

use bincode::serialize;
use serde::{Deserialize, Serialize};

use crate::{buffer::buffer_mgr::BufferMgr, error::DbResult, storage::page::Page};

use super::log_record::{LogRecord, COMMIT_FLAG};

/// A commit transaction log record.
#[derive(Serialize, Deserialize)]
pub struct CommitRecord {
    tx_num: i32,
}

impl CommitRecord {
    pub fn new(page: &Page) -> Self {
        CommitRecord {
            tx_num: page.get_int(4),
        }
    }

    pub fn create(tx_num: i32) -> Self {
        CommitRecord { tx_num }
    }
    
    pub fn to_bytes(&self) -> DbResult<Vec<u8>> {
        let mut result = vec![COMMIT_FLAG as u8];
        result.extend(serialize(self)?);
        Ok(result)
    }
}

impl LogRecord for CommitRecord {
    fn op(&self) -> i32 {
        COMMIT_FLAG
    }

    fn tx_number(&self) -> i32 {
        self.tx_num
    }

    fn undo(&self, _tx_num: i32, _buffer_mgr: &Arc<BufferMgr>) -> DbResult<()> {
        // Commit records don't need to be undone
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tx::recovery::log_record::{create_log_record, LogRecord};

    #[test]
    fn test_commit_record_serialization() -> crate::error::DbResult<()> {
        let record = CommitRecord::create(123);
        let bytes = record.to_bytes()?;
        
        let deserialized = create_log_record(&bytes)?;
        
        assert_eq!(deserialized.op(), COMMIT_FLAG);
        assert_eq!(deserialized.tx_number(), 123);
        
        let commit = (&*deserialized).as_any().downcast_ref::<CommitRecord>()
            .expect("Failed to downcast to CommitRecord");
        assert_eq!(commit.tx_num, 123);
        Ok(())
    }
}