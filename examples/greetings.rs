use tegen::tegen::TextGenerator;

fn main() {
    let tg = TextGenerator::new();
    println!("{}", tg.generate("{Good {night|morning|evening|day}|Hello|Greetings|Howdy|What's up}, {friend|mate}! {How are you|How's it going}?"));
}
