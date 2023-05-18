pub fn greeting()
{
	println!("Welcome to CorpoBank USA...");
}

pub fn login_prompt_name() -> String
{
	print!("Username: ");
	return sanitize_input();
}

pub fn login_prompt_password() -> String
{
	print!("Password: ");
	return sanitize_input();
}

pub fn main_menu() -> String
{
	println!("\t0) Check Balance
	\t1) Deposit Money
	\t2) Withdraw Money
	\t3) Send Money\n
	\t7) Log Off
	\t8) Exit\n
	Make your choice before I make it for you...");

	return sanitize_input();
}

pub fn output_current_user_balance(current_user_balance: usize)
{
	println!("You currently have ${current_user_balance}.\n")
}

pub fn input_deposit_prompt() -> usize
{
	println!("How much money would you like to deposit?");
	let user_deposit = sanitize_input();

	return user_deposit.parse::<usize>().unwrap();
}

pub fn deposit_confirmation(user_deposit: usize)
{
	println!("Your account has been credited ${user_deposit}.");
}

pub fn input_withdrawal_prompt() -> usize
{
	println!("How much money would you like to withdraw?");
	let user_withdrawal = sanitize_input();

	return user_withdrawal.parse::<usize>().unwrap();
}

pub fn withdrawal_confirmation(user_withdrawal: usize)
{
	println!("${user_withdrawal} has been withdrawn from your account.");
}

pub fn insufficient_funds()
{
	println!("Woah there lil' buddy!  This here looks like more money than you have, we don't loan to losers 'round these parts...\n")
}

pub fn log_off_prompt()
{
	println!("Until next time!");
}

pub fn exit_prompt()
{
	println!("Thank you for your mo- err... patronage!");
}

pub fn trade_menu() -> String
{
	println!("
	Whomst'd've you like to send your money to?\n
	(\"Back\" to return to main menu)
	");

	return sanitize_input();
}

pub fn trade_funds(tradee_username: String) -> usize
{
	println!("How much money would you like to send to {tradee_username}?");
	let trading_value = sanitize_input();

	return trading_value.parse::<usize>().unwrap();
}

//	Ensure potentially unwanted trailing characters are stripped (eg: newline from pressing Enter)
fn sanitize_input() -> String
{
	use std::io::{stdin, stdout, Write};
	let mut user_input = String::new();
	let _ = stdout().flush();
	stdin().read_line(&mut user_input).expect("wrong input");
	if let Some('\n') = user_input.chars().next_back() { user_input.pop(); }
	if let Some('\r') = user_input.chars().next_back() { user_input.pop(); }

	return user_input;
}

//	This is the best way to handle errors and unwanted behavior.  Fight me. (Please don't actually)
pub fn inappropriate_username() { println!("We have no account under that username."); }
pub fn inappropriate_password() { println!("Wrong password, buckaroo."); }
pub fn inappropriate_input() { println!("Alrighty then, keep your secrets...\n"); }
pub fn inappropriate_trade() { println!("Are you trying to trade yourself, again?  You silly goose...\n"); }
