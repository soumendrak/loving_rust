// Struct

enum HockeyPosition {
    Forward,
    Defender,
    Goalie,
}

struct HockeyPlayer {
    name: String,
    number: u8,
    position: HockeyPosition,
    goals_ytd: u8,
}

fn main() {
    let player = HockeyPlayer {
        name: String::from("Soumendra Kumar Sahoo"),
        number: 100,
        position: HockeyPosition::Defender,
        goals_ytd: 10,
    };
    println!(
        "{} has scored {} goals this year.",
        player.name, player.goals_ytd
    )
}
