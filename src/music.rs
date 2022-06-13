use crate::music_player::*;

fn intro_pat_1(player : &Player) {
    // Pattern 1
    player.play_note(Value::Quarter, Note::F, 3, true);
    player.play_note(Value::Eighth, Note::F, 3, false);

    player.play_note(Value::Quarter, Note::G,3, true);
    player.play_note(Value::Eighth, Note::G,3, false);

    player.play_note(Value::Quarter, Note::C,3, false);
}

fn intro_pat_2(player : &Player) {
    // Pattern 2
    player.play_note(Value::Quarter, Note::G,3, true);
    player.play_note(Value::Eighth, Note::G,3, false);

    player.play_note(Value::Quarter, Note::A,3, true);
    player.play_note(Value::Eighth, Note::A,3, false);

    player.play_note(Value::Sixteenth, Note::C,4, false);

    player.play_note(Value::Sixteenth, Note::Bb,3, false);

    player.play_note(Value::Eighth, Note::A,3, false);
}

fn intro_pat_3(player : &Player) {
    // Pattern 3
    player.play_note(Value::Quarter, Note::C,3, true);
    player.play_note(Value::Eighth, Note::C,3, false);

    player.play_pause(Value::Quarter);

    player.play_note(Value::Sixteenth, Note::C,3, false);

    player.play_note(Value::Sixteenth, Note::C,3, false);

    player.play_note(Value::Sixteenth, Note::E,3, false);

    player.play_note(Value::Eighth, Note::F,3, false);

    player.play_note(Value::Sixteenth, Note::F,3, true);
}

fn intro_pat_4(player : &Player) {
    // Pattern 4
    player.play_note(Value::Quarter, Note::E, 3, false);

    player.play_note(Value::Quarter, Note::F, 3, false);

    player.play_pause(Value::Quarter);

    player.play_note(Value::Sixteenth, Note::F, 3, false);
    player.play_note(Value::Eighth, Note::F, 3, false);
    player.play_note(Value::Sixteenth, Note::F, 3, false);
}

fn introduction(player : &Player) {
    intro_pat_1(player);
    intro_pat_2(player);
    intro_pat_1(player);
    intro_pat_3(player);
    intro_pat_1(player);
    intro_pat_2(player);
    intro_pat_1(player);
    intro_pat_4(player);
}

fn verse(player : &Player, first : bool) {
    // Pattern 5
    player.play_pause(Value::Quarter);

    player.play_note(Value::Eighth, Note::D, 3, false);

    player.play_note(Value::Eighth, Note::E, 3, false);

    player.play_note(Value::Eighth, Note::F, 3, false);

    player.play_note(Value::Eighth, Note::F, 3, false);

    player.play_note(Value::Eighth, Note::G, 3, false);

    player.play_note(Value::Eighth, Note::E, 3, false);

    // Pattern 6
    player.play_note(Value::Sixteenth, Note::E, 3, false);

    player.play_note(Value::Sixteenth, Note::D, 3, false);

    player.play_note(Value::Eighth, Note::C, 3, true);
    player.play_note(Value::Half, Note::C, 3, false);

    player.play_pause(Value::Quarter);

    // Pattern 7
    player.play_pause(Value::Eighth);

    player.play_note(Value::Eighth, Note::D, 3, false);
    player.play_note(Value::Eighth, Note::D, 3, false);

    player.play_note(Value::Eighth, Note::E, 3, false);

    player.play_note(Value::Eighth, Note::F, 3, false);

    player.play_note(Value::Quarter, Note::D, 3, false);

    player.play_note(Value::Eighth, Note::C, 3, false);

    if first {
        // Pattern 8
        player.play_note(Value::Eighth, Note::C, 4, false);

        player.play_pause(Value::Eighth);

        player.play_note(Value::Eighth, Note::C, 4, false);

        player.play_note(Value::Half, Note::G, 3, false);

        player.play_pause(Value::Eighth);
    } else {
        // Pattern 23
        player.play_note(Value::Eighth, Note::C, 4, false);

        player.play_pause(Value::Eighth);

        player.play_note(Value::Eighth, Note::C, 4, false);

        player.play_note(Value::Quarter, Note::G, 3, false);

        player.play_note(Value::Eighth, Note::A, 3, false);

        player.play_note(Value::Eighth, Note::G, 3, false);

        player.play_note(Value::Eighth, Note::F, 3, false);
    }

    // Pattern 9
    player.play_pause(Value::Eighth);

    player.play_note(Value::Eighth, Note::D, 3, false);
    player.play_note(Value::Eighth, Note::D, 3, false);

    player.play_note(Value::Eighth, Note::E, 3, false);

    player.play_note(Value::Eighth, Note::F, 3, false);

    player.play_note(Value::Eighth, Note::D, 3, false);

    player.play_note(Value::Eighth, Note::F, 3, false);

    player.play_note(Value::Eighth, Note::G, 3, false);

    // Pattern 10
    player.play_pause(Value::Eighth);

    player.play_note(Value::Eighth, Note::E, 3, false);

    player.play_note(Value::Eighth, Note::D, 3, false);

    player.play_note(Value::Eighth, Note::C, 3, true);
    player.play_note(Value::Quarter, Note::C, 3, false);

    player.play_pause(Value::Quarter);

    // Pattern 11
    player.play_pause(Value::Eighth);

    player.play_note(Value::Eighth, Note::D, 3, false);

    player.play_note(Value::Eighth, Note::D, 3, false);

    player.play_note(Value::Eighth, Note::E, 3, false);

    player.play_note(Value::Eighth, Note::F, 3, false);

    player.play_note(Value::Eighth, Note::D, 3, false);

    player.play_note(Value::Quarter, Note::C, 3, false);

    // Pattern 12
    player.play_note(Value::Eighth, Note::G, 3, false);

    player.play_note(Value::Eighth, Note::G, 3, false);

    player.play_note(Value::Eighth, Note::G, 3, false);

    player.play_note(Value::Eighth, Note::A, 3, false);

    player.play_note(Value::Quarter, Note::G, 3, false);

    player.play_pause(Value::Quarter);

    // Pattern 13
    player.play_note(Value::Half, Note::F, 3, true);
    player.play_note(Value::Eighth, Note::F, 3, false);

    player.play_note(Value::Eighth, Note::G, 3, false);

    player.play_note(Value::Eighth, Note::A, 3, false);

    player.play_note(Value::Eighth, Note::F, 3, false);

    // Pattern 14
    player.play_note(Value::Eighth, Note::G, 3, false);

    player.play_note(Value::Eighth, Note::G, 3, false);

    player.play_note(Value::Eighth, Note::G, 3, false);

    player.play_note(Value::Eighth, Note::A, 3, false);

    player.play_note(Value::Quarter, Note::G, 3, false);

    player.play_note(Value::Quarter, Note::C, 3, false);

    // Pattern 15
    player.play_pause(Value::Half);

    player.play_note(Value::Eighth, Note::D, 3, false);

    player.play_note(Value::Eighth, Note::E, 3, false);

    player.play_note(Value::Eighth, Note::F, 3, false);

    player.play_note(Value::Eighth, Note::D, 3, false);

    // Pattern 16
    player.play_pause(Value::Eighth);

    player.play_note(Value::Eighth, Note::G, 3, false);

    player.play_note(Value::Eighth, Note::A, 3, false);

    player.play_note(Value::Eighth, Note::G, 3, true);
    player.play_note(Value::Quarter, Note::G, 3, false);
}

fn refrain_common(player : &Player) {
    // Never gonna…
    player.play_note(Value::Sixteenth, Note::C, 3, false);

    player.play_note(Value::Sixteenth, Note::D, 3, false);

    player.play_note(Value::Sixteenth, Note::F, 3, false);

    player.play_note(Value::Sixteenth, Note::D, 3, false);
}

fn bridge(player : &Player) {
    for _ in 0..4 {
        player.play_note(Value::Whole, Note::C, 3, false);

        player.play_note(Value::Eighth, Note::F, 3, true);
        player.play_note(Value::Sixteenth, Note::F, 3, false);

        player.play_note(Value::Sixteenth, Note::F, 3, false);

        player.play_pause(Value::Eighth);

        player.play_note(Value::Eighth, Note::E, 3, false);

        player.play_pause(Value::Half);
    }
}

fn refrain(player : &Player, count : usize) {
    for _ in 0..count {
        refrain_common(player);

        // Pattern 17
        player.play_note(Value::Eighth, Note::A, 3, true);
        player.play_note(Value::Sixteenth, Note::A, 3, false);

        player.play_note(Value::Eighth, Note::A, 3, true);
        player.play_note(Value::Sixteenth, Note::A, 3, false);

        player.play_note(Value::Quarter, Note::G, 3, true);
        player.play_note(Value::Eighth, Note::G, 3, false);

        refrain_common(player);

        // Pattern 18
        player.play_note(Value::Eighth, Note::G, 3, true);
        player.play_note(Value::Sixteenth, Note::G, 3, false);

        player.play_note(Value::Eighth, Note::G, 3, true);
        player.play_note(Value::Sixteenth, Note::G, 3, false);

        player.play_note(Value::Quarter, Note::F, 3, true);
        player.play_note(Value::Eighth, Note::F, 3, false);

        refrain_common(player);

        // Pattern 19
        player.play_note(Value::Quarter, Note::F, 3, false);

        player.play_note(Value::Eighth, Note::G, 3, false);

        player.play_note(Value::Eighth, Note::E, 3, true);
        player.play_note(Value::Sixteenth, Note::E, 3, false);

        player.play_note(Value::Sixteenth, Note::D, 3, false);

        player.play_note(Value::Quarter, Note::C, 3, false);

        player.play_note(Value::Eighth, Note::C, 3, false);

        // Pattern 20
        player.play_note(Value::Quarter, Note::G, 3, false);

        player.play_note(Value::Half, Note::F, 3, false);

        refrain_common(player);

        // Pattern 17
        player.play_note(Value::Eighth, Note::A, 3, true);
        player.play_note(Value::Sixteenth, Note::A, 3, false);

        player.play_note(Value::Eighth, Note::A, 3, true);
        player.play_note(Value::Sixteenth, Note::A, 3, false);

        player.play_note(Value::Quarter, Note::G, 3, true);
        player.play_note(Value::Eighth, Note::G, 3, false);

        refrain_common(player);

        // Pattern 21
        player.play_note(Value::Quarter, Note::C, 4, false);

        player.play_note(Value::Eighth, Note::E, 3, false);

        player.play_note(Value::Eighth, Note::F, 3, true);
        player.play_note(Value::Sixteenth, Note::F, 3, false);

        player.play_note(Value::Sixteenth, Note::E, 3, false);

        player.play_note(Value::Eighth, Note::D, 3, false);

        refrain_common(player);

        // Pattern 19
        player.play_note(Value::Quarter, Note::F, 3, false);

        player.play_note(Value::Eighth, Note::G, 3, false);

        player.play_note(Value::Eighth, Note::E, 3, true);
        player.play_note(Value::Sixteenth, Note::E, 3, false);

        player.play_note(Value::Sixteenth, Note::D, 3, false);

        player.play_note(Value::Quarter, Note::C, 3, false);

        player.play_note(Value::Eighth, Note::C, 3, false);

        // Pattern 22
        player.play_note(Value::Quarter, Note::G, 3, false);

        player.play_note(Value::Half, Note::F, 3, false);
    }

    player.play_pause(Value::Quarter);
}

pub fn music() {
    let player = Player::new(114.);

    introduction(&player);

    verse(&player, true);

    refrain(&player, 1);

    verse(&player, false);

    refrain(&player, 2);

    bridge(&player);

    verse(&player, false);

    refrain(&player, 3);
}