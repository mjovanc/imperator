//! BSD 3-Clause License
//!
//! Copyright (c) 2024, Marcus Cvjeticanin
//!
//! Redistribution and use in source and binary forms, with or without
//! modification, are permitted provided that the following conditions are met:
//!
//! 1. Redistributions of source code must retain the above copyright notice, this
//!    list of conditions and the following disclaimer.
//!
//! 2. Redistributions in binary form must reproduce the above copyright notice,
//!    this list of conditions and the following disclaimer in the documentation
//!    and/or other materials provided with the distribution.
//!
//! 3. Neither the name of the copyright holder nor the names of its
//!    contributors may be used to endorse or promote products derived from
//!    this software without specific prior written permission.
//!
//! THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
//! AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
//! IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//! DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
//! FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
//! DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//! SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
//! CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
//! OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
//! OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::collections::HashMap;

use crate::{
    condition::Condition,
    sqlite::util::{generate_limit_str, generate_offset_str, generate_order_by_str},
};

use rusqlite::{Connection, Result};

use log::info;

use crate::table::Table;

pub fn delete<T: Table + Default>(conn: Connection) -> DeleteQueryBuilder<T> {
    DeleteQueryBuilder::new(conn)
}

pub struct DeleteQueryBuilder<T: Table + Default> {
    conn: Connection,
    table: Option<T>,
    where_condition: Option<Condition>,
    order_by: Option<HashMap<Vec<String>, String>>,
    limit: Option<usize>,
    offset: Option<usize>,
}

impl<T: Table + Default> DeleteQueryBuilder<T> {
    pub fn new(conn: Connection) -> Self {
        DeleteQueryBuilder {
            conn,
            table: None,
            where_condition: None,
            order_by: None,
            limit: None,
            offset: None,
        }
    }

    pub fn from(mut self, table: T) -> Self {
        self.table = Some(table);
        self
    }

    pub fn where_clause(mut self, condition: Condition) -> Self {
        self.where_condition = Some(condition);
        self
    }

    pub fn order_by(mut self, col_and_order: HashMap<Vec<String>, String>) -> Self {
        self.order_by = Some(col_and_order);
        self
    }

    pub fn limit(mut self, count: usize) -> Self {
        self.limit = Some(count);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn build(mut self) -> Result<(), String> {
        let table_name = self
            .table
            .as_ref()
            .map(|t| t.get_name().to_string())
            .unwrap_or("".to_string());

        // Sanitize table name from unwanted quotations or backslashes
        let table_name_str = table_name.replace("\"", "").replace("\\", "");

        let where_condition_str = if let Some(condition) = self.where_condition {
            format!("WHERE {}", condition.build())
        } else {
            String::new()
        };

        let order_by_str = generate_order_by_str(&self.order_by);
        let limit_str = generate_limit_str(self.limit);
        let offset_str = generate_offset_str(self.offset);

        // Construct the query based on defined variables above
        let query = format!(
            "DELETE FROM {} {} {} {}",
            table_name_str,
            where_condition_str,
            order_by_str,
            format!("{} {}", limit_str, offset_str),
        );

        info!("{}", query);
        println!("{}", query);

        // Execute SQL
        let _ = match self.conn.transaction() {
            Ok(tx) => tx.execute(&query.to_string(), []),
            Err(_) => todo!(),
        };

        Ok(())
    }
}
