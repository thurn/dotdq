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

use data::delegate_data::PlayerTrickNumber;
use data::play_phase_data::{CompletedTrick, PlayPhaseData, PlayedCard};
use data::primitive::primitives::{Card, PlayerName};

use crate::play_phase::play_phase_queries;
use crate::rounds::tricks;

/// Plays the indicated [Card] from the hand identified by [PlayerName] if
/// it is currently legal to do so.
pub fn play_card(data: &mut PlayPhaseData, hand: PlayerName, card: Card) {
    data.hands.hand_mut(hand).remove(card);
    data.current_trick.cards.push(PlayedCard { played_by: hand, card });
    if data.current_trick.cards.len() == 4 {
        let trick = data.current_trick.clone();
        let winner = tricks::winner(data, &trick);
        data.completed_tricks.push(CompletedTrick { trick, winner });
        data.current_trick.cards.clear();
    }
    data.turn = next_to_play(data);
}

pub fn can_play(data: &PlayPhaseData, player: PlayerName, card: Card) -> bool {
    let follows_suit = if let Some(suit) = tricks::suit(&data.current_trick) {
        suit == card.suit() || play_phase_queries::suit_count(data, player, suit) == 0
    } else {
        true
    };

    let must_follow_suit = data.programs.current_delegates.must_follow_suit.run_query(
        data,
        &PlayerTrickNumber::new(player, tricks::current_number(data)),
        true,
    );

    data.is_turn(player)
        && data.hands.hand(player).contains(card)
        && (data.current_trick.cards.len() == 4 || follows_suit || !must_follow_suit)
}

/// Returns the [PlayerName] to next play a card during a round.
fn next_to_play(data: &PlayPhaseData) -> Option<PlayerName> {
    if data.hands.all_empty() {
        return None;
    }

    Some(match data.current_trick.cards.len() {
        0 => {
            if let Some(last) = data.completed_tricks.last() {
                last.winner
            } else {
                PlayerName::User
            }
        }
        1..=3 => data.current_trick.cards.last().unwrap().played_by.next(),
        _ => panic!("Invalid trick size"),
    })
}
