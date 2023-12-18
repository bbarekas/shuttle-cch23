use axum::{routing::{get, post}, Router};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};


pub fn get_routes() -> Router {

    Router::new()
        .route("/15", get(axum::http::StatusCode::OK))
        .route("/15/nice", post(validate_pass))
        .route("/15/game", post(validation_game))
}

#[derive(Deserialize, Serialize)]
struct ValidateInput {
    input: String,
}

#[derive(Serialize, Deserialize)]
enum Result {
    #[serde(rename = "nice")]
    Nice,
    #[serde(rename = "naughty")]
    Naughty,
}

#[derive(Serialize, Deserialize)]
struct ValidateOutput {
    result: Result,
}


impl ValidateInput {
    fn is_nice(&self) -> bool {
        self.check_three_vowels() &&
        self.check_double_letter() &&
        self.check_forbidden_substrs()
    }

    fn check_three_vowels(&self) -> bool {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
        self.input.matches(&vowels[..]).count() >= 3
    }

    fn check_double_letter(&self) -> bool {
        let mut previous = None;
        for c in self.input.chars() {
            if Some(c) == previous && c.is_alphabetic() {
                return true;
            }
            previous = Some(c);
        }
        false
    }

    fn check_forbidden_substrs(&self) -> bool {
        let forbidden_substrs = ["ab", "cd", "pq", "xy"];
        for pat in forbidden_substrs.into_iter() {
            match self.input.matches(pat).next() {
                Some(_) => return false,
                None => (),
            }
        }
        true
    }

    fn
    check_rule_1(&self) -> bool {
        self.input.len() >= 8
    }

    fn check_rule_2(&self) -> bool {
        let mut uppercase_found = false;
        let mut lowercase_found = false;
        let mut digit_found = false;

        for c in self.input.chars() {
            if c.is_uppercase() {
                uppercase_found = true;
            } else if c.is_lowercase() {
                lowercase_found = true;
            } else if c.is_numeric() {
                digit_found = true;
            }

            if uppercase_found && lowercase_found && digit_found {
                return true;
            }
        }
        false
    }

    fn check_rule_3(&self) -> bool {
        self.input.chars().filter(|c| c.is_numeric()).count() >= 5
    }

    fn check_rule_4(&self) -> bool {
        self.input
            .chars()
            .map(|c| if c.is_numeric() { c } else { ' ' })
            .collect::<String>()
            .split_whitespace()
            .map(|int| int.parse::<u32>().unwrap())
            .sum::<u32>()
            == 2023
    }

    fn rule_five_broken(&self) -> bool {
        let mut chars = self.input.chars();
        let mut last_last_c = match chars.next() {
            Some(c) => c,
            None => return true,
        };
        let mut last_c = match chars.next() {
            Some(c) => c,
            None => return true,
        };
        let mut found_joy = false;
        for c in self.input.chars() {
            if c.is_alphabetic() {
                match (last_last_c, last_c, c) {
                    (_, 'j', 'o') => (),
                    (_, _, 'o') => return true,
                    ('j', 'o', 'y') => found_joy = true,
                    (_, _, 'y') | ('o', 'y', 'j') | ('y', _, 'j') => return true,
                    (_, _, 'j') => (),
                    (_, 'j', _) => return true,
                    _ => (),
                }
                last_last_c = last_c;
                last_c = c;
            }
        }
        !found_joy
    }

    fn check_rule_6(&self) -> bool {
        let mut chars = self.input.chars();
        let mut last_last_c = match chars.next() {
            Some(c) => c,
            None => return false,
        };
        let mut last_c = match chars.next() {
            Some(c) => c,
            None => return false,
        };
        for c in chars {
            if c == last_last_c && c.is_alphabetic() && last_c.is_alphabetic() {
                return true;
            }
            last_last_c = last_c;
            last_c = c;
        }
        false
    }

    fn check_rule_7(&self) -> bool {
        for c in self.input.chars() {
            match c {
                '\u{2980}'..='\u{2BFF}' => return true,
                _ => (),
            }
        }
        false
    }

    fn check_rule_8(&self) -> bool {
        for c in self.input.chars() {
            match emojis::get(c.to_string().as_str()) {
                Some(_) => return true,
                None => (),
            }
        }
        false
    }

    fn check_rule_9(&self) -> bool {
        sha256::digest(&self.input).ends_with('a')
    }
}



async fn validate_pass(Json(data): Json<ValidateInput>) -> (StatusCode, Json<ValidateOutput>) {
    if data.is_nice() {
        (StatusCode::OK, Json(ValidateOutput {result: Result::Nice} ))
    } else {
        (StatusCode::BAD_REQUEST, Json(ValidateOutput {result: Result::Naughty} ))
    }

}



#[derive(Serialize, Deserialize)]
enum Reason {
    #[serde(rename = "8 chars")]
    One,
    #[serde(rename = "more types of chars")]
    Two,
    #[serde(rename = "55555")]
    Three,
    #[serde(rename = "math is hard")]
    Four,
    #[serde(rename = "not joyful enough")]
    Five,
    #[serde(rename = "illegal: no sandwich")]
    Six,
    #[serde(rename = "outranged")]
    Seven,
    #[serde(rename = "😳")]
    Eight,
    #[serde(rename = "not a coffee brewer")]
    Nine,
    #[serde(rename = "that's a nice password")]
    None,
}


#[derive(Serialize, Deserialize)]
struct ValidationGameOutput {
    result: Result,
    reason: Reason
}


async fn validation_game(Json(data): Json<ValidateInput>) -> (StatusCode, Json<ValidationGameOutput>) {


    if !data.check_rule_1() {
        return (StatusCode::BAD_REQUEST,
                Json(ValidationGameOutput {result: Result::Naughty, reason: Reason::One} ));
    }
    if !data.check_rule_2() {
        return (StatusCode::BAD_REQUEST,
                Json(ValidationGameOutput {result: Result::Naughty, reason: Reason::Two} ));
    }
    if !data.check_rule_3() {
        return (StatusCode::BAD_REQUEST,
                Json(ValidationGameOutput {result: Result::Naughty, reason: Reason::Three} ));
    }
    if !data.check_rule_4() {
        return (StatusCode::BAD_REQUEST,
                Json(ValidationGameOutput {result: Result::Naughty, reason: Reason::Four} ));
    }
    if data.rule_five_broken() {
        return (StatusCode::NOT_ACCEPTABLE,
                Json(ValidationGameOutput {result: Result::Naughty, reason: Reason::Five} ));
    }
    if !data.check_rule_6() {
        return (StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                Json(ValidationGameOutput {result: Result::Naughty, reason: Reason::Six} ));
    }
    if !data.check_rule_7() {
        return (StatusCode::RANGE_NOT_SATISFIABLE,
                Json(ValidationGameOutput {result: Result::Naughty, reason: Reason::Seven} ));
    }
    if !data.check_rule_8() {
        return (StatusCode::UPGRADE_REQUIRED,
                Json(ValidationGameOutput {result: Result::Naughty, reason: Reason::Eight} ));
    }
    if !data.check_rule_9() {
        return (StatusCode::IM_A_TEAPOT,
                Json(ValidationGameOutput {result: Result::Naughty, reason: Reason::Nine} ));
    }

    (StatusCode::OK, Json(ValidationGameOutput {result: Result::Nice, reason: Reason::None} ))
}
