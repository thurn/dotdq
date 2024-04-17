// Copyright © Dungeon of the Diamond Queen 2024-present
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ProgramName {
    Redstar,
}

impl Display for ProgramName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = format!("{:?}", self);
        result.make_ascii_uppercase();
        assert!(result.len() <= 8, "Program name {result} cannot exceed 8 bytes");
        write!(f, "{}", result)
    }
}
