use std::{mem::swap, str::FromStr, sync::Arc};

use futures_util::{SinkExt, StreamExt};
use http::{header::{self}, HeaderValue, Uri};
use tokio::{net::{TcpListener, TcpStream}, sync::Mutex};
use tokio_websockets::{ClientBuilder, MaybeTlsStream, Message, WebSocketStream};

use serde::{Deserialize, Serialize};

use serde_json::json;

pub struct Socket {
    socket: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    browser: String,
    platform: String,
    user_agent: String,
    test_id: i32,
    is_start: bool,
    total_score: i32,
    send_buffer: Vec<(String, i32, bool)>, // card_idx, score, is_correct
    rank_id: String,
    username: String,
    quest_cnt: i32,
    correct_cnt: i32,
}

impl Socket {
    fn get_port(battle_id: i32) -> i32 {
        let mut port = 800;
		if battle_id > 18999 && battle_id < 28000 {
			port = 801;
		} else if battle_id > 27999 && battle_id < 37000 {
			port = 802;
		} else if battle_id > 36999 && battle_id < 46000 {
			port = 803;
		} else if battle_id > 45999 && battle_id < 55000 {
			port = 804;
		} else if battle_id > 54999 && battle_id < 64000 {
			port = 805;
		} else if battle_id > 63999 && battle_id < 73000 {
			port = 806;
		} else if battle_id > 72999 && battle_id < 82000 {
			port = 807;
		} else if battle_id > 81999 && battle_id < 91000 {
            port = 808;
		} else if battle_id > 90999 && battle_id < 100000 {
			port = 809;
		}

        return port;
    }

    pub async fn connect(host: String, battle_id: i32) -> Result<Socket, Box<dyn std::error::Error>> {
        let uri = format!("wss://{}/wss_{}", host, Self::get_port(battle_id));

        let uri = Uri::from_str(uri.as_str())?;
        let (client, _) = ClientBuilder::from_uri(uri)
        .add_header(
            header::USER_AGENT, 
            HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
        )
        .add_header(
            header::ORIGIN,
            HeaderValue::from_static("https://b.classcard.net")
        )
        .connect().await?;

        let client = Arc::new(Mutex::new(client));

        let client_ = client.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs_f32(10.0)).await;

                match client_.lock().await.send(Message::text("{\"cmd\":\"pong\"}")).await {
                    Err(_) => return,
                    _ => ()
                };
            };
        });

        Ok(Socket {
            socket: client,
            browser: String::from("Chrome"),
            platform: String::from("Mac OS X"),
            user_agent: String::from("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36"),
            test_id: -1,
            is_start: false,
            total_score: 1000,
            send_buffer: vec![],
            rank_id: String::from(""),
            username: String::from(""),
            quest_cnt: 0,
            correct_cnt: 0
        })
    }
    pub fn set_browser(mut self, browser: String) -> Self {
        self.browser = browser;
        return self;
    }
    pub fn set_platform(mut self, platform: String) -> Self {
        self.platform = platform;
        return self;
    }
    pub fn set_user_agent(mut self, agent: String) -> Self {
        self.user_agent = agent;
        return self;
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CheckResult {
    pub cmd: String,
    pub result: String,
    pub b_name: String,
    pub n: i32,
    pub id: String,
    pub is_ver: bool,
    pub ptn_idx: i32,
    pub reason: String
}

impl Socket {
    pub async fn check_battle(&self, battle_id: i32) -> Result<CheckResult, Box<dyn std::error::Error>> {
        let json = json!({
            "cmd": "b_check",
            "battle_id": battle_id.to_string(),
            "is_auto": false,
            "major_ver": 8,
            "minor_ver": 0
        });

        self.socket.lock().await.send(Message::text(json.to_string())).await?;

        let Some(Ok(msg)) = self.socket.lock().await.next().await else { return Err(Box::from("Failed to read from socket")); };

        let Some(data) = msg.as_text() else { return Err(Box::from("Failed to get data")); };

        let res: CheckResult = serde_json::from_str(data)?;

        Ok(res)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JoinResult {
    pub cmd: String,
    pub result: String,
    pub msg: String,
    pub test_id: i32,
    pub test_time: i32,
    pub test_random: bool,
    pub b_name: String,
    pub b_image: Option<bool>,
    pub b_user_idx: String,
    pub is_end: bool, // 이미 끝난 게임인지 여부
    pub is_start: bool, // 이미 시작한 게임인지 여부
    pub b_mode: i32, // ?
    pub set_idx: i32, // 단어장 인덱스
    pub set_name: String, // 단어장 이름
    pub set_type: i32, // 단어장 타입
    pub b_text: Option<bool>, // 기본값 true인데 뭔지 모름
    pub is_event: i32,
    pub ptn_idx: i32
}

impl Socket {
    pub async fn join(&mut self, battle_id: i32, username: String) -> Result<JoinResult, Box<dyn std::error::Error>> {
        self.username = username;
        let username = &self.username;

        let json = json!({
            "cmd": "b_join",
            "battle_id": battle_id.to_string(),
            "user_name": username,
            "is_add": 0,
            "is_auto": false,
            "platform": self.platform,
            "browser": self.browser,
            "major_ver": 8,
            "minor_ver": 0
        });

        self.socket.lock().await.send(Message::text(json.to_string())).await?;

        let Some(Ok(msg)) = self.socket.lock().await.next().await else { return Err(Box::from("Failed to read from socket")); };

        let Some(data) = msg.as_text() else { return Err(Box::from("Failed to get data")); };

        let res: JoinResult = serde_json::from_str(data)?;

        self.test_id = res.test_id;
        self.is_start = res.is_start;

        let json = json!({
            "cmd": "b_join",
            "battle_id": battle_id.to_string(),
            "user_name": username,
            "is_add": 1,
            "is_auto": false,
            "platform": self.platform,
            "browser": self.browser,
            "major_ver": 8,
            "minor_ver": 0
        });

        self.socket.lock().await.send(Message::text(json.to_string())).await?;

        // b_team 메서드가 들어오는데 별로 안 중요해 보임
        let Some(Ok(_)) = self.socket.lock().await.next().await else { return Err(Box::from("Failed to read from socket")); };

        Ok(res)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestOption {
    pub option_idx: String,
    pub option_quest: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quest {
    pub test_card_idx: String,
    pub front: String,
    pub back: String,
    pub example_sentence: String,
    //pub option_info: Vec<QuestOption>,
    //pub example_front: String,
    //pub example_back: String,
    //pub front_quest: Vec<String>,
    pub back_quest: Vec<String>,
    //pub exam_quest: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestList {
    pub result: String,
    pub msg: String,
    pub quest_list: Option<Vec<Quest>>
}

impl Socket {
    pub async fn get_battle_quest(&self) -> Result<QuestList, Box<dyn std::error::Error>> {
        let params = [("test_id", self.test_id.to_string())];
        let client = reqwest::Client::new();
        let res = client.post("https://b.classcard.net/ClassBattle/battle_quest")
            .header("User-Agent", self.user_agent.clone())
            .header("Accept", "*/*")
            .form(&params)
            .send()
            .await?;

        res.error_for_status_ref()?;

        let json: QuestList = serde_json::from_str(res.text().await?.as_str())?;

        Ok(json)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestStart {
    cmd: String,
    rank_id: String,
    is_field: i32
}

impl Socket {
    pub async fn wait_for_start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_start {
            return Ok(())
        }

        let Some(Ok(msg)) = self.socket.lock().await.next().await else { return Err(Box::from("Failed to read from socket")); };
        let Some(data) = msg.as_text() else { return Err(Box::from("Failed to get data")); };

        //println!("{}", data);

        let data: TestStart = serde_json::from_str(data)?;

        if data.cmd != "b_test_start" {
            return Err(Box::from("Server sent error"));
        }

        self.rank_id = data.rank_id;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendRankResult {
    cmd: String,
}

impl Socket {
    // answer는 0-based 인덱스
    pub async fn submit(&mut self, quest: &Quest, answer: i32) -> Result<bool, Box<dyn std::error::Error>> {
        let is_correct = if quest.back_quest[answer as usize] == quest.back { true }
        else { false };

        let score = if is_correct { 100 } else { 0 };

        self.total_score += score;

        let card_idx = &quest.test_card_idx;
        self.send_buffer.push((card_idx.clone(), score, is_correct));

        self.quest_cnt += 1;
        if is_correct {
            self.correct_cnt += 1;
        }

        if self.send_buffer.len() < 5 {
            return Ok(is_correct);
        }

        let mut buffer = vec![];
        swap(&mut self.send_buffer, &mut buffer);

        let json = json!({
            "cmd": "b_get_rank".to_string(),
            "total_score": self.total_score,
            "quest": buffer.into_iter().map(|(card_idx, score, is_correct)| {
                json!({
                    "card_idx": card_idx,
                    "score": score,
                    "correct_yn": is_correct as i32
                })
            }).collect::<Vec<_>>()
        });

        self.socket.lock().await.send(Message::text(json.to_string())).await?;

        let Some(Ok(msg)) = self.socket.lock().await.next().await else { return Err(Box::from("Failed to read from socket")); };
        let Some(data) = msg.as_text() else { return Err(Box::from("Failed to get data")); };

        //println!("{}", data);

        let data: SendRankResult = serde_json::from_str(data)?;

        if data.cmd != "b_send_rank" {
            return Err(Box::from("Server sent error"));
        }

        Ok(is_correct)
    }
    pub async fn final_submit(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = vec![];
        swap(&mut self.send_buffer, &mut buffer);

        let json = json!({
            "cmd": "b_get_rank".to_string(),
            "total_score": self.total_score,
            "quest": buffer.into_iter().map(|(card_idx, score, is_correct)| {
                json!({
                    "card_idx": card_idx,
                    "score": score,
                    "correct_yn": is_correct as i32
                })
            }).collect::<Vec<_>>()
        });
        self.socket.lock().await.send(Message::text(json.to_string())).await?;

        let params = [
            ("rank_id", self.rank_id.to_string()),
            ("rank_type", 0.to_string()),
            ("rank_num", 99999.to_string()),
            ("user_name", self.username.clone()),
            ("total_score", self.total_score.to_string()),
            ("quest_cnt", self.quest_cnt.to_string()),
            ("correct_cnt", self.correct_cnt.to_string()),
            ("wrong_cnt", (self.quest_cnt - self.correct_cnt).to_string()),
            ("unknow_cnt", 0.to_string())
        ];
        let client = reqwest::Client::new();
        let res = client.post("https://b.classcard.net/ClassMainAsync/battle_send_std_score")
            .header("User-Agent", self.user_agent.clone())
            .header("Accept", "*/*")
            .form(&params)
            .send()
            .await?;

        res.error_for_status_ref()?;

        //println!("{}", res.text().await?);

        Ok(())
    }
}