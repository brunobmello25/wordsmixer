use std::env;

use wordsmixer::Config;

fn main() {
    let config = Config::load(env::args());

    println!("{}", wordsmixer::mix_words(&config.words).join(" "));
}
