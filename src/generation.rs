use crate::parser::*;
use std::process::exit;
use std::sync::{Mutex, OnceLock};
use std::fmt::Write;
use std::collections::HashMap;

#[derive(Clone)]
struct Variable{
	stack_loc: usize
}
struct StackSize{
	val: usize
}

fn get_global_stack_size() -> &'static Mutex<StackSize>{
	static INSTANCE: OnceLock<Mutex<StackSize>> = OnceLock::new();
	INSTANCE.get_or_init(|| Mutex::new(StackSize{val: 0}))
}


fn push(reg: &str, output: &mut String){
	get_global_stack_size().lock().unwrap().val += 1;
	_ = write!(output, "    push {reg}\n");
}

fn pop(reg: &str, output: &mut String){
	get_global_stack_size().lock().unwrap().val -= 1;
	_ = write!(output, "    pop {reg}\n");
}

// Generates code for an expression
fn gen_expr(expr: NodeExpr, output: &mut String, vars: &mut HashMap<String, Variable>){
	match expr.var{
		// If integer litteral, push onto stack
		Expr::NodeExprIntLit(expr_int_lit) =>{
			output.push_str(format!("    mov rax, {}\n", expr_int_lit.value.unwrap()).as_str());
			push("rax", output);
		}
		Expr::NodeExprStrLit(expr_str_lit) =>{
			// Check which label the string corresponds to (probably terrible code)
			vars.iter().for_each(|v| {
				if v.0 == &expr_str_lit.clone().value.unwrap(){
					println!("foo");
				}
			});
			output.insert_str(28, &format!("    {}: db\n", "a"));
		}
		// If identifier, check if it's declared, then generate code for it
		Expr::NodeExprIdent(ident) =>{
			if !vars.contains_key(&ident.clone().value.unwrap()){
				println!("Undeclared identifier: {}", ident.value.unwrap());
				exit(1);
			}
			let var = vars.get(&ident.value.unwrap()).unwrap();
			push(&format!("QWORD [rsp + {}]", (get_global_stack_size().lock().unwrap().val - var.stack_loc - 1) * 8), output)
		}
	}
}

// Generate code for a statement
fn gen_stmt(stmt: NodeStmt, mut output: &mut String, vars: &mut HashMap<String, Variable>){
	match stmt.var{
		// Generate code for exit
		Stmts::NodeStmtExit { expr } =>{
			gen_expr(expr, &mut output, vars);
			output.push_str("    mov rax, 60\n");
			pop("rdi", output);
			output.push_str("    syscall\n");
		}
		// Generate code for let
		Stmts::NodeStmtLet { ident, expr } =>{
			// If variable already exists, throw an error and exit
			if vars.contains_key(&ident.clone().value.unwrap()){
				println!("Identifier already used: {}", ident.value.unwrap());
				exit(1);
			}
			// Add the variable to the map and update the stack size
			vars.insert(ident.value.unwrap(), Variable {stack_loc: get_global_stack_size().lock().unwrap().val});
			// Then generate the code for the expression in the variable
			gen_expr(expr, output, vars);
			
		}
		// Generate code for print
		Stmts::NodeStmtPrint {expr} =>{
			gen_expr(expr, output, vars);
		}
	}
}

// Generates code for the start of the program, and each statement
pub fn gen_prog(root: NodeProg) -> String{
	let mut output = String::from("global _start\nsection .data\n_start:\n");
	let mut variables: HashMap<String, Variable> = HashMap::new();

	root.stmts.iter().for_each(|s|{
		gen_stmt(s.clone(), &mut output, &mut variables);
	});
	
	// If the user didn't specify an exit, this exits with 0
	output.push_str("    mov rax, 60\n    mov rdi, 0\n    syscall\n");
	output.clone()
}