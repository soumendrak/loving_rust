// Enums

enum Clock {
    Sundial(u8),
    Digital(u8, u8),
    Analog(u8, u8, u8),
}

fn tell_me(clock: Clock) {
    match clock {
        Clock::Sundial(hours) => println!("It's about {} hours.", hours),
        Clock::Digital(hours, minutes) => {
            println!("It's about {} hours and {} minutes.", hours, minutes)
        }
        Clock::Analog(hours, minutes, seconds) => {
            println!("It's about {}:{}:{}", hours, minutes, seconds)
        }
    }
}

fn main() {
    tell_me(Clock::Sundial(10));
    tell_me(Clock::Analog(10, 20, 10));
    tell_me(Clock::Digital(10, 20));
}
