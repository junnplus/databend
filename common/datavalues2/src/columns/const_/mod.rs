// Copyright 2021 Datafuse Labs.
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

use std::sync::Arc;

use common_arrow::arrow::array::*;
use common_arrow::arrow::bitmap::Bitmap;

use crate::prelude::*;

#[derive(Clone)]
pub struct ConstColumn {
    length: usize,
    column: ColumnRef,
}

impl ConstColumn {
    pub fn new(column: ColumnRef, length: usize) -> Self {
        Self { column, length }
    }

    pub fn inner(&self) -> &ColumnRef {
        &self.column
    }
}

impl Column for ConstColumn {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn data_type(&self) -> DataTypePtr {
        self.column.data_type()
    }

    fn is_nullable(&self) -> bool {
        self.column.is_nullable()
    }

    fn len(&self) -> usize {
        self.length
    }

    fn null_at(&self, _row: usize) -> bool {
        self.column.null_at(0)
    }

    fn only_null(&self) -> bool {
        self.column.null_at(0)
    }

    fn is_const(&self) -> bool {
        true
    }

    fn validity(&self) -> (bool, Option<&Bitmap>) {
        if self.column.null_at(0) {
            (true, None)
        } else {
            (false, None)
        }
    }

    fn memory_size(&self) -> usize {
        self.column.memory_size()
    }

    fn as_arrow_array(&self) -> ArrayRef {
        let column = self.column.replicate(&[self.length]);
        column.as_arrow_array()
    }

    fn slice(&self, _offset: usize, length: usize) -> ColumnRef {
        Arc::new(Self {
            column: self.column.clone(),
            length,
        })
    }

    fn replicate(&self, offsets: &[usize]) -> ColumnRef {
        debug_assert!(
            offsets.len() == self.len(),
            "Size of offsets must match size of column"
        );

        Arc::new(Self::new(self.column.clone(), *offsets.last().unwrap()))
    }

    fn convert_full_column(&self) -> ColumnRef {
        self.column.replicate(&[self.length])
    }

    unsafe fn get_unchecked(&self, _index: usize) -> DataValue {
        self.column.get_unchecked(0)
    }
}