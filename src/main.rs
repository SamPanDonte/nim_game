use nim_game::NimGame;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let amount = std::env::args()
        .nth(1)
        .map(|arg| arg.parse::<usize>())
        .unwrap_or(Ok(100))?;

    let now = std::time::Instant::now();
    let mut game = NimGame::with_capacity(amount);

    game.solve(amount);

    for index in 1..=amount {
        if index % 2 == 0 && (index <= 2 || game[index - 2] != 0) {
            println!(
                "{index}. {} - {}",
                if game[index] != 0 { "win" } else { "lose" },
                game[index]
            );
        }
    }

    println!(
        "Total time: {:?}",
        std::time::Instant::now().duration_since(now)
    );

    Ok(())
}
