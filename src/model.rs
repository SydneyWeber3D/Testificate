//	Really basic bank account class for logging in and account balance.
pub struct BankModel
{
	pub user_name: String,
	pub user_password: String,
	user_balance: usize
}

impl BankModel
{
	//	Initializer.
	pub const fn new(user_name: String, user_password: String, user_balance: usize) -> Self
	{
		Self { user_name, user_password, user_balance }
	}

	pub fn check_balance(&self) -> usize
	{
		return self.user_balance;
	}

	//	Problem child #1
	pub fn deposit_money(&mut self, deposit_amount: usize)
	{
		self.user_balance += deposit_amount;
	}

	pub fn attempt_withdrawal(&self, withdraw_amount: usize) -> bool
	{
		return self.user_balance > withdraw_amount;
	}

	//	Problem child #2
	pub fn withdraw_money(&mut self, withdraw_amount: usize)
	{
		self.user_balance -= withdraw_amount;
	}
}
