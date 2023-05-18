use super::model;
use super::view;

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

pub fn login_module(users: &[model::BankModel; 3])
{
	view::greeting();

	//	Ask user for username/password input to select a dummy bank account.
	let input_username = view::login_prompt_name();
	let input_password = view::login_prompt_password();

	//	Compare user inputs to existing dummy accounts.
	for u in users
	{
		//	Who doesn't love nested if statements?
		if u.user_name == input_username
		{
			if u.user_password == input_password
			{
				//	If bank account matches user inputs, log-in as that account.
				account_module(u, users);
			} else { view::inappropriate_password(); }
		} else { view::inappropriate_username(); }
	}
}

fn account_module(user: &model::BankModel, stored_users: &[model::BankModel; 3])
{
	/*'core: */loop
	{
		let user_menu_option = view::main_menu().parse::<usize>().unwrap();

		//	"Why use enums for menu selection" you ask?  Well...  It was part of an older challenge a buddy of mine gave me to get back into programing.
		match FromPrimitive::from_usize(user_menu_option)
		{
			Some(MenuOptions::CheckBalance) =>
			view::output_current_user_balance(user.check_balance()),

			Some(MenuOptions::DepositMoney) => {
			let user_money_value = view::input_deposit_prompt();
			//	Problem child #1
			user.deposit_money(user_money_value);
			view::output_current_user_balance(user.check_balance()) },

			Some(MenuOptions::WithdrawMoney) => {
				let user_money_value = view::input_withdrawal_prompt();
				if user.attempt_withdrawal(user_money_value) {
					//	Problem child #2
					user.withdraw_money(user_money_value);
				} else { view::insufficient_funds(); }},

			Some(MenuOptions::TradeMoney) => trade_module(user, stored_users),

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

fn trade_module(user: &model::BankModel, stored_users: &[model::BankModel; 3])
{
	/*'trade: */loop
	{
		let input_trading_to = view::trade_menu();

		for account in stored_users
		{
			if input_trading_to == user.user_name {
				view::inappropriate_trade();
				break;
			} else if input_trading_to == account.user_name {
				//	What line I believe is the root cause of the problem.
				let input_trade_value = view::trade_funds(account.user_name);
				if user.attempt_withdrawal(input_trade_value) {
					//	Problem child #1 and #2 working together to ruin my life.
					user.withdraw_money(input_trade_value);
					account.deposit_money(input_trade_value);
				}
			} else if input_trading_to == "Back" {
				account_module(user, stored_users);
			} else { view::inappropriate_input(); }
		}
	}
}
