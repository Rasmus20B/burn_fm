use crate::decompile;
use crate::component;
use crate::file;

use crate::compile::token::*;

pub fn compile_burn(code: &str) {
    let tokens = tokenize(code);
}

fn tokenize(code: &str) -> Vec<Token> {
    let mut list = Vec::<Token>::new();
    let mut buffer = String::new();

    for c in code.chars() {
        if !c.is_whitespace() || buffer.len() > 0 {
            buffer.push(c);
        }
        let b = buffer.as_str();

        let tmp = match c {
            ' ' => if buffer.is_empty() { vec![] } else {
                match buffer.trim() {
                    "table" => vec![Token { ttype: TokenType::Table, text: b.trim().to_string() }],
                    "relationship" => vec![Token { ttype: TokenType::Relationship, text: b.trim().to_string() }],
                    "value_list" => vec![Token { ttype: TokenType::ValueList, text: b.trim().to_string() }],
                    "end" => vec![Token { ttype: TokenType::End, text: "end".trim().to_string() }],
                    "FoundIn" => vec![Token { ttype: TokenType::FoundIn, text: "FoundIn".to_string() }],
                    "Text," | "Number," | "Date," | "Time," | "Timestamp," | "Container," => 
                        vec![Token { ttype: TokenType::DataType, 
                            text: b
                                .strip_suffix(',')
                                .unwrap()
                                .to_string() } ],
                    _ => vec![]
                }
            },
            '[' => if buffer.is_empty() { 
                        vec![ Token { ttype: TokenType::OpenSquare, text: "[".to_string() }]
                    } else {
                        match buffer.trim().strip_suffix('[').unwrap() {
                            "FoundIn" => vec![
                                Token { ttype: TokenType::FoundIn, text: "FoundIn".to_string() },
                                Token { ttype: TokenType::OpenSquare, text: "[".to_string() }],
                            _ => vec![]
                    }
            },
            ']' =>  if buffer.is_empty() { 
                        vec![Token { ttype: TokenType::CloseSquare, text: "]".to_string() }]
                    } else {
                        vec![
                            Token { ttype: TokenType::Identifier, text: buffer.strip_suffix(']').unwrap().to_string() },
                            Token { ttype: TokenType::CloseSquare, text: "]".to_string() }
                        ]
                    }
            '"' =>  if buffer.len() > 1 {
                        vec![ Token { ttype: TokenType::String, text: buffer.to_string()}]
                    } else {
                        vec![]
                    }
            '!' =>  if list.last().unwrap().ttype != TokenType::OpenCurly {
                        vec![Token { ttype: TokenType::Exclamation,  text: "!".to_string() }]
                    } else {
                        vec![]
                    }
            '{' => vec![Token { ttype: TokenType::OpenCurly, text: "{".to_string() }],
            '}' =>  if buffer.len() > 1 {
                        let mut ret = vec![];
                        let calc = buffer.split('}').collect::<Vec::<&str>>()[0];
                        ret.push(Token { ttype: TokenType::Calculation, text: calc.to_string() });
                        ret.push(Token { ttype: TokenType::CloseCurly, text: "}".to_string() });
                        ret
                    } else {
                        vec![]
                    },
            ';' => if buffer.is_empty() { vec![Token { ttype: TokenType::SemiColon, text: ";".to_string() }] }
                   else {
                       let mut ret = vec![];
                       let tmp = match buffer.trim() {
                            "table" => Token { ttype: TokenType::Table, text: b.trim().to_string() },
                            "relationship" => Token { ttype: TokenType::Relationship, text: b.trim().to_string() },
                            "value_list" => Token { ttype: TokenType::ValueList, text: b.trim().to_string() },
                            x =>Token { ttype: TokenType::Identifier, text: x.trim().to_string() },
                       };
                       ret.push(tmp);
                       ret.push(Token { ttype: TokenType::SemiColon, text: ";".to_string() });
                       ret
                   }
            ':' =>  if buffer.len() > 1 {
                        let mut tmp = vec![];
                        tmp.push(Token { ttype:TokenType::Identifier, text: b.trim().strip_suffix(':').unwrap().to_string() });
                        tmp.push(Token {ttype: TokenType::Colon, text: ":".to_string() });
                        tmp
                    } else {
                        vec![Token {ttype: TokenType::Colon, text: ":".to_string() }]
                    }
            ',' =>  if buffer.len() > 1 {
                        let mut ret = vec![];
                        let tmp = match buffer.strip_suffix(',').unwrap() {
                            "Required" => Token { ttype: TokenType::Required, text: buffer.trim()
                                .strip_suffix(',')
                                .unwrap()
                                .to_string() },
                            "Unique" => Token { ttype: TokenType::Unique, text: buffer.trim()
                                .strip_suffix(',')
                                .unwrap()
                                .to_string() },
                            "Text" | "Number" | "Date" | "Time" | "Timestamp" | "Container" => 
                                Token { ttype: TokenType::DataType, 
                                    text: b
                                        .strip_suffix(',')
                                        .unwrap()
                                        .to_string() },
                            x => Token { ttype: TokenType::Identifier, 
                                         text: x.trim()
                                             .to_string() }
                        };
                        ret.push(tmp); 
                        ret.push(Token { ttype: TokenType::Comma, text: ",".to_string() });
                        ret
                    } else {
                        vec![Token { ttype: TokenType::Comma, text: ",".to_string() }]
                    }
            _ => vec![]
        };

        if !tmp.is_empty() {
            for t in tmp {
                list.push(t);
            }
            buffer.clear();
        }
    }
        

    for l in &list {
        println!("{:?} : \"{}\"", l.ttype, l.text)
    }

    return list;

}

