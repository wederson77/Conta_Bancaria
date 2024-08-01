mod account;
use account::{BankAccount, Currency, Account};

fn main() {
    
    let mut account = Account::new(1000.0, Currency::USD);

    account.deposit(500.0);
    println!("Novo saldo {:.2}", account.check_balance());

    let withdraw_succes = account.withdraw(1200.0);
    if withdraw_succes{
        println!("Saque realizado com sucesso {:.2}", account.check_balance());
    }else{
        println!("Saque não efetuado {:.2}", account.check_balance());
    }

}
