#![allow(dead_code, unused_imports, unused)]
use std::io;
mod accounting;
mod core;
mod errors;
mod trading_platform;
mod tx;
use crate::{accounting::Accounts, trading_platform::TradingPlatform, core::Order};

fn read_from_stdin(label: &str) -> String {
    let mut buffer = String::new();
    println!("{}", label);
    io::stdin()
        .read_line(&mut buffer)
        .expect("Couldn't read from stdin");
    buffer.trim().to_owned()
}

// potremmo usare anyhow per aver un tipo di errore migliore
fn read_order_from_stdin() -> Result<Order, String> {
    let price = read_from_stdin("Price:").parse().map_err(|_| "Not a valid price")?;
    let amount = read_from_stdin("Amount:").parse().map_err(|_| "Not a valid amount")?;
    let side = read_from_stdin("Side:").try_into().map_err(|e| "Side must be SELL or BUY")?;
    let signer = read_from_stdin("Signer:");

    Ok(Order{
        price,
        amount,
        side,
        signer
    })
}

fn main() {
    println!("Hello, accounting world!");

    let mut trading_platform = TradingPlatform::new();
    loop {
        let input = read_from_stdin(
            "Choose operation [order, orderbook, txtlog, deposit, withdraw, send, print, quit], confirm with return:",
        );
        match input.as_str() {
            "order" => {
                let _ = match read_order_from_stdin() {
                   Ok(order) => {trading_platform.order(order); println!("Ok")}
                   Err(e) => eprintln!("{:?}", e)
                };
            }
            "orderbook" => {}
            "txlog" => {}
            "deposit" => {
                let account = read_from_stdin("Account:");

                let raw_amount = read_from_stdin("Amount:").parse();
                if let Ok(amount) = raw_amount {
                    let _ = trading_platform.deposit(&account, amount);
                    println!("Deposited {} into account '{}'", amount, account)
                } else {
                    eprintln!("Not a number: '{:?}'", raw_amount);
                }
            }
            "withdraw" => {
                let account = read_from_stdin("Account:");
                let raw_amount = read_from_stdin("Amount:").parse();
                if let Ok(amount) = raw_amount {
                    let _ = trading_platform.withdraw(&account, amount);
                } else {
                    eprintln!("Not a number: '{:?}'", raw_amount);
                }
            }
            "send" => {
                let sender = read_from_stdin("Sender Account:");
                let recipient = read_from_stdin("Recipient Account:");
                let raw_amount = read_from_stdin("Amount:").parse();
                if let Ok(amount) = raw_amount {
                    let _ = trading_platform.send(&sender, &recipient, amount);
                } else {
                    eprintln!("Not a number: '{:?}'", raw_amount);
                }
            }
            "print" => {
                println!("The ledger: {:?}", trading_platform);
            }
            "quit" => {
                println!("Quitting...");
                break;
            }
            _ => {
                eprintln!("Invalid option: '{}'", input);
            }
        }
    }
}
