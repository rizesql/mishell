use crate::tokenizer::Token;

#[derive(Debug, Clone)]
pub enum ASTNode{

    Command {
        name: String,
        args: Vec<String>,
    },
    Sequence(Vec<ASTNode>),
    Pipeline{
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    Redirection{
        command: Box<ASTNode>,
        file: String,
        direction: String,
    },
    Logical{
        left: Box<ASTNode>,
        right: Box<ASTNode>,
        operator: String,
    },
    ForLoop{
        variable: String,
        values: Vec<String>,
        body: Box<ASTNode>,
    },
    IfCondition{
        condition: Box<ASTNode>,
        then_branch: Box<ASTNode>,
        else_branch: Option<Box<ASTNode>>
    }
}

pub struct Parser{
    tokens: Vec<Token>,
    position: usize,
}

impl Parser{

    pub fn new(tokens: Vec<Token>) -> Self{
        Parser{tokens, position: 0}
    }

    pub fn parse(&mut self) -> Result<ASTNode,String>{
        self.parse_program()
    }

    

    fn parse_program(&mut self) -> Result<ASTNode,String>{

        let mut commands = Vec::new();

        while self.position < self.tokens.len() {
            commands.push(self.parse_expression()?);

            if self.top() == Some(&Token::Keyword("done".to_string())) 
            || self.top() == Some(&Token::Keyword("fi".to_string()))
            || self.top() == Some(&Token::Keyword("else".to_string()))
            {
                break;
            }

        }

        if commands.len() == 1{
            Ok(commands.pop().unwrap())
        }else{
            Ok(ASTNode::Sequence(commands))
        }

    }

    fn parse_expression(&mut self) -> Result<ASTNode, String>{

        if self.top() == Some(&Token::Keyword("for".to_string())){
            self.parse_for_loop()
        }
        else if self.top() == Some(&Token::Keyword("if".to_string())){
            self.parse_if_statement()
        }else{
            let mut left = self.parse_command()?;

            while let Some(token) = self.top() {
                let clone_token= token.clone();
                match clone_token{
                    Token::Operator(op) if op == "|" =>{
                        self.next();
                        let right = self.parse_command()?;
                        left = ASTNode::Pipeline {
                             left: Box::new(left),
                              right: Box::new(right),
                             };
                    },
                    Token::Operator(op) if op == ">" || op == "<" => {
                        self.next();
                        if let Some(Token::Value(file)) = self.top(){
                            let file_name = file.clone();
                            self.next();
                            left = ASTNode::Redirection { 
                                    command: Box::new(left),
                                    file: file_name,
                                    direction: op.clone()
                                 };
                        } else{
                            return Err("Expected a file after redirection!".to_string());
                        }
                    },
                    Token::Operator(op) if op == "||" || op == "&&" => {
                        self.next();
                        let right = self.parse_command()?;
                        left = ASTNode::Logical { 
                                left:Box::new(left),
                                right: Box::new(right),
                                operator: op.clone() 
                            };
                    }

                    _=>break,
                }
            }
            self.consume_separator()?;
            Ok(left)
        }

    }

    fn parse_command(&mut self) -> Result<ASTNode,String>{
        if let Some(Token::Command(name)) = self.top(){
            let comm_name = name.clone();
            
            self.next();

            let mut args = Vec::new();
            
            while let Some(token) = self.top() {
                match token {
                    Token::Value(arg) | 
                    Token::IntegerLiteral(arg) | 
                    Token::FloatLiteral(arg) | 
                    Token::StringLiteral(arg) => {
                        args.push(arg.clone());
                        self.next(); 
                    }
                    _ => break, 
                }
            }
            

            Ok(ASTNode::Command { name: comm_name, args: args })
        
        } else{
            Err("Expected a command".to_string())
        }
    }

    fn parse_if_statement(&mut self) -> Result<ASTNode, String>{
        self.consume_keyword("if")?;
        let condition = self.parse_expression()?;
        self.consume_keyword("then")?;
        let then_branch = self.parse_program()?;
        let else_branch = if self.top() == Some(&Token::Keyword("else".to_string())){
            self.consume_keyword("else")?;
            Some(Box::new(self.parse_program()?))
        } else{
            None
        };
        self.consume_keyword("fi")?;

        Ok(ASTNode::IfCondition { condition: Box::new(condition), then_branch: Box::new(then_branch),  else_branch })

    }

    fn parse_for_loop(&mut self) -> Result<ASTNode, String>{

        self.consume_keyword("for")?;
        let variable = self.consume_identifier()?;
        self.consume_keyword("in")?;

        let mut values = Vec::new();

        while let Some(Token::Value(value)) = self.top(){
            values.push(value.clone());
            self.next();
        }
        self.consume_separator()?;
        self.consume_keyword("do")?;
        
        let body = self.parse_program()?;
        
        self.consume_keyword("done")?;

        Ok(ASTNode::ForLoop { variable: variable, values: values, body: Box::new(body) })

    }


    fn next(&mut self){
        self.position += 1;
    }

    fn top(&mut self) -> Option<&Token>{
        self.tokens.get(self.position)
    } 
    
    fn consume_separator(&mut self) -> Result<(), String>{
        if let Some(Token::Separator(_)) = self.top(){

            self.next();
            Ok(())
        }else{
            Err("Expected an separator".to_string())
        }

    }

    fn consume_identifier(&mut self) -> Result<String,String>{
        if let Some(Token::Value(id)) = self.top(){
            let identifier = id.clone();
            self.next();
            Ok(identifier)
        }else{
            Err("Expected an identifier".to_string())
        }
    }

    fn consume_keyword(&mut self, keyword: &str) -> Result<(), String>{
        if Some(&Token::Keyword(keyword.to_string())) == self.top(){
            self.next();
            Ok(())     
        }else{
            Err(format!("Expected '{}'",keyword))
        }
    }

}