use rand::Rng;
use crate::{Error,Context};
#[derive(poise::ChoiceParameter, Debug, PartialEq)]
pub enum RPSOptions {
    Rock,
    Paper,
    Scissors,
}
///Rock Paper Scissors
#[poise::command(slash_command)]
pub async fn rps(ctx: Context<'_>,player_choice:RPSOptions) -> Result<(), Error> {
    let bot_choice = {
        let mut rng = rand::thread_rng();
        let result = rng.gen_range(1..=3);
        match result {
            1 => RPSOptions::Rock,
            2 => RPSOptions::Paper,
            3 => RPSOptions::Scissors,
            _ => todo!(),
        }

    };
    let response = match (player_choice, bot_choice) {
        (RPSOptions::Rock, RPSOptions::Paper) => "Paper, I win!",
        (RPSOptions::Rock, RPSOptions::Scissors) => "Scissors, you win!",
        (RPSOptions::Rock, RPSOptions::Rock) => "Rock, it's a tie!",
        (RPSOptions::Paper, RPSOptions::Scissors) => "Scissors, I win!",
        (RPSOptions::Paper, RPSOptions::Rock) => "Rock, you win.",
        (RPSOptions::Paper, RPSOptions::Paper) => "Paper, it's a tie!",
        (RPSOptions::Scissors, RPSOptions::Rock) => "Rock, I win!",
        (RPSOptions::Scissors, RPSOptions::Paper) => "Paper, you win!",
        (RPSOptions::Scissors, RPSOptions::Scissors) => "Scissors, it's a tie!",
       };
    ctx.say(response)
        .await?;
    Ok(())
}