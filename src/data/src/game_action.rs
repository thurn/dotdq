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

use crate::contract_phase_data::ContractPhaseAction;
use crate::play_phase_data::PlayPhaseAction;
use crate::widget_id::WidgetId;

#[derive(Debug, Clone, Copy)]
pub enum GameAction {
    SetHover(Option<WidgetId>),
    SetMouseDown(Option<WidgetId>),
    PlayPhaseAction(PlayPhaseAction),
    ContractPhaseAction(ContractPhaseAction),
}
