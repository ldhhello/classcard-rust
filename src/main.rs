mod classcard;
use std::io::Write;
use classcard::socket::{Socket, QuestList, Quest};
use tokio::io::{AsyncReadExt};
use colored::Colorize;

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
        .set_platform(String::from("Mac OS X"));

    socket.check_battle(battle_id).await?;

    socket.join(battle_id, nickname).await?;

    let res = socket.get_battle_quest().await?;

    let quest_list = res.quest_list.unwrap();

    println!("{}{}", "시작".bold().bright_green(), "을 기다리는 중");
    socket.wait_for_start().await?;

    println!("{}", "배틀을 시작합니다.".bold());
    println!("{}~{} 이외의 답을 입력하면 {}됩니다.", "1".bold(), "4".bold(), "최종 제출".bold().cyan());

    for (idx, quest) in quest_list.iter().enumerate() {
        println!();
        println!("Question {}: {}", (idx+1).to_string().bold(), quest.front.bold().bright_blue());

        for (i, q) in quest.back_quest.iter().enumerate() {
            println!("{}. {}", (i+1).to_string().bold(), q.bold().bright_magenta());
        }

        print!("{}", "-> ".bold().cyan());
        std::io::stdout().flush()?;
        let ans = input_num().await.unwrap_or(-1);

        if ans <= 0 || ans > 4 {
            break;
        }

        if socket.submit(quest, ans-1).await? {
            println!("{}", "맞았습니다!!".bold().green());
        }
        else {
            println!("{}", "틀렸습니다".bold().red());
        }
    }

    // 여기에 최종 제출 코드 추가
    socket.final_submit().await?;
    println!("{}에 성공했습니다.", "최종 제출".bold().cyan());

    Ok(())
}
