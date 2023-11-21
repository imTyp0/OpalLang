use std::process::exit;

#[derive(PartialEq, Clone, Hash, Eq, Debug)]
pub enum TokenType{
	Exit,
	Print,
	IntLit,
	StrLit,
	Let,
	Ident,
	Eq,
	Semi,
	OpenParen,
	ClosedParen
}
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Token{
	pub t: TokenType,
	pub value: Option<String>
}

// Looks at and returns the next character
fn peek(i: usize, src: &String) -> (Option<char>, usize){
	if i >= src.len(){
		(Option::None, 0)
	}
	else{
		(src.chars().nth(i), i)
	}
}

// Consumes and returns the next character
fn consume(mut i: usize, src: &str) -> (char, usize){
	let c = src.chars().nth(i).unwrap();
	i += 1;
	(c, i)
}

// Main function to turn input file into tokens
pub fn tokenize(src: String) -> Vec<Token>{
    let mut i = 0usize;
    let mut c;
    let mut tokens: Vec<Token> = vec![];
	let mut buf = String::new();

	// While there are still characters in the file
	while peek(i, &src).0.is_some(){
		// Checks for keywords/identifiers
		if peek(i, &src).0.unwrap().is_alphabetic(){
			(c, i) = consume(i, &src);
            buf.push(c);
			while peek(i, &src).0.is_some() && peek(i, &src).0.unwrap().is_alphanumeric(){
				(c, i) = consume(i, &src);
                buf.push(c);
			}
			// Checks for different keywords
			if buf == "exit"{
				tokens.push(Token { t: (TokenType::Exit), value: (Option::None) });
				buf.clear();
				continue
			}
			else if buf == "print"{
				tokens.push(Token { t: (TokenType::Print), value: (Option::None) });
				buf.clear();
				continue
			}
			else if buf == "let"{ // Not implemented
				tokens.push(Token { t: (TokenType::Let), value: (Option::None) });
				buf.clear();
				continue
			}
			else{ // Not implemented (ish)
				tokens.push(Token { t: (TokenType::Ident), value: (Some(buf.clone()))});
				buf.clear();
				continue;
			}
		}
		// Check for integer literals
		else if peek(i, &src).0.unwrap().is_ascii_digit(){
			(c, i) = consume(i, &src);
            buf.push(c);
			while peek(i, &src).0.is_some() && peek(i, &src).0.unwrap().is_ascii_digit(){
				(c, i) = consume(i, &src);
                buf.push(c);
			}

			tokens.push(Token { t: (TokenType::IntLit), value: (Some(buf.clone())) });
			buf.clear();
			continue;
		}
        // Check for string literal
        else if peek(i, &src).0.unwrap() == '"'{
            i = consume(i, &src).1;
            while !matches!(peek(i, &src).0, Some('"')){
                (c, i) = consume(i, &src);
                buf.push(c);
            }
            i = consume(i, &src).1;
            tokens.push(Token{t: TokenType::StrLit, value: Some(buf.clone())});
            buf.clear();
            continue;
        }
		// Checks for different syntax elements
		else if peek(i, &src).0.unwrap() == ';'{
			i = consume(i, &src).1;
			tokens.push(Token { t: (TokenType::Semi), value: (Option::None) });
			buf.clear();
			continue;
		}
		else if peek(i, &src).0.unwrap() == '('{
			i = consume(i, &src).1;
			tokens.push(Token { t: (TokenType::OpenParen), value: (Option::None) });
			buf.clear();
			continue;
		}
		else if peek(i, &src).0.unwrap() == ')'{
			i = consume(i, &src).1;
			tokens.push(Token { t: (TokenType::ClosedParen), value: (Option::None) });
			buf.clear();
			continue;
		}
		// else if peek(i, src).0.unwrap() == '"'{
		// 	consume();
		// 	tokens.push(Token { t: (TokenType::Quote), value: (Option::None) });
		// 	buf.clear();
		// 	continue;
		// }
		else if peek(i, &src).0.unwrap() == '='{
			i = consume(i, &src).1;
			tokens.push(Token { t: (TokenType::Eq), value: (Option::None) });
			buf.clear();
			continue;
		}
		else if peek(i, &src).0.unwrap().is_whitespace(){
			i = consume(i, &src).1;
			continue;
		}
		else{
			println!("Unknown token found: {}", buf);
			exit(1);
		}
	}
	tokens
}