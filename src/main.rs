use std::{collections::HashSet, io::Read};
use clap::{Args,Parser,Subcommand};

#[derive(Parser)]
#[command(author,version,about,long_about=None)]
#[command(propagate_version=true)]
struct Cli{
	#[command(subcommand)]
	command:Commands,
}

#[derive(Subcommand)]
enum Commands{
	Whitelist(WhitelistCommand),
}

#[derive(Args)]
struct WhitelistCommand{
	whitelist_file:std::path::PathBuf,
	script_file:std::path::PathBuf,
}

fn main()->Result<(),std::io::Error>{
	match Cli::parse().command{
		Commands::Whitelist(whitelist_command)=>{
			let success=whitelist_script(whitelist_command.whitelist_file,whitelist_command.script_file)?;
			println!("passed whitelist: {}",success);
		},
	}
	Ok(())
}

fn check_function_call(whitelist:&HashSet<String>,function_call:&full_moon::ast::FunctionCall)->bool{
	println!("encountered function call with prefix: {:?}",function_call.prefix());
	//test if the function is whitelisted
	match function_call.prefix(){
		full_moon::ast::Prefix::Name(token)=>{
			match token.token().token_type(){
				full_moon::tokenizer::TokenType::Identifier{identifier}=>{
					if whitelist.contains(identifier.as_str()){
						println!("A-ok function call");
						return true;
					}else{
						println!("illegal function call");
						return false;
					}
				},
				_=>println!("token_type too complicated"),
			}
		},
		_=>println!("function call too complicated"),
	}
	return false;
}

fn check_expression(whitelist:&HashSet<String>,expression:&full_moon::ast::Expression)->bool{
	match expression{
		full_moon::ast::Expression::FunctionCall(function_call)=>check_function_call(whitelist,function_call),
		/*
		full_moon::ast::Expression::BinaryOperator { lhs, binop, rhs } => todo!(),
		full_moon::ast::Expression::Parentheses { contained, expression } => todo!(),
		full_moon::ast::Expression::UnaryOperator { unop, expression } => todo!(),
		full_moon::ast::Expression::Function(_) => todo!(),
		full_moon::ast::Expression::IfExpression(_) => todo!(),
		full_moon::ast::Expression::InterpolatedString(_) => todo!(),
		full_moon::ast::Expression::TableConstructor(_) => todo!(),
		full_moon::ast::Expression::Number(_) => todo!(),
		full_moon::ast::Expression::String(_) => todo!(),
		full_moon::ast::Expression::Symbol(_) => todo!(),
		full_moon::ast::Expression::TypeAssertion { expression, type_assertion } => todo!(),
		full_moon::ast::Expression::Var(_) => todo!(),
		_ => todo!(),
		*/
		_=>return true,
	}
}

fn check_statement(whitelist:&HashSet<String>,statement:&full_moon::ast::Stmt)->bool{
	match statement{
		full_moon::ast::Stmt::FunctionCall(function_call)=>check_function_call(whitelist,function_call),
		//not a function call
		full_moon::ast::Stmt::Assignment(assignment)=>{
			for pair in assignment.expressions().pairs(){
				if !check_expression(whitelist,pair.value()){
					return false;
				}
			}
			return true;
		},
		full_moon::ast::Stmt::Do(_) => todo!(),
		full_moon::ast::Stmt::FunctionDeclaration(_) => todo!(),
		full_moon::ast::Stmt::GenericFor(_) => todo!(),
		full_moon::ast::Stmt::If(_) => todo!(),
		full_moon::ast::Stmt::LocalAssignment(_) => todo!(),
		full_moon::ast::Stmt::LocalFunction(_) => todo!(),
		full_moon::ast::Stmt::NumericFor(_) => todo!(),
		full_moon::ast::Stmt::Repeat(_) => todo!(),
		full_moon::ast::Stmt::While(_) => todo!(),
		full_moon::ast::Stmt::CompoundAssignment(_) => todo!(),
		full_moon::ast::Stmt::ExportedTypeDeclaration(_) => todo!(),
		full_moon::ast::Stmt::TypeDeclaration(_) => todo!(),
		_ => todo!(),
	}
}

fn whitelist_script(
	whitelist_file:std::path::PathBuf,
	script_file:std::path::PathBuf,
)->Result<bool,std::io::Error>{
	//read whitelist from file
	let whitelist:HashSet<String>={
		let mut whitelist_file=std::fs::File::open(whitelist_file)?;
		let mut s=String::new();
		whitelist_file.read_to_string(&mut s)?;
		//this collects each line in the file into a HashSet<String>
		s.lines().map(Into::into).collect()
	};

	//read the file
	let script={
		let mut file=std::fs::File::open(script_file)?;
		let mut s=String::new();
		file.read_to_string(&mut s)?;
		s
	};

	let mut ret=true;

	//parse the string
	match full_moon::parse(script.as_str()){
		Ok(ast)=>{
			for statement in ast.nodes().stmts(){
				if !check_statement(&whitelist,statement){
					ret=false;
				}
			}
		},
		Err(e)=>{
			println!("parsing error: {e}");
			ret=false;
		}
	}
	Ok(ret)
}