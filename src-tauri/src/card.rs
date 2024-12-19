use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Card {
    id: String,
    name: String,
    #[serde(default)]
    cost: u32,
    #[serde(default)]
    health: u32,
    #[serde(default)]
    attack: u32,
    #[serde(default)]
    text: String,
    #[serde(default)]
    #[serde(rename = "type")]
    card_type: String,
}

pub fn data_list() -> Vec<Card> {

    // 1. 读取 JSON 文件内容
    let file_content = fs::read_to_string("./config/cards.json").unwrap();

    // 2. 解析 JSON 数据到结构体
    let cards: Vec<Card> = serde_json::from_str(&file_content).expect("Failed to parse JSON");
    cards
}

pub fn data_map() -> HashMap<String, Card> {
    let cards = data_list();
    let mut map: HashMap<String, Card> = HashMap::new();
    for card in cards {
        map.insert(card.id.to_string(), card);
    }
    map
}

pub fn get_card_name(name: String) -> Card {
    let cards = data_list();

    for card in cards {
        if card.name.contains(name.as_str()) {
            return card;
        }
    }
    Card {
        id: "-1".to_string(),
        name,
        cost: 0,
        health: 0,
        attack: 0,
        text: "".to_string(),
        card_type: "".to_string(),
    }

    // let map = data_map();
    // let key = "AT_005t";
    // let card = map.get(name.as_str()).unwrap().clone();
    // println!("User wrote {:?}", card);
    // card
}
#[test]
fn aa() {
    let card = get_card_name("无界".to_string());
    println!("{:?}", card)
}