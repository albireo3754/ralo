// use poise::Context;

// #[poise::command(slash_command, rename = "등록", prefix_command)]
// pub async fn enroll(
//     ctx: Context<'_>,
//     // #[description = "등록할 소환사명"] summoner_name: String,
//     // #[description = "티어"]
//     // #[autocomplete = "autocomplete_tier"]
//     // tier: String,
//     // #[description = "등급(마~챌은 아무거나)"]
//     // #[autocomplete = "autocomplete_division"]
//     // division: i32,
// ) -> Result<(), Error> {
//     let discord_user_id = ctx.author().id;
//     if let Some(player) = ctx
//         .data()
//         .player_manager
//         .find_player_with_discord_user_id(discord_user_id.get())
//         .await
//     {
//         ctx.say(format!(
//             "이미 등록된 유저입니다. 등록된 유저: {}",
//             player.summoner_name
//         ))
//         .await?;
//         return Ok(());
//     };

//     let tier = Tier::deserialize_with_tier_division(tier.clone(), division);
//     if tier.is_none() {
//         ctx.say(format!("잘못된 티어 양식입니다")).await?;
//         return Ok(());
//     }

//     let player = ctx
//         .data()
//         .player_manager
//         .register_player(discord_user_id.get())
//         .await;
//     ctx.say(format!(
//         "등록되었습니다. 등록된 유저: {}",
//         player.summoner_name
//     ))
//     .await?;

//     Ok(())
// }
