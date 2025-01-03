// struct Tokens<'a>;

// peg::parser! {
//     grammar token_parser<'a>(parser_options: &ParserOptions, source_info: &SourceInfo) for Tokens<'a> {
//         pub(crate) rule program() -> ast::Program =
//             linebreak() c:complete_commands() linebreak() { ast::Program { complete_commands: c } } /
//             linebreak() { ast::Program { complete_commands: vec![] } }

//         rule complete_commands() -> Vec<ast::CompleteCommand> =
//             c:complete_command() ++ newline_list()

//         rule complete_command() -> ast::CompleteCommand =
//             first:seq_step() remainder:(s:separator_op() l:seq_step() { (s, l) })* last_sep:separator_op()? {
//                 let mut and_ors = vec![first];
//                 let mut seps = vec![];

//                 for (sep, ao) in remainder.into_iter() {
//                     seps.push(sep);
//                     and_ors.push(ao);
//                 }

//                 // N.B. We default to synchronous if no separator op is given.
//                 seps.push(last_sep.unwrap_or(SeparatorOperator::Sequence));

//                 let mut items = vec![];
//                 for (i, ao) in and_ors.into_iter().enumerate() {
//                     items.push(ast::CompoundListItem(ao, seps[i].clone()));
//                 }

//                 ast::CompoundList(items)
//             }

//         rule seq_step() -> ast::AndOrList =
//             first:pipeline() additional:_seq_item()* { ast::AndOrList { first, additional } }

//         rule _seq_item() -> ast::AndOr =
//             op:_and_or_op() linebreak() p:pipeline() { op(p) }

//         rule _seq_separator() -> fn(ast::Pipeline) -> ast::AndOr =
//             specific_operator("&&") { ast::AndOr::And } /
//             specific_operator("||") { ast::AndOr::Or }

//         rule pipeline() -> ast::Pipeline =
//             bang:bang()? seq:pipe_seq() { ast::Pipeline { bang: bang.is_some(), seq } }
//         rule bang() -> bool = specific_word("!") { true }

//         pub(crate) rule pipe_seq() -> Vec<ast::Command> =
//             c:command() ++ (specific_operator("|") linebreak()) { c }

//         // N.B. We needed to move the function definition branch up to avoid conflicts with array assignment syntax.
//         rule command() -> ast::Command =
//             f:function_definition() { ast::Command::Function(f) } /
//             c:simple_command() { ast::Command::Simple(c) } /
//             c:compound_command() r:redirect_list()? { ast::Command::Compound(c, r) } /
//             expected!("command")

//         // N.B. The arithmetic command is a non-sh extension.
//         // N.B. The arithmetic for clause command is a non-sh extension.
//         pub(crate) rule compound_command() -> ast::CompoundCommand =
//             a:arithmetic_command() { ast::CompoundCommand::Arithmetic(a) } /
//             b:brace_group() { ast::CompoundCommand::BraceGroup(b) } /
//             s:subshell() { ast::CompoundCommand::Subshell(s) } /
//             f:for_clause() { ast::CompoundCommand::ForClause(f) } /
//             i:if_clause() { ast::CompoundCommand::IfClause(i) } /
//             w:while_clause() { ast::CompoundCommand::WhileClause(w) } /
//             u:until_clause() { ast::CompoundCommand::UntilClause(u) } /
//             c:arithmetic_for_clause() { ast::CompoundCommand::ArithmeticForClause(c) } /
//             expected!("compound command")

//         pub(crate) rule arithmetic_command() -> ast::ArithmeticCommand =
//             specific_operator("(") specific_operator("(") expr:arithmetic_expression() specific_operator(")") specific_operator(")") {
//                 ast::ArithmeticCommand { expr }
//             }

//         pub(crate) rule arithmetic_expression() -> ast::UnexpandedArithmeticExpr =
//             raw_expr:$(arithmetic_expression_piece()*) { ast::UnexpandedArithmeticExpr { value: raw_expr } }

//         rule arithmetic_expression_piece() =
//             specific_operator("(") (!specific_operator(")") arithmetic_expression_piece())* specific_operator(")") {} /
//             !arithmetic_end() [_] {}

//         // TODO: evaluate arithmetic end; the semicolon is used in arithmetic for loops.
//         rule arithmetic_end() -> () =
//             specific_operator(")") specific_operator(")") {} /
//             specific_operator(";") {}

//         rule subshell() -> ast::SubshellCommand =
//             specific_operator("(") c:compound_list() specific_operator(")") { ast::SubshellCommand(c) }

//         rule compound_list() -> ast::CompoundList =
//             linebreak() first:seq_step() remainder:(s:separator() l:seq_step() { (s, l) })* last_sep:separator()? {
//                 let mut and_ors = vec![first];
//                 let mut seps = vec![];

//                 for (sep, ao) in remainder.into_iter() {
//                     seps.push(sep.unwrap_or(SeparatorOperator::Sequence));
//                     and_ors.push(ao);
//                 }

//                 // N.B. We default to synchronous if no separator op is given.
//                 let last_sep = last_sep.unwrap_or(None);
//                 seps.push(last_sep.unwrap_or(SeparatorOperator::Sequence));

//                 let mut items = vec![];
//                 for (i, ao) in and_ors.into_iter().enumerate() {
//                     items.push(ast::CompoundListItem(ao, seps[i].clone()));
//                 }

//                 ast::CompoundList(items)
//             }

//         rule for_clause() -> ast::ForClauseCommand =
//             specific_word("for") n:name() linebreak() specific_word("in") w:wordlist()? sequential_sep() d:do_group() {
//                 ast::ForClauseCommand { variable_name: n.to_owned(), values: w, body: d }
//             } /
//             specific_word("for") n:name() sequential_sep()? d:do_group() {
//                 ast::ForClauseCommand { variable_name: n.to_owned(), values: None, body: d }
//             }

//         // N.B. The arithmetic for loop is a non-sh extension.
//         rule arithmetic_for_clause() -> ast::ArithmeticForClauseCommand =
//             specific_word("for")
//             specific_operator("(") specific_operator("(")
//                 initializer:arithmetic_expression()? specific_operator(";")
//                 condition:arithmetic_expression()? specific_operator(";")
//                 updater:arithmetic_expression()?
//             specific_operator(")") specific_operator(")")
//             sequential_sep()
//             body:do_group() {
//                 ast::ArithmeticForClauseCommand { initializer, condition, updater, body }
//             }

//         // N.B. For some reason we seem to need to allow a select subset
//         // of unescaped operators in regex words.
//         rule regex_word() -> ast::Word =
//             value:$((!specific_word("]]") regex_word_piece())+) {
//                 ast::Word { value }
//             }

//         rule regex_word_piece() =
//             word() {} /
//             specific_operator("|") {} /
//             specific_operator("(") parenthesized_regex_word()* specific_operator(")") {}

//         rule parenthesized_regex_word() =
//             regex_word_piece() /
//             !specific_operator(")") !specific_operator("]]") [_]

//         rule name() -> &'input str =
//             w:[Token::Word(_, _)] { w.to_str() }

//         // TODO: validate if this should call non_reserved_word() or word()
//         rule wordlist() -> Vec<ast::Word> =
//             (w:non_reserved_word() { ast::Word::from(w) })+

//         rule if_clause() -> ast::IfClauseCommand =
//             specific_word("if") condition:compound_list() specific_word("then") then:compound_list() elses:else_part()? specific_word("fi") {
//                 ast::IfClauseCommand {
//                     condition,
//                     then,
//                     elses,
//                 }
//             }

//         rule else_part() -> Vec<ast::ElseClause> =
//             cs:_conditional_else_part()+ u:_unconditional_else_part()? {
//                 let mut parts = vec![];
//                 for c in cs.into_iter() {
//                     parts.push(c);
//                 }

//                 if let Some(uncond) = u {
//                     parts.push(uncond);
//                 }

//                 parts
//             } /
//             e:_unconditional_else_part() { vec![e] }

//         rule _conditional_else_part() -> ast::ElseClause =
//             specific_word("elif") condition:compound_list() specific_word("then") body:compound_list() {
//                 ast::ElseClause { condition: Some(condition), body }
//             }

//         rule _unconditional_else_part() -> ast::ElseClause =
//             specific_word("else") body:compound_list() {
//                 ast::ElseClause { condition: None, body }
//              }

//         rule while_clause() -> ast::WhileOrUntilClauseCommand =
//             specific_word("while") c:compound_list() d:do_group() { ast::WhileOrUntilClauseCommand(c, d) }

//         rule until_clause() -> ast::WhileOrUntilClauseCommand =
//             specific_word("until") c:compound_list() d:do_group() { ast::WhileOrUntilClauseCommand(c, d) }

//         // N.B. Non-sh extensions allows use of the 'function' word to indicate a function definition.
//         rule function_definition() -> ast::FunctionDefinition =
//             specific_word("function")? fname:fname() specific_operator("(") specific_operator(")") linebreak() body:function_body() {
//                 ast::FunctionDefinition { fname: fname.to_owned(), body, source: source_info.source.clone() }
//             } /
//             specific_word("function") fname:fname() linebreak() body:function_body() {
//                 ast::FunctionDefinition { fname: fname.to_owned(), body, source: source_info.source.clone() }
//             } /
//             expected!("function definition")

//         rule function_body() -> ast::FunctionBody =
//             c:compound_command() r:redirect_list()? { ast::FunctionBody(c, r) }

//         rule fname() -> &'input str =
//             // Special-case: don't allow it to end with an equals sign, to avoid the challenge of
//             // misinterpreting certain declaration assignments as function definitions.
//             // TODO: Find a way to make this still work without requiring this targeted exception.
//             w:[Token::Word(word, _) if !word.ends_with('=')] { w.to_str() }

//         rule brace_group() -> ast::BraceGroupCommand =
//             specific_word("{") c:compound_list() specific_word("}") { ast::BraceGroupCommand(c) }

//         rule do_group() -> ast::DoGroupCommand =
//             specific_word("do") c:compound_list() specific_word("done") { ast::DoGroupCommand(c) }

//         rule simple_command() -> ast::SimpleCommand =
//             prefix:cmd_prefix() word_and_suffix:(word_or_name:cmd_word() suffix:cmd_suffix()? { (word_or_name, suffix) })? {
//                 match word_and_suffix {
//                     Some((word_or_name, suffix)) => {
//                         ast::SimpleCommand { prefix: Some(prefix), word_or_name: Some(ast::Word::from(word_or_name)), suffix }
//                     }
//                     None => {
//                         ast::SimpleCommand { prefix: Some(prefix), word_or_name: None, suffix: None }
//                     }
//                 }
//             } /
//             word_or_name:cmd_name() suffix:cmd_suffix()? {
//                 ast::SimpleCommand { prefix: None, word_or_name: Some(ast::Word::from(word_or_name)), suffix } } /
//             expected!("simple command")

//         rule cmd_name() -> &'input Token =
//             non_reserved_word()

//         rule cmd_word() -> &'input Token =
//             !assignment_word() w:non_reserved_word() { w }

//         rule cmd_prefix() -> ast::CommandPrefix =
//             p:(
//                 i:io_redirect() { ast::CommandPrefixOrSuffixItem::IoRedirect(i) } /
//                 assignment_and_word:assignment_word() {
//                     let (assignment, word) = assignment_and_word;
//                     ast::CommandPrefixOrSuffixItem::AssignmentWord(assignment, word)
//                 }
//             )+ { ast::CommandPrefix(p) }

//         rule cmd_suffix() -> ast::CommandSuffix =
//             s:(
//                 i:io_redirect() {
//                     ast::CommandPrefixOrSuffixItem::IoRedirect(i)
//                 } /
//                 assignment_and_word:assignment_word() {
//                     let (assignment, word) = assignment_and_word;
//                     ast::CommandPrefixOrSuffixItem::AssignmentWord(assignment, word)
//                 } /
//                 w:word() {
//                     ast::CommandPrefixOrSuffixItem::Word(ast::Word::from(w))
//                 }
//             )+ { ast::CommandSuffix(s) }

//         rule redirect_list() -> ast::RedirectList =
//             r:io_redirect()+ { ast::RedirectList(r) } /
//             expected!("redirect list")

//     // N.B. here strings are extensions to the POSIX standard.
//         rule io_redirect() -> ast::IoRedirect =
//             n:io_number()? f:io_file() {
//                     let (kind, target) = f;
//                     ast::IoRedirect::File(n, kind, target)
//                 } /
//             n:io_number()? h:io_here() { ast::IoRedirect::HereDocument(n, h) } /
//             expected!("I/O redirect")

//         // N.B. Process substitution forms are extensions to the POSIX standard.
//         rule io_file() -> (ast::IoFileRedirectKind, ast::IoFileRedirectTarget) =
//             specific_operator("<")  f:io_filename() { (ast::IoFileRedirectKind::Read, f) } /
//             specific_operator("<&") f:io_filename_or_fd() { (ast::IoFileRedirectKind::DuplicateInput, f) } /
//             specific_operator(">")  f:io_filename() { (ast::IoFileRedirectKind::Write, f) } /
//             specific_operator(">&") f:io_filename_or_fd() { (ast::IoFileRedirectKind::DuplicateOutput, f) } /
//             specific_operator(">>") f:io_filename() { (ast::IoFileRedirectKind::Append, f) } /
//             specific_operator("<>") f:io_filename() { (ast::IoFileRedirectKind::ReadAndWrite, f) } /
//             specific_operator(">|") f:io_filename() { (ast::IoFileRedirectKind::Clobber, f) }

//         rule io_filename_or_fd() -> ast::IoFileRedirectTarget =
//             fd:io_fd() { ast::IoFileRedirectTarget::Fd(fd) } /
//             io_filename()

//         rule io_fd() -> u32 =
//             w:[Token::Word(_, _)] {? w.to_str().parse().or(Err("io_fd u32")) }

//         rule io_filename() -> ast::IoFileRedirectTarget =
//             f:filename() { ast::IoFileRedirectTarget::Filename(ast::Word::from(f)) }

//         rule filename() -> &'input Token =
//             word()

//         pub(crate) rule io_here() -> ast::IoHereDocument =
//            specific_operator("<<-") here_tag:here_tag() doc:[_] closing_tag:here_tag() {
//                 let requires_expansion = !here_tag.to_str().contains(['\'', '"', '\\']);
//                 ast::IoHereDocument {
//                     remove_tabs: true,
//                     requires_expansion,
//                     here_end: ast::Word::from(here_tag),
//                     doc: ast::Word::from(doc)
//                 }
//             } /
//             specific_operator("<<") here_tag:here_tag() doc:[_] closing_tag:here_tag() {
//                 let requires_expansion = !here_tag.to_str().contains(['\'', '"', '\\']);
//                 ast::IoHereDocument {
//                     remove_tabs: false,
//                     requires_expansion,
//                     here_end: ast::Word::from(here_tag),
//                     doc: ast::Word::from(doc)
//                 }
//             }

//         rule here_tag() -> &'input Token =
//             word()

//         rule process_substitution() -> (ast::ProcessSubstitutionKind, ast::SubshellCommand) =
//             specific_operator("<") s:subshell() { (ast::ProcessSubstitutionKind::Read, s) } /
//             specific_operator(">") s:subshell() { (ast::ProcessSubstitutionKind::Write, s) }

//         rule newline_list() -> () =
//             newline()+ {}

//         rule linebreak() -> () =
//             quiet! {
//                 newline()* {}
//             }

//         rule separator_op() -> ast::SeparatorOperator =
//             specific_operator("&") { ast::SeparatorOperator::Async } /
//             specific_operator(";") { ast::SeparatorOperator::Sequence }

//         rule separator() -> Option<ast::SeparatorOperator> =
//             s:separator_op() linebreak() { Some(s) } /
//             newline_list() { None }

//         rule sequential_sep() -> () =
//             specific_operator(";") linebreak() /
//             newline_list()

//         //
//         // Token interpretation
//         //

//         rule non_reserved_word() -> &'input Token =
//             !reserved_word() w:word() { w }

//         rule word() -> &'input Token =
//             [Token::Word(_, _)]

//         rule reserved_word() -> &'input Token =
//             [Token::Word(w, _) if matches!(w.as_str(),
//                 "!" |
//                 "{" |
//                 "}" |
//                 "case" |
//                 "do" |
//                 "done" |
//                 "elif" |
//                 "else" |
//                 "esac" |
//                 "fi" |
//                 "for" |
//                 "if" |
//                 "in" |
//                 "then" |
//                 "until" |
//                 "while"
//             )]

//         rule newline() -> () = quiet! {
//             specific_operator("\n") {}
//         }

//         pub(crate) rule assignment_word() -> (ast::Assignment, ast::Word) =
//             [Token::Word(w, _)] {?
//                 let parsed = parse_assignment_word(w.as_str())?;
//                 Ok((parsed, ast::Word { value: w.to_owned() }))
//             }

//         rule array_elements() -> Vec<&'input String> =
//             e:array_element()*

//         rule array_element() -> &'input String =
//             linebreak() [Token::Word(e, _)] linebreak() { e }

//         // N.B. An I/O number must be a string of only digits, and it must be
//         // followed by a '<' or '>' character (but not consume them).
//         rule io_number() -> u32 =
//             [Token::Word(w, _) if w.chars().all(|c: char| c.is_ascii_digit())]
//             &([Token::Operator(o, _) if o.starts_with('<') || o.starts_with('>')]) {
//                 w.parse().unwrap()
//             }

//         //
//         // Helpers
//         //
//         rule specific_operator(expected: &str) -> &'input Token =
//             [Token::Operator(w, _) if w.as_str() == expected]

//         rule specific_word(expected: &str) -> &'input Token =
//             [Token::Word(w, _) if w.as_str() == expected]
//     }
// }

// peg::parser! {
//     grammar assignments() for str {
//         pub(crate) rule name_and_scalar_value() -> ast::Assignment =
//             nae:name_and_equals() value:scalar_value() {
//                 let (name, append) = nae;
//                 ast::Assignment { name, value, append }
//             }

//         pub(crate) rule name_and_equals() -> (ast::AssignmentName, bool) =
//             name:name() append:("+"?) "=" {
//                 (name, append.is_some())
//             }

//         pub(crate) rule literal_array_element() -> (Option<String>, String) =
//             "[" inner:$((!"]" [_])*) "]=" value:$([_]*) {
//                 (Some(inner.to_owned()), value.to_owned())
//             } /
//             value:$([_]+) {
//                 (None, value.to_owned())
//             }

//         rule name() -> ast::AssignmentName =
//             aen:array_element_name() {
//                 let (name, index) = aen;
//                 ast::AssignmentName::ArrayElementName(name.to_owned(), index.to_owned())
//             } /
//             name:scalar_name() {
//                 ast::AssignmentName::VariableName(name.to_owned())
//             }

//         rule array_element_name() -> (&'input str, &'input str) =
//             name:scalar_name() "[" ai:array_index() "]" { (name, ai) }

//         rule array_index() -> &'input str =
//             $((![']'] [_])*)

//         rule scalar_name() -> &'input str =
//             $(alpha_or_underscore() non_first_variable_char()*)

//         rule non_first_variable_char() -> () =
//             ['_' | '0'..='9' | 'a'..='z' | 'A'..='Z'] {}

//         rule alpha_or_underscore() -> () =
//             ['_' | 'a'..='z' | 'A'..='Z'] {}

//         rule scalar_value() -> ast::AssignmentValue =
//             v:$([_]*) { ast::AssignmentValue::Scalar(ast::Word { value: v.to_owned() }) }
//     }
// }

use regex::Regex;

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

pub fn tokenizer(input: &str) -> Vec<Token> {
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
                    tokens.push(clasify_tokens(&current_token));
                    current_token.clear();
                }
            }
            ';' => {
                if !current_token.is_empty() {
                    tokens.push(clasify_tokens(&current_token));
                    current_token.clear();
                }
                current_token.push(character);
            }

            _ => current_token.push(character),
        }
    }

    tokens
}

pub fn clasify_tokens(token: &str) -> Token {
    let int_regex = Regex::new(r"^[+-]?\d+$").unwrap();
    let float_regex = Regex::new(r"^[+-]?(\d+\.\d*|\.\d+)$").unwrap();
    let string_regex = Regex::new(r#"^"([^"\\]|\\.)*"$"#).unwrap();

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
        "ls" | "echo" | "grep" | "cat" | "find" | "cd" | "mv" | "rm" | "mkdir" | "exec"
        | "exit" => Token::Command(token.to_string()),
        _ => Token::Value(token.to_string()),
    }
}

pub fn debug_tokens(tokens: &[Token]) {
    for (i, token) in tokens.iter().enumerate() {
        println!("Token {}: {:?}", i, token);
    }
}
