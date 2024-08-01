pub trait BankAccount{
    fn new(balance: f64, currency: Currency) -> Self;
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64) -> bool;
    fn check_balance(&self) -> f64;
}

pub enum Currency{
    USD,
    EUR,
    GBP,
    BRL,
    BTC,
}

pub struct Account{
    balance: f64,
    currency: Currency,
}

impl BankAccount for Account{
    fn new(balance: f64, currency: Currency) -> Self{
        Account{balance, currency}
    }
    fn deposit(&mut self, amount: f64){
        self.balance += amount;
    }
    fn withdraw(&mut self, amount: f64) -> bool{
        
        match self.balance > amount{
            true =>{
                self.balance -= amount;
                true
            }
            false => false,
        }
    }
    fn check_balance(&self) -> f64{

        self.balance
    }
}
