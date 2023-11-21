use crate::tokenization::*;
use std::{
    sync::{Mutex, OnceLock},
    process::exit
};
#[derive(Clone, Debug)]
pub enum Expr{
    NodeExprIntLit(Token),
    NodeExprIdent(Token),
    NodeExprStrLit(Token)
}
#[derive(Clone)]
pub enum Stmts{
    NodeStmtExit{expr: NodeExpr},
    NodeStmtLet{ident: Token, expr: NodeExpr},
    NodeStmtPrint{expr: NodeExpr}
}
#[derive(Clone, Debug)]
pub struct NodeExpr{
    pub var: Expr
}
#[derive(Clone)]
pub struct NodeStmt{
   pub var: Stmts
}

pub struct NodeProg{
    pub stmts: Vec<NodeStmt>
}

// Global variables workaround
struct Index{
	val: usize
}

struct Tokens{
	val: Vec<Token>
}

fn get_global_index() -> &'static Mutex<Index>{
	static INSTANCE: OnceLock<Mutex<Index>> = OnceLock::new();
	INSTANCE.get_or_init(|| Mutex::new(Index{val: 0}))
}

fn get_global_tokens() -> &'static Mutex<Tokens>{
	static INSTANCE: OnceLock<Mutex<Tokens>> = OnceLock::new();
	INSTANCE.get_or_init(|| Mutex::new(Tokens{val: vec![]}))
}

// Looks at and returns the token at a certain offset
fn peek(offset: usize) -> Option<Token>{
	let i = get_global_index().lock().unwrap();
	let t = get_global_tokens().lock().unwrap();
    
    if i.val + offset >= t.val.len(){
		Option::None
	}
	else{
		Some(t.val[i.val + offset].clone())
	}
}

// Consumes and returns the next token
fn consume() -> Token{
	let mut i = get_global_index().lock().unwrap();
	let t = get_global_tokens().lock().unwrap();

	let o = &t.val[i.val];
	i.val += 1;
	o.clone()
}

// Parses an expression (integer litteral/identifier)
fn parse_expr() -> Option<NodeExpr>{
    if peek(0).is_some() && peek(0).unwrap().t == TokenType::IntLit{
        Some(NodeExpr {var: Expr::NodeExprIntLit(consume())})
    }
    else if peek(0).is_some() && peek(0).unwrap().t == TokenType::StrLit{
        Some(NodeExpr{var: Expr::NodeExprStrLit(consume())})
    }
    else if peek(0).is_some() && peek(0).unwrap().t == TokenType::Ident{
        Some(NodeExpr{var: Expr::NodeExprIdent(consume())})
    }
    else{
        None
    }
}

// Parses a statement, so far: exit(integer_litteral); and let identifier = integer_litteral;
fn parse_stmt() -> Option<NodeStmt>{
    let stmt_exit: Stmts;
    let stmt_let: Stmts;
    let stmt_print: Stmts;
    // Check for `exit(`
    if peek(0).unwrap().t == TokenType::Exit && peek(1).is_some() && peek(1).unwrap().t == TokenType::OpenParen{
        consume();
        consume();
        // Check for a valid expression
        if let Some(node_expr) = parse_expr(){
            stmt_exit = Stmts::NodeStmtExit { expr: node_expr};
        } else{
            println!("Invalid expression.");
            exit(1);
        }
        // If so, do we have a closed parenthese
        if peek(0).is_some() && peek(0).unwrap().t == TokenType::ClosedParen{
            consume();
        } else{
            println!("Missing closing parenthese.");
            exit(1);
        }
        // If so, do we have a semi colon
        if peek(0).is_some() && peek(0).unwrap().t == TokenType::Semi{
            consume();
        } else{
            println!("Missing semi colon.");
            exit(1);
        }
        return Some(NodeStmt{var: stmt_exit});
    }
    // Check for `let identifier =` 
    else if 
        peek(0).is_some() && peek(0).unwrap().t == TokenType::Let &&
        peek(1).is_some() && peek(1).unwrap().t == TokenType::Ident &&
        peek(2).is_some() && peek(2).unwrap().t == TokenType::Eq
    {
        consume();
        let ident = consume();
        consume();

        // Check for a valid expression
        if let Some(expr) = parse_expr(){
            stmt_let = Stmts::NodeStmtLet {ident, expr};
        } else{
            println!("Invalid expression.");
            exit(1);
        }
        // Check for a semi colon
        if peek(0).is_some() && peek(0).unwrap().t == TokenType::Semi{
            consume();
        } else{
            println!("Expected a semi colon.");
            exit(1);
        }
        return Some(NodeStmt{var: stmt_let});
    }
    // Check for `print(`
    else if peek(0).unwrap().t == TokenType::Print && peek(1).is_some() && peek(1).unwrap().t == TokenType::OpenParen{
        consume();
        consume();
                // Check for a valid expression
                if let Some(node_expr) = parse_expr(){
                    stmt_print = Stmts::NodeStmtPrint {expr: node_expr};
                } else{
                    println!("Invalid expression.");
                    exit(1);
                }
                // If so, do we have a closing parenthese
                if peek(0).is_some() && peek(0).unwrap().t == TokenType::ClosedParen{
                    consume();
                } else{
                    println!("Missing closing parenthese.");
                    exit(1);
                }
                // If so, do we have a semi colon
                if peek(0).is_some() && peek(0).unwrap().t == TokenType::Semi{
                    consume();
                } else{
                    println!("Missing semi colon.");
                    exit(1);
                }
                return Some(NodeStmt{var: stmt_print});
    }
    None
}

// Parses the array of tokens into statements
pub fn parse_prog(tokens: Vec<Token>) -> Option<NodeProg>{
    get_global_tokens().lock().unwrap().val = tokens;
    
    let mut prog = NodeProg {stmts: vec![]};
    // While we still have tokens
    while peek(0).is_some(){
        // Check for a valid statement
        if let Some(stmt) = parse_stmt(){
            prog.stmts.push(stmt);
        } else{
            println!("Invalid statement");
            exit(1);
        }
    }
    Some(prog)
}