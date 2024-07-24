mod classcard;
use std::io::Write;
use clap::Parser;
use classcard::socket::{Socket, QuestList, Quest};
use tokio::io::{AsyncReadExt};
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(about, version, before_help = 
    format!("{} by {}\nVersion {}", "classcard-rust".bold().yellow(), "Donghyun Lee".bold().bright_cyan(), env!("CARGO_PKG_VERSION"))
)]
struct Cli {
    /// 응답 버퍼의 크기입니다.
    #[arg(short, long, default_value_t = 5)]
    buffer_size: usize,

    /// 문제를 맞았을 때 얻는 점수입니다. 
    #[arg(short, long, default_value_t = 100)]
    correct_score: i32,
}

pub async fn input() -> Result<String, Box<dyn std::error::Error>> {
    let mut res = Vec::new();
    let mut stdin = tokio::io::stdin();
    loop {
        let ch = stdin.read_u8().await?;

        if ch == b'\r' {
            continue;
        }
        if ch == b'\n' {
            return Ok(String::from_utf8(res)?);
        }

        res.push(ch);
    }
}
pub async fn input_num() -> Result<i32, Box<dyn std::error::Error>> {
    let str = input().await?;
    return Ok(str::parse::<i32>(&str)?);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    println!("{} by {}", "classcard-rust".bold().yellow(), "Donghyun Lee".bold().bright_cyan());
    println!("{}: https://github.com/ldhhello", "Github".bold().yellow());
    println!();

    print!("{}{}", "배틀 id".bold().green(), "를 입력하세요: ");
    std::io::stdout().flush()?;
    let battle_id = input_num().await?;

    print!("{}{}", "닉네임".bold().green(), "을 입력하세요: ");
    std::io::stdout().flush()?;
    let nickname = input().await?;

    let mut socket = Socket::connect(String::from("mobile3.classcard.net"), battle_id).await?
        .set_browser(String::from("Chrome"))
        .set_platform(String::from("Mac OS X"))
        .set_buffer_size(cli.buffer_size)
        .set_correct_score(cli.correct_score);

    socket.check_battle(battle_id).await?;

    socket.join(battle_id, nickname).await?;

    let res = socket.get_battle_quest().await?;

    let quest_list = res.quest_list.unwrap();

    println!("{}{}", "시작".bold().bright_green(), "을 기다리는 중");
    socket.wait_for_start().await?;

    println!();
    println!("{}", "배틀을 시작합니다.".bold());
    println!("{} 또는 {}을 입력하면 {}됩니다.", "0".bold(), "음수 값".bold(), "최종 제출".bold().cyan());

    'outer: loop {
        for (idx, quest) in quest_list.iter().enumerate() {
            println!();
            println!("Question {}: {}", (idx+1).to_string().bold(), quest.front.bold().bright_blue());
    
            for (i, q) in quest.back_quest.iter().enumerate() {
                println!("{}. {}", (i+1).to_string().bold(), q.bold().bright_magenta());
            }
    
            let ans = loop {
                print!("{}", "-> ".bold().cyan());
                std::io::stdout().flush()?;
                let ans = input_num().await.unwrap_or(1000);
    
                if 1 <= ans && ans <= 4 {
                    break ans;
                }
                else if ans <= 0 {
                    break ans;
                }
            };
    
            if ans <= 0 {
                break 'outer;
            }
    
            if socket.submit(quest, ans-1).await? {
                println!("{}", "맞았습니다!!".bold().green());
            }
            else {
                println!("{}", "틀렸습니다".bold().red());
            }
        }
    }

    socket.final_submit().await?;
    println!("{}에 성공했습니다.", "최종 제출".bold().cyan());
    Ok(())
}
