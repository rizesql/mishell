use regex::Regex;

use crate::executables_cache::EXECUTABLES_CACHE;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(String),    // for, if
    Identifier(String), // variable names
    Value(String),      // true,false, $file
    BooleanLiteral(String),
    IntegerLiteral(String),
    FloatLiteral(String),
    StringLiteral(String),
    Operator(String),   // |, >, <, &&, ||
    Command(String),    // grep, echo
    Separator(String),  // ;
    Whitespace(String), // \n \t
    Comment(String),
}

pub async fn tokenizer(input: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();

    let mut is_string = false;

    for character in input.chars() {
        match character {
            '"' => {
                is_string = !is_string;
            }

            ' ' | '\t' | '\n' => {
                if is_string {
                    current_token.push(character);
                } else if !current_token.is_empty() {
                    tokens.push(clasify_tokens(&current_token, &tokens).await);
                    current_token.clear();
                }
            }
            ';' => {
                if !current_token.is_empty() {
                    tokens.push(clasify_tokens(&current_token, &tokens).await);
                    current_token.clear();
                }
                current_token.push(character);
            }

            _ => current_token.push(character),
        }
    }

    if !current_token.is_empty() {
        tokens.push(clasify_tokens(&current_token, &tokens).await);
        current_token.clear();
    }

    tokens
}

pub async fn clasify_tokens(token: &str, tokens_list: &Vec<Token>) -> Token {
    let int_regex = Regex::new(r"^[+-]?\d+$").unwrap();
    let float_regex = Regex::new(r"^[+-]?(\d+\.\d*|\.\d+)$").unwrap();
    let string_regex = Regex::new(r#"^"([^"\\]|\\.)*"$"#).unwrap();

    // Aici se caută token-ul în cache
    if let Some(token) = get_command_from_cache(token, tokens_list).await {
        return token;
    }

    match token {
        _ if int_regex.is_match(token) => Token::IntegerLiteral(token.to_string()),
        _ if float_regex.is_match(token) => Token::FloatLiteral(token.to_string()),
        _ if string_regex.is_match(token) => Token::StringLiteral(token.to_string()),
        "for" | "in" | "do" | "done" | "if" | "else" | "then" | "fi" => {
            Token::Keyword(token.to_string())
        }
        ";" => Token::Separator(token.to_string()),
        "|" | ">" | "<" | "&&" | "||" => Token::Operator(token.to_string()),
        "true" | "false" => Token::BooleanLiteral(token.to_string()),
        _ if token.starts_with('$') => Token::Value(token.to_string()),
        " " | "\n" | "\t" => Token::Whitespace(token.to_string()),
        // "ls" | "echo" | "grep" | "cat" | "find" | "cd" | "mv" | "rm" | "mkdir" | "exec"
        // | "exit" => Token::Command(token.to_string()),
        _ => Token::Value(token.to_string()),
    }
}

async fn get_command_from_cache(token: &str, tokens_list: &Vec<Token>) -> Option<Token> {
    let cache = EXECUTABLES_CACHE.clone();
    if cache.lookup(token).await {
        tracing::info!("command {} is in cache lookup", token);
        match tokens_list.last() {
            Some(last_token) => match last_token {
                Token::Keyword(keyword) if keyword != "for" => {
                    return Some(Token::Command(token.into())); // If found in cache, classify as command
                }
                _ => {}
            },
            None => return Some(Token::Command(token.into())), // If found in cache, classify as command
        }
    }
    None
}

pub fn debug_tokens(tokens: &[Token]) {
    for (i, token) in tokens.iter().enumerate() {
        println!("Token {}: {:?}", i, token);
    }
}
