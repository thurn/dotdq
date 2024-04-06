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
use data::primitives::{Card, HandIdentifier, PlayerName, Suit};
use itertools::Itertools;

/// Returns true if the indicated [PlayPhaseAction] is currently legal to take
pub fn can_perform_action(data: &PlayPhaseData, action: PlayPhaseAction) -> bool {
    match action {
        PlayPhaseAction::PlayCard(player, hand, card) => can_play_card(data, player, hand, card),
    }
}

/// Returns an iterator over actions the indicated `player` can take in the
/// current game state.
pub fn legal_actions(
    data: &PlayPhaseData,
    player: PlayerName,
) -> impl Iterator<Item = PlayPhaseAction> + '_ {
    player
        .owned_hands()
        .flat_map(move |hand| {
            data.hand(hand).sorted().map(move |card| PlayPhaseAction::PlayCard(player, hand, card))
        })
        .filter(|&action| can_perform_action(data, action))
}

/// Returns the [PlayerName] whose turn to act it currently is
pub fn current_turn(data: &PlayPhaseData) -> PlayerName {
    next_to_play(data).owner()
}

/// Returns the [HandIdentifier] to next play a card during a round.
pub fn next_to_play(data: &PlayPhaseData) -> HandIdentifier {
    match data.current_trick.cards.len() {
        0 => {
            if let Some(last) = data.completed_tricks.last() {
                trick_winner(&last.trick)
            } else {
                data.contract.declarer.primary_hand()
            }
        }
        1..=3 => data.current_trick.cards.last().unwrap().played_by.next(),
        4 => trick_winner(&data.current_trick),
        _ => panic!("Invalid trick size"),
    }
}

/// Returns the [HandIdentifier] which won a given trick.
///
/// Panics if the provided trick is not completed.
fn trick_winner(trick: &Trick) -> HandIdentifier {
    let suit = trick_suit(trick).expect("Trick was empty");
    trick
        .cards
        .iter()
        .filter(|c| c.card.suit == suit)
        .max_by_key(|c| c.card)
        .expect("Trick was empty")
        .played_by
}

/// Returns the [Suit] being used for the provided trick, or None if the trick
/// is empty.
fn trick_suit(trick: &Trick) -> Option<Suit> {
    Some(trick.cards.first()?.card.suit)
}

/// Returns the number of cards of the given [Suit] in the indicated hand.
fn suit_count(data: &PlayPhaseData, hand: HandIdentifier, suit: Suit) -> usize {
    data.hand(hand).filter(|card| card.suit == suit).count()
}

fn can_play_card(
    data: &PlayPhaseData,
    player: PlayerName,
    hand: HandIdentifier,
    card: Card,
) -> bool {
    let follows_suit = if let Some(suit) = trick_suit(&data.current_trick) {
        suit == card.suit || suit_count(data, hand, suit) == 0
    } else {
        true
    };

    next_to_play(data) == hand
        && player.owns_hand(hand)
        && data.hands.get(&hand).unwrap().contains(&card)
        && follows_suit
}
