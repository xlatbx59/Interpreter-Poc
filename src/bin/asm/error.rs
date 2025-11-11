//Author: xlatbx59
//Github profile: https://github.com/xlatbx59
//Link to repo: https://github.com/xlatbx59/Interpreter-Poc
use core::fmt;
use crate::lexapro::{Token, TokenType};

pub enum IaGrammarErrorCode{
    EndOfFile, SyntaxError
}

pub enum IaError{
    Gramerr(IaGrammarError), Tokerr(IaTokenError)
}
pub struct IaTokenError{
    token: String,
    line: usize,
    colone: usize,
}

pub struct IaGrammarError{
    token: Token,
    prev_token: Option<Token>,
    line: usize,
}

pub fn throw_endfile_error(token: Token, line: usize) -> IaGrammarError{
    IaGrammarError{token, prev_token: None, line}
}
pub fn throw_syntax_error(token: Token, prev_token: Token, line: usize) -> IaGrammarError{
    IaGrammarError{token, prev_token: Some(prev_token), line}
}
pub fn throw_token_error(token: String, line: usize, colone: usize) -> IaTokenError{
    IaTokenError{token, line, colone}
}

impl fmt::Display for IaGrammarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let toktype = match self.token.token_type{
            TokenType::Instruction => "instruction",
            TokenType::Label => "label",
            TokenType::EndOfLine => "end of line",
            TokenType::Colon => "colon",
            TokenType::DecLiteral | TokenType::HexLiteral => "immediate"
        };
        
        let (prevtoktype, prevtok) = if let Some(prevtoken) = self.prev_token.clone(){
            match prevtoken.token_type{
                TokenType::Instruction => ("instruction", prevtoken.token),
                TokenType::Label => ("label", prevtoken.token),
                TokenType::EndOfLine => ("end of line", prevtoken.token),
                TokenType::Colon => ("colon", prevtoken.token),
                TokenType::DecLiteral | TokenType::HexLiteral => ("immediate", prevtoken.token)
            }
        }else{
            return write!(f, "[-]Can't parse further bc we reached the end of the file, last token:\r\ntype {}\r\nlexeme {}", toktype, self.token.token)
        };

        write!(f, "[-]Oh my God, can't you code properly???????\r\nLine: On line {}, there's a {} \"{}\" after a {} \"{}\"", self.line, toktype, self.token.token, prevtoktype, prevtok)
    }
}
impl fmt::Display for IaTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[-]This token is invalid: '{}', line: {}, colone {}", self.token, self.line, self.colone)
    }
}