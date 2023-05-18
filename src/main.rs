#![allow(non_snake_case)]
pub mod controller;
pub mod view;
pub mod model;

fn main()
{
	//	Initialize dummy bank accounts and hop into controller.
	let user_models: [model::BankModel; 3] = [
	model::BankModel::new("brunotamer".to_string(), "settleDOWNok".to_string(), 69_420),
	model::BankModel::new("bawkington".to_string(), "zanyxander".to_string(), 8_008_135),
	model::BankModel::new("weberino".to_string(), "peepeepoopoo".to_string(), 666)];

	controller::login_module(&user_models);
}
