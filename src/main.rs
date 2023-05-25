#![allow(non_snake_case)]
pub mod view;
pub mod model;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use std::process::exit;

#[derive(FromPrimitive)]
enum MenuOptions
{
	CheckBalance,
	DepositMoney,
	WithdrawMoney,
	TradeMoney,
	LogOff,
	ExitBank
}

fn main()
{
	//	Initialize dummy bank accounts and hop into controller.
	let mut user_models: [model::BankModel; 3] = [
	model::BankModel::new(String::from("brunotamer"), String::from("settleDOWNok"), 69_420),
	model::BankModel::new(String::from("bawkington"), String::from("zanyxander"), 8_008_135),
	model::BankModel::new(String::from("weberino"), String::from("peepeepoopoo"), 666)];

	login_module(&mut user_models);
}

pub fn login_module(stored_accounts: &mut [model::BankModel; 3])
{
	view::greeting();

	//	Ask user for username/password input to select a dummy bank account.
	let input_username = view::login_prompt_name();
	let input_password = view::login_prompt_password();

	//	Compare user inputs to existing dummy accounts.
	for (i, u) in (&stored_accounts).iter().enumerate()
	{
		//	Who doesn't love nested if statements?
		if u.user_name == input_username
		{
			if u.user_password == input_password
			{
				account_module(i, stored_accounts);
			} else { view::inappropriate_password(); }
		} else { view::inappropriate_username(); }
	}


	//account_module(target, users);
}

fn account_module(userID: usize, accounts: &mut [model::BankModel; 3])
{
	/*'core: */loop
	{
		let user_menu_option = view::main_menu().parse::<usize>().unwrap();

		//	"Why use enums for menu selection" you ask?  Well...  It was part of an older challenge a buddy of mine gave me to get back into programing.
		match FromPrimitive::from_usize(user_menu_option)
		{
			Some(MenuOptions::CheckBalance) =>
			view::output_current_user_balance(accounts[userID].check_balance()),

			Some(MenuOptions::DepositMoney) => {
			let user_money_value = view::input_deposit_prompt();
			//	Problem child #1
			accounts[userID].deposit_money(user_money_value);
			view::output_current_user_balance(accounts[userID].check_balance()) },

			Some(MenuOptions::WithdrawMoney) => {
				let user_money_value = view::input_withdrawal_prompt();
				if accounts[userID].attempt_withdrawal(user_money_value) {
					//	Problem child #2
					accounts[userID].withdraw_money(user_money_value);
				} else { view::insufficient_funds(); }},

			Some(MenuOptions::TradeMoney) => trade_module(userID, accounts),

			Some(MenuOptions::LogOff) => {
				view::log_off_prompt();
				break},
			Some(MenuOptions::ExitBank) => {
				view::exit_prompt();
				exit(69)},
			_ => println!("Dook dook")
		}
	}
}

fn trade_module(userID: usize, accounts: &mut [model::BankModel; 3])
{
	/*'trade: */loop
	{
		let input_trading_to = view::trade_menu();

		for (i, u) in accounts.iter().enumerate()
		{
			if input_trading_to == accounts[userID].user_name {
				view::inappropriate_trade();
				break;
			} else if input_trading_to == u.user_name {
				//	What line I believe is the root cause of the problem.
				let input_trade_value = view::trade_funds(accounts[i].user_name);
				if accounts[userID].attempt_withdrawal(input_trade_value) {
					//	Problem child #1 and #2 working together to ruin my life.
					accounts[userID].withdraw_money(input_trade_value);
					accounts[i].deposit_money(input_trade_value);
				}
			} else if input_trading_to == "Back" {
				break;
			} else { view::inappropriate_input(); }
		}
	}
}
