

pub struct SlkLexerToken {
    pub token_type: TokenType,
    pub char_start: usize,
    pub char_end: usize,
    pub file_name: String,
}


impl SlkLexerToken {
    pub fn token_len(&self) -> usize {
        self.char_end-self.char_start
    }
}

pub enum TokenType {
    KeyWord(KeyWord),
    Literal(Literal),
    Punctuation(Punctuation),
    Delimitor(Delimitor),
    Symbol(Symbol),
    Comment(Comment),
    Unknown(String),
}

pub enum KeyWord {
    VarDeclare,
    FuncDeclare,
    StructDeclare,
    If,
    Loop,
    Break,
    Return,
}

pub enum Literal {
    String(String),
    Integer(String),
    Float{
        int: String,
        deci: String,
    }
}
