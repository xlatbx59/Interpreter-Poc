//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/Interpreter-Poc
use crate::error::*;
use crate::mnemonics::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType{
    Label, DecLiteral, Instruction,
    EndOfLine, HexLiteral, Colon
}

#[derive(Debug)]
pub struct Lexeme{
    _lexeme: Option<String>,
    _tok_type: TokenType
}

#[derive(Debug, Clone)]
pub struct Token{
    pub token: String,
    pub token_type: TokenType,
}

fn new_token(token: String, category: TokenType) ->Token{
    return Token{token, token_type: category};
}

//https://youtu.be/6db1BSMFUGE?si=eCN4hIbdPSEnEKNz
pub fn lexapro_doesnt_work(source_line: &str) -> Result<Vec<Token>, IaTokenError>{
    let mut line: usize = 1;
    let mut lexemes: Vec<Token> = Vec::new();
    let iterator = &mut source_line.char_indices().peekable();
    let hit_non_eol = |value: &(usize, char)|{      //Did it hit something else than end of line
        if  value.1 == '\n' || value.1 == '\r'{
            return false
        };
        true
    };
    let hit_non_separator = |value: &(usize, char)|{ //Did it hit something else than a separator
        if value.1 == ':' || value.1 == ' ' || value.1 == '\t' || value.1 == ',' || value.1 == '(' || value.1 == ')'{
            return false
        };
        hit_non_eol(value)
    };

    while let Some(indice) = iterator.next(){
        let mut current_token: String = String::new();
        
        match indice.1{
            '\r' | ' ' | '\t' => continue,
            ';' => while let Some(_) = iterator.next_if(hit_non_eol){},
            ':' => lexemes.push(new_token(String::from(indice.1), TokenType::Colon)),
            '\n' => {
                lexemes.push(new_token(current_token, TokenType::EndOfLine));
                line += 1;
            }
            '0'=> {             //If it starts with 0, it has to be an hexadecimal
                let mut digit_len: i8 = 0;

                let Some(spec) = iterator.next() else {
                    current_token.push(indice.1);
                    lexemes.push(new_token(current_token, TokenType::DecLiteral));
                    break;          //If we get there, this means we reached the end of the file and the token is just '0'
                };
                if spec.1 != 'x' {
                    return Err(throw_token_error(current_token, line, source_line.char_indices().offset()));
                }
                while let Some(hex) = iterator.next_if(hit_non_separator){
                    if hex.1.is_ascii_hexdigit() == false{
                        break;
                    }
                    digit_len += 1;
                    current_token.push(hex.1);
                }
                if digit_len < 1 || digit_len > 10{
                    return Err(throw_token_error(current_token, line, source_line.char_indices().offset()));
                }
                lexemes.push(new_token(current_token, TokenType::HexLiteral));
            }
            _ => {
                let label_allowed  = |value: &(usize, char)|{ //Did it hit something else than a separator
                    if !value.1.is_ascii_alphanumeric() && value.1 != '_'{
                        return false
                    };
                    true
                };

                if !indice.1.is_ascii_alphanumeric() && indice.1 != '_'{
                    return Err(throw_token_error(current_token, line, source_line.char_indices().offset()));
                };
                current_token.push(indice.1);

                //Consumes every digits until there's a separator
                let token_type: TokenType = if indice.1.is_ascii_digit(){
                    while let Some(next_char) = iterator.next_if(hit_non_separator){
                        if !next_char.1.is_ascii_digit(){
                            return Err(throw_token_error(current_token, line, source_line.char_indices().offset()));
                        }
                        current_token.push(next_char.1);
                    }
                    TokenType::DecLiteral
                }else{
                    while let Some(next_char) = iterator.next_if(label_allowed){
                        current_token.push(next_char.1);
                    }
                    //If it's not a mnemonic it's a label
                    if IA_MNEMONIC.contains(&current_token.as_str()) {
                        TokenType::Instruction
                    }else {
                        TokenType::Label
                    }
                };
                lexemes.push(new_token(String::from(current_token), token_type))
            }
        };
    }
    return Ok(lexemes)
}