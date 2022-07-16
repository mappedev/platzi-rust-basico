use csv::{ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::{fs, io};

#[derive(Debug)]
struct HistoryData {
    data_type: String,
    tag: String,
    text: String,
    live: i32,
    options: Vec<HistoryData>,
}

impl HistoryData {
    fn new(row: StringRecord) -> HistoryData {
        HistoryData {
            data_type: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            text: row.get(2).unwrap().trim().to_string(),
            live: row.get(3).unwrap().trim().parse().unwrap_or(0),
            options: vec![],
        }
    }
}

const FILENAME_PATH: &str = "./src/utils/history.csv";
const INIT_TAG: &str = "INICIO";

fn main() {
    let content = fs::read_to_string(FILENAME_PATH).unwrap();
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(content.as_bytes());
    let mut history_data: HashMap<String, HistoryData> = HashMap::new();
    let mut last_record = String::new();
    let mut live = 100;
    let mut current_tag = INIT_TAG;

    for row in rdr.records() {
        let row = row.unwrap();
        let data = HistoryData::new(row);

        if data.data_type == "SITUACION" {
            let record_tag = data.tag.clone();

            history_data.insert(record_tag.clone(), data);
            last_record = record_tag;
        } else {
            if let Some(last_tag_data) = history_data.get_mut(&last_record) {
                (*last_tag_data).options.push(data);
            }
        }
    }

    loop {
        println!("Tienes {} de vida", live);
        if let Some(data) = history_data.get(current_tag) {
            println!("{}", data.text);
            for (i, option) in data.options.iter().enumerate() {
                println!("[{}] {}", i, option.text);
            }

            let mut selection_input = String::new();
            io::stdin().read_line(&mut selection_input).unwrap();
            let selection = selection_input.trim().parse().unwrap_or(99);

            if let Some(selection_data) = data.options.get(selection) {
                current_tag = &selection_data.tag;
            }

            live += data.live;
            println!("");
        } else {
            break;
        }

        if live <= 0 {
            println!("Fin del juego...");
            break;
        }
    }
}
