/*
By: Draydon Levesque
Date: 2025-05-09
Program Details: Blackjack
*/

mod modules;

use std::process::exit;

use crate::modules::label::Label;
use crate::modules::messagebox::{MessageBox, MessageBoxResult};
use crate::modules::preload_image::LoadingScreenOptions;
use crate::modules::preload_image::TextureManager;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use crate::modules::text_input::TextInput;
use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;
/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "blackjack3".to_owned(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Create card deck data
    let mut deck = deck_reset();
  

    // Add backcard to the full list of assets
    let all_assets = [&deck[..], &["assets/backcard.png"]].concat();

    // Create the texture manager
    let tm = TextureManager::new();

    // Create custom loading screen options
    let loading_options = LoadingScreenOptions {
        title: Some("BLACKJACK".to_string()),
        background_color: DARKGREEN,
        bar_fill_color: Color::new(0.2, 0.8, 0.2, 1.0), // Brighter green
        // Use default values for other options
        ..Default::default()
    };

    // Use the built-in loading screen with custom options
    // Pass the &all_assets slice directly without converting to Vec<String>
    tm.preload_with_loading_screen(&all_assets, Some(loading_options)).await;

    // Continue with the rest of the game setup
    let mut show = "assets/backcard.png";
    let mut end_game = MessageBox::confirm("Out Of Chips", "You have run out of chips. Do you want to play again?");
    let lblchips = Label::new("chips \n 500", 425.0, 525.0, 30);
    let lblplayer = Label::new("0", 450.0, 275.0, 30);
    let lbldealer = Label::new("0", 450.0, 100.0, 30);
    let lblwin = Label::new("", 450.0, 190.0, 30);
    let mut start = TextButton::new(500.0, 400.0, 100.0, 50.0, "Start".to_string(), BLUE, GREEN, 30);
    let mut rand_card = TextButton::new(400.0, 400.0, 100.0, 50.0, "Hit".to_string(), BLUE, GREEN, 30);
    let mut stand = TextButton::new(300.0, 400.0, 100.0, 50.0, "Stand".to_string(), BLUE, GREEN, 30);
    let mut reset = TextButton::new(200.0, 400.0, 100.0, 50.0, "reset".to_string(), BLUE, GREEN, 30);
    let mut txtbet = TextInput::new(100.0, 500.0, 300.0, 40.0, 25.0);

    txtbet.with_colors(WHITE, RED, BLACK, WHITE);
    txtbet.set_prompt("Enter Bet Here");
    rand_card.with_round(15.0);
    rand_card.with_border(RED, 5.0);
    start.with_round(15.0);
    start.with_border(RED, 5.0);
    stand.with_round(15.0);
    stand.with_border(RED, 5.0);
    reset.with_round(15.0);
    reset.with_border(RED, 5.0);
    rand_card.enabled = false;
    stand.enabled = false;
    reset.enabled = false;

    let pcard5 = StillImage::new(show, 75.0, 150.0, 345.0, 200.0, true, 1.0).await;
    let pcard4 = StillImage::new(show, 75.0, 150.0, 290.0, 200.0, true, 1.0).await;
    let pcard3 = StillImage::new(show, 75.0, 150.0, 235.0, 200.0, true, 1.0).await;
    let pcard2 = StillImage::new(show, 75.0, 150.0, 180.0, 200.0, true, 1.0).await;
    let pcard1 = StillImage::new(show, 75.0, 150.0, 125.0, 200.0, true, 1.0).await;
    let dcard1 = StillImage::new(show, 75.0, 150.0, 125.0, 25.0, true, 1.0).await;
    let dcard2 = StillImage::new(show, 75.0, 150.0, 180.0, 25.0, true, 1.0).await;
    let dcard3 = StillImage::new(show, 75.0, 150.0, 235.0, 25.0, true, 1.0).await;
    let dcard4 = StillImage::new(show, 75.0, 150.0, 290.0, 25.0, true, 1.0).await;
    let dcard5 = StillImage::new(show, 75.0, 150.0, 345.0, 25.0, true, 1.0).await;
    let mut pvalue = 0;
    let mut dvalue = 0;
    let mut turn = 3;
    let mut dturn = 2;
    let mut chips = 500;
    let mut bet = 0;
    let mut prebet = 0;

    let mut labels: Vec<Label> = vec![lblchips, lblplayer, lbldealer, lblwin];

    let mut images: Vec<StillImage> = vec![pcard1, pcard2, pcard3, pcard4, pcard5, dcard1, dcard2, dcard3, dcard4, dcard5];

    rand::srand(miniquad::date::now() as u64);
    loop {
        use_virtual_resolution(1024.0, 768.0);
        clear_background(DARKGREEN);
        if start.click() {
            prebet = chips;
            labels[3].set_text(&format!(""));
            let bet_amount = txtbet.get_text();
            if let Ok(amount) = bet_amount.trim().parse::<i32>() {
                if chips < amount && amount > 0 {
                    labels[3].set_text(&format!("Not Enough Chips"));
                } else {
                    bet = amount;
                    if bet <= 0 {
                        labels[3].set_text(&format!("Invalid Bet"));
                    } else {
                        chips -= bet;
                        labels[0].set_text(&format!("Chips:\n {}", chips));
                        stand.enabled = true;
                        rand_card.enabled = true;
                        show = deck.choose().unwrap();

                        images[0].set_preload(tm.get_preload(show).unwrap());
                        deck.retain(|&x| x != show);
                        pvalue = playervalue(pvalue, show);
                        show = deck.choose().unwrap();
                        images[1].set_preload(tm.get_preload(show).unwrap());
                        deck.retain(|&x| x != show);
                        pvalue = playervalue(pvalue, show);

                        show = deck.choose().unwrap();
                        images[5].set_preload(tm.get_preload(show).unwrap());
                        deck.retain(|&x| x != show);
                        dvalue = dealervalue(dvalue, show);
                        start.enabled = false;
                        labels[2].set_text(&format!("Dealer value:\n {}", dvalue));
                        labels[1].set_text(&format!("Player value:\n {}", pvalue));
                        println!("your bet is {}", bet);
                    }
                }
            } else {
                labels[3].set_text(&format!("Invalid bet"));
            }
        }
        if rand_card.click() {
            show = deck.choose().unwrap();
            if turn == 3 {
                images[2].set_preload(tm.get_preload(show).unwrap());

                deck.retain(|&x| x != show);

                pvalue = playervalue(pvalue, show);

                turn = 4;
            } else if turn == 4 {
                images[3].set_preload(tm.get_preload(show).unwrap());
                deck.retain(|&x| x != show);

                pvalue = playervalue(pvalue, show);

                turn = 5;
            } else if turn == 5 {
                images[4].set_preload(tm.get_preload(show).unwrap());
                deck.retain(|&x| x != show);

                pvalue = playervalue(pvalue, show);
            }
            if pvalue > 21 {
                rand_card.enabled = false;
                stand.enabled = false;
                reset.enabled = true;
                prebet = chips;
                labels[3].set_text(&format!("You lose"));
            }
            labels[1].set_text(&format!("Player value:\n {}", pvalue));
            println!("your bet is {}", bet);
        }

        if stand.click() {
            stand.enabled = false;
            rand_card.enabled = false;

            while dvalue < 18 {
                show = deck.choose().unwrap();
                deck.retain(|&x| x != show);
                if dturn == 2 {
                    images[6].set_preload(tm.get_preload(show).unwrap());
                    dturn = 3;
                } else if dturn == 3 {
                    images[7].set_preload(tm.get_preload(show).unwrap());
                    dturn = 4;
                } else if dturn == 4 {
                    images[8].set_preload(tm.get_preload(show).unwrap());
                    dturn = 5;
                } else if dturn == 5 {
                    images[9].set_preload(tm.get_preload(show).unwrap());
                }
                dvalue = dealervalue(dvalue, show);
            }
            labels[2].set_text(&format!("Dealer value:\n {}", dvalue));

            if pwincheck(pvalue, dvalue) == true {
                chips += bet * 2;
                labels[0].set_text(&format!("Chips:\n {}", chips));
                labels[3].set_text(&format!("You Win"));
            } else if dwincheck(pvalue, dvalue) == true {
                prebet = chips;

                labels[3].set_text(&format!("You lose"));
            } else {
                chips += bet;
                labels[0].set_text(&format!("Chips:\n {}", chips));
                labels[3].set_text(&format!("You Draw"));
            }
            println!("your bet is {}", bet);
            reset.enabled = true;
        }
        if reset.click() {
            for i in 0..10 {
                        images[i].set_preload(tm.get_preload("assets/backcard.png").unwrap());
                    }
            pvalue = 0;
            dvalue = 0;
            turn = 3;
            dturn = 2;
            bet = 0;
            start.enabled = true;
            rand_card.enabled = false;
            stand.enabled = false;
            reset.enabled = false;
            labels[1].set_text(&format!("Player value:\n {}", pvalue));
            labels[2].set_text(&format!("Dealer value:\n {}", dvalue));
            txtbet.set_text(&format!(""));
            labels[3].set_text(&format!(""));
            if deck.len() <=10 {
            deck = deck_reset();
            }
            
        }
        if chips == 0 && bet != prebet {
            end_game.show();
            prebet = bet;
            labels[3].set_text(&format!("No More Chips"));
        }

        if let Some(result) = end_game.draw() {
            // Only runs when a button was clicked or dialog was closed
            match result {
                MessageBoxResult::ButtonPressed(0) => {
                    // "Yes" button pressed
                    for i in 0..10 {
                        images[i].set_preload(tm.get_preload("assets/backcard.png").unwrap());
                    }
         
                    pvalue = 0;
                    dvalue = 0;
                    turn = 3;
                    dturn = 2;
                    bet = 0;
                    start.enabled = true;
                    rand_card.enabled = false;
                    stand.enabled = false;
                    reset.enabled = false;
                    labels[1].set_text(&format!("Player value:\n {}", pvalue));
                    labels[2].set_text(&format!("Dealer value:\n {}", dvalue));
                    labels[3].set_text(&format!(""));
                    chips = 500;
                    labels[0].set_text(&format!("Chips:\n {}", chips));
                    reset.enabled = false;
                    txtbet.set_text(&format!(""));
                    
                }
                MessageBoxResult::ButtonPressed(1) => {
                    // "No" button pressed
                    // Continue without saving...
                    exit(0);
                }
                MessageBoxResult::ButtonPressed(2) => {
                    // "Cancel" button pressed (for confirm_with_cancel dialogs)
                    // Handle cancel operation...
                    exit(0);
                }
                #[allow(unused)]
                MessageBoxResult::ButtonPressed(_) => {
                    // IMPORTANT: This catch-all pattern is required by the Rust compiler
                    // even for simple confirm dialogs to ensure all possible values are covered
                    exit(0);
                }
                MessageBoxResult::Closed => {
                    // Dialog closed with X or Escape key
                    // Handle as cancel...
                    exit(0);
                }
            }
        }
        for image in &images {
            image.draw();
        }
        for label in &labels {
            label.draw();
        }

        txtbet.draw();
        end_game.draw();

        next_frame().await;
    }
}

fn playervalue(mut pvalue: i32, show: &str) -> i32 {
    if show.contains("ace") {
        pvalue += 11;
        if pvalue > 21 {
            pvalue -= 10;
        }
    } else if show.contains("jack") || show.contains("queen") || show.contains("king") {
        pvalue += 10;
    } else {
        let part = &show[7..9];
        let value = part.parse::<i32>().unwrap();
        pvalue += value;
    }

    pvalue
}
fn dealervalue(mut dvalue: i32, show: &str) -> i32 {
    if show.contains("ace") {
        dvalue += 11;
        if dvalue > 21 {
            dvalue -= 10;
        }
    } else if show.contains("jack") || show.contains("queen") || show.contains("king") {
        dvalue += 10;
    } else {
        let part = &show[7..9];
        let value = part.parse::<i32>().unwrap();
        dvalue += value;
    }

    dvalue
}
fn pwincheck(pvalue: i32, dvalue: i32) -> bool {
    if pvalue > 21 {
        return false;
    } else if pvalue == 21 && dvalue != 21 {
        return true;
    } else if dvalue > 21 && pvalue < 22 {
        return true;
    } else if pvalue > dvalue {
        return true;
    } else {
        return false;
    }
}
fn dwincheck(pvalue: i32, dvalue: i32) -> bool {
    if dvalue > 21 {
        return false;
    } else if dvalue == 21 && pvalue != 21 {
        return true;
    } else if pvalue > 21 && dvalue < 22 {
        return true;
    } else if dvalue > pvalue {
        return true;
    } else {
        return false;
    }
}
fn deck_reset() -> Vec<&'static str> {
    let mut deck: Vec<&'static str> = Vec::new();
   deck = vec![
        "assets/aceHeart.png",
        "assets/aceDiamond.png",
        "assets/aceClubs.png",
        "assets/aceSpade.png",
        "assets/jackHeart.png",
        "assets/jackDiamond.png",
        "assets/jackClubs.png",
        "assets/jackSpade.png",
        "assets/queenHeart.png",
        "assets/queenDiamond.png",
        "assets/queenClubs.png",
        "assets/queenSpade.png",
        "assets/kingHeart.png",
        "assets/kingDiamond.png",
        "assets/kingClubs.png",
        "assets/kingSpade.png",
        "assets/02clubs.png",
        "assets/02heart.png",
        "assets/02diamond.png",
        "assets/02spade.png",
        "assets/03clubs.png",
        "assets/03heart.png",
        "assets/03diamond.png",
        "assets/03spade.png",
        "assets/04clubs.png",
        "assets/04heart.png",
        "assets/04diamond.png",
        "assets/04spade.png",
        "assets/05clubs.png",
        "assets/05heart.png",
        "assets/05diamond.png",
        "assets/05spade.png",
        "assets/06clubs.png",
        "assets/06heart.png",
        "assets/06diamond.png",
        "assets/06spade.png",
        "assets/07clubs.png",
        "assets/07heart.png",
        "assets/07diamond.png",
        "assets/07spade.png",
        "assets/08clubs.png",
        "assets/08heart.png",
        "assets/08diamond.png",
        "assets/08spade.png",
        "assets/09clubs.png",
        "assets/09heart.png",
        "assets/09diamond.png",
        "assets/09spade.png",
        "assets/10clubs.png",
        "assets/10heart.png",
        "assets/10diamond.png",
        "assets/10spade.png",
    ];
    deck
}