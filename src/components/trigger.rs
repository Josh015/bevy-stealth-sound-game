use crate::common::recurrence::Recurrence;

/// Entity that player can collide with to trigger an in-game effect.
pub struct Trigger {
    number_of_uses: Recurrence,
    // TODO: How to assign effect? Maybe custom commands?
}

// TODO: Can trigger all sorts of world effects (eg. exit the level, display
// announcement text in UI along the top, display story messages in the UI along
// the bottom, open a door, give the player a powerup, etc).
