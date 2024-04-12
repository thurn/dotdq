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

use data::play_phase_data::{PlayPhaseAction, PlayPhaseData, Trick};
use data::primitives::{Card, PlayerName, Suit};

/// Returns true if the indicated [PlayPhaseAction] is currently legal to take
pub fn can_perform_action(
    data: &PlayPhaseData,
    player: PlayerName,
    action: PlayPhaseAction,
) -> bool {
    match action {
        PlayPhaseAction::PlayCard(card) => can_play_card(data, player, card),
    }
}

/// Returns an iterator over actions the indicated `player` can take in the
/// current game state.
pub fn legal_actions(
    data: &PlayPhaseData,
    player: PlayerName,
) -> impl Iterator<Item = PlayPhaseAction> + '_ {
    data.hand(player)
        .iter()
        .map(PlayPhaseAction::PlayCard)
        .filter(move |&action| can_perform_action(data, player, action))
}

/// Returns the [PlayerName] whose turn to act it currently is, or None if
/// the game has ended
pub fn current_turn(data: &PlayPhaseData) -> Option<PlayerName> {
    if data.all_hands_empty() {
        None
    } else {
        Some(next_to_play(data))
    }
}

/// Returns the [PlayerName] to next play a card during a round.
pub fn next_to_play(data: &PlayPhaseData) -> PlayerName {
    match data.current_trick.cards.len() {
        0 => {
            if let Some(last) = data.completed_tricks.last() {
                trick_winner(&last.trick)
            } else {
                PlayerName::User
            }
        }
        1..=3 => data.current_trick.cards.last().unwrap().played_by.next(),
        4 => trick_winner(&data.current_trick),
        _ => panic!("Invalid trick size"),
    }
}

/// Returns the number of tricks the [PlayerName] player has won in the
/// provided game so far.
pub fn tricks_won(data: &PlayPhaseData, player: PlayerName) -> usize {
    data.completed_tricks.iter().filter(|t| t.winner == player).count()
}

/// Returns true if the [PlayerName] player completed their assigned contract
pub fn met_contract(data: &PlayPhaseData, player: PlayerName) -> bool {
    data.contract.contract_number(player) == tricks_won(data, player)
}

/// Returns the [PlayerName] which won a given trick.
///
/// Panics if the provided trick is not completed.
pub fn trick_winner(trick: &Trick) -> PlayerName {
    let suit = trick_suit(trick).expect("Trick was empty");
    trick
        .cards
        .iter()
        .filter(|c| c.card.suit() == suit)
        .max_by_key(|c| c.card)
        .expect("Trick was empty")
        .played_by
}

/// Returns the [Suit] being used for the provided trick, or None if the trick
/// is empty.
fn trick_suit(trick: &Trick) -> Option<Suit> {
    Some(trick.cards.first()?.card.suit())
}

/// Returns the number of cards of the given [Suit] in the indicated hand.
fn suit_count(data: &PlayPhaseData, hand: PlayerName, suit: Suit) -> usize {
    data.hand(hand).iter().filter(|card| card.suit() == suit).count()
}

fn can_play_card(data: &PlayPhaseData, player: PlayerName, card: Card) -> bool {
    let follows_suit = if let Some(suit) = trick_suit(&data.current_trick) {
        suit == card.suit() || suit_count(data, player, suit) == 0
    } else {
        true
    };

    next_to_play(data) == player
        && data.hand(player).contains(card)
        && (data.current_trick.cards.len() == 4 || follows_suit)
}
