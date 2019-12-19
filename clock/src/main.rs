mod lib;
use lib::Clock;

fn main() {
    Clock::new(0, 1) == Clock::new(0, 1441);
}
