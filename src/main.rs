use curl::easy::Easy;
use skim::prelude::*;
use std::io::Cursor;
use std::fs;
use serde::{Serialize, Deserialize};

const VERB_LIST_FILE: &str = "/usr/local/share/conjugator/german_verb_list.txt";
const CONJUGATION_URL: &str = "https://german-verbs-conjugation-api.herokuapp.com/german-verbs-api?verb=VERB&tense=TENSE&verbCase=CASE";
const VERB_TENSES: [&str; 11] = [
    "PRASENS",
    "PRATERITUM",
    "FUTUR1",
    "FUTUR2",
    "PERFEKT",
    "PLUSQUAMPERFEKT",
    "KONJUNKTIV1_PRASENS",
    "KONJUNKTIV1_PERFEKT",
    "KONJUNKTIV2_PRATERITUM",
    "KONJUNKTIV2_FUTUR1",
    "KONJUNKTIV2_FUTUR2",
];

const VERB_CASES: [&str; 4] = [
    "NOMINATIVE",
    "DATIVE",
    "ACCUSATIVE",
    "GENITIVE",
];


#[derive(Serialize, Deserialize)]
struct Data {
    S1: Vec<String>,
    S2: Vec<String>,
    S3: Vec<String>,
    P1: Vec<String>,
    P2: Vec<String>,
    P3: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Verb {
    success: bool,
    data: Data,
}


/// Prompts user to select opting using `skim` interface.
/// Takes a newline separated string.
/// Returns user select item.
///
/// # Errors.
/// Returns an empty string on failure.
fn user_select_item(prompt: &str, content: String) -> String {
    // Set skim options
    let options = SkimOptionsBuilder::default()
        .multi(false)
        .prompt(Some(prompt))
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(content));

    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());


    if let Some(choice) = selected_items.iter().next() {
        return choice.output().to_string();
    }

    return "".to_string();
}


/// Conjugate German verbs.
fn main() {

    // User selects verb
    let verb_choices: String = fs::read_to_string(VERB_LIST_FILE).expect("Failed to read file");
    let verb: String = user_select_item(&"Select verb: ", verb_choices)
        .replace("ö", "oe")
        .replace("ü", "ue")
        .replace("ä", "ae")
        .replace("ß", "ss");

    // User selects verb tense
    let mut tense_choices: String = "".to_string();
    for tense in VERB_TENSES.iter() {
        tense_choices.push_str(&(tense.to_lowercase() + &"\n".to_string()));
    }
    let tense: String = user_select_item(&"Select tense: ", tense_choices);

    // User selects verb case
    let mut case_choices: String = "".to_string();
    for case in VERB_CASES.iter() {
        case_choices.push_str(&(case.to_lowercase() + &"\n".to_string()));
    }
    let case: String = user_select_item(&"Select verb case: ", case_choices);


    let url: String = CONJUGATION_URL
        .replace("VERB", &verb)
        .replace("CASE", &case)
        .replace("TENSE", &tense.to_uppercase());

    println!("{}", url);


    let mut handle = Easy::new();

    handle.get(true).unwrap();
    handle.url(&url).unwrap();

    let mut buf = Vec::new();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            buf.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    let foo: Verb = serde_json::from_str(&String::from_utf8(buf).unwrap()).unwrap();
    if !foo.success {
        return;
    }

    // WARING: Extremely WET code!! Proceed with caution. You have been warned!
    print!("ich");
    for item in foo.data.S1.iter() {
        print!(" {}", item);
    }
    println!("");

    print!("du ");
    for item in foo.data.S2.iter() {
        print!(" {}", item);
    }
    println!("");

    print!("er ");
    for item in foo.data.S3.iter() {
        print!(" {}", item);
    }
    println!("");

    print!("wir");
    for item in foo.data.P1.iter() {
        print!(" {}", item);
    }
    println!("");

    print!("ihr");
    for item in foo.data.P2.iter() {
        print!(" {}", item);
    }
    println!("");

    print!("Sie");
    for item in foo.data.P3.iter() {
        print!(" {}", item);
    }
    println!("");
}

