use crate::tokens::Token;



pub(crate) type TokensGroups = Vec<Vec<Token>>;
pub(crate) struct Tokonizer(TokensGroups);

impl Tokonizer {
   pub(crate) fn new(line: &str) -> Self {
        let mut tokens = Vec::new();
        let mut special = String::new();
        let mut modkey = false;

        line.chars().for_each(|c| {
            if c == '<' {
                modkey = true;
            } else if c == '>' {
                modkey = false;
                tokens.push(Token::new(&special));
                special.clear();
            } else if modkey {
                special.push_str(&c.to_string())
            } else {
                tokens.push(Token::new(&c.to_string()));
            }
        });

        let mut token_groups = Vec::new();
        let mut tmp: Vec<Token> = Vec::new();

        for token in tokens.into_iter() {
            if token == Token::Deliminator {
                token_groups.push(tmp.clone());
                tmp.clear();
            } else {
                tmp.push(token);
            }
        }
        token_groups.push(tmp);

        Self(token_groups)
    }
   pub(crate) fn get(self) -> TokensGroups {
        self.0
   }
}


