use crate::input;

pub fn day2() -> input::Result<()> {
    let contents = input::load_day_file("day2.txt");
    let mut score: u32 = 0;
    let mut score2: u32 = 0;

    for line in contents.lines() {
        match line {
            "A X" => score += 1 + 3, // pierre / pierre -> égalité
            "A Y" => score += 2 + 6, // pierre / feuille -> victoire
            "A Z" => score += 3 + 0, // pierre / ciseaux -> défaite
            "B X" => score += 1 + 0, // feuille / pierre -> défaite
            "B Y" => score += 2 + 3, // feuille / feuille -> égalité
            "B Z" => score += 3 + 6, // feuille / ciseaux -> victoire
            "C X" => score += 1 + 6, // ciseaux / pierre -> victoire
            "C Y" => score += 2 + 0, // ciseaux / feuille -> défaite
            "C Z" => score += 3 + 3, // ciseaux / ciseaux -> égalité
            _ => (),
        }
    }

    for line in contents.lines() {
        match line {
            "A X" => score2 += 0 + 3, // pierre / défaite -> ciseaux
            "A Y" => score2 += 3 + 1, // pierre / égalité -> pierre
            "A Z" => score2 += 6 + 2, // pierre / victoire -> feuille
            "B X" => score2 += 0 + 1, // feuille / défaite -> pierre
            "B Y" => score2 += 3 + 2, // feuille / égalité -> feuille
            "B Z" => score2 += 6 + 3, // feuille / victoire -> ciseaux
            "C X" => score2 += 0 + 2, // ciseaux / défaite -> feuille
            "C Y" => score2 += 3 + 3, // ciseaux / égalité -> ciseaux
            "C Z" => score2 += 6 + 1, // ciseaux / victoire -> pierre
            _ => (),
        }
    }
    // print the score
    println!("Step 1: {}", score);
    println!("Step 2: {}", score2);

    Ok(())
}