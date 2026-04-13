// Copyright 2025 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An implementation of `deadpool::managed::Manager` for `rusqlite`
//! for usage in WASM environments.
//!
//! Similar to the one implemented in `crate::connection::default`,
//! we do not implement connection recycling here. Mostly due to
//! [`managed::Manager::recycle`] method expecting a future with `Send`
//! bound which is not available in WASM environment.

use std::path::PathBuf;

use deadpool::managed::{self, Metrics};
use rusqlite::OpenFlags;

use crate::utils::SyncOutsideWasmWrapper;

/// [`Manager`][managed::Manager] for creating and recycling SQLite
/// [`Connection`]s.
#[derive(Debug)]
pub struct Manager {
    database_path: PathBuf,

    /// VFS used by this database connection in WASM environment.
    vfs: String,
}

impl Manager {
    /// Creates a new [`Manager`] for a database.
    #[must_use]
    pub fn new(database_path: PathBuf, vfs: String) -> Self {
        Self { database_path, vfs }
    }
}

impl managed::Manager for Manager {
    type Type = SyncOutsideWasmWrapper<rusqlite::Connection>;
    type Error = rusqlite::Error;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        let path = self.database_path.clone();

        let conn = rusqlite::Connection::open_with_flags_and_vfs(
            path,
            OpenFlags::default(),
            self.vfs.as_str(),
        )?;
        Ok(SyncOutsideWasmWrapper::new(conn))
    }

    async fn recycle(
        &self,
        _conn: &mut Self::Type,
        _: &Metrics,
    ) -> managed::RecycleResult<Self::Error> {
        // We cannot implement connection recycling
        // at the moment, due to
        // `managed::Manager::recycle` expecting
        // a future with `Send` bound which is not
        // available in WASM environments.
        Ok(())
    }
}
