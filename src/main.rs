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

fn main() {
    println!("Hello, accounting world!");

    let mut trading_platform = TradingPlatform::new();
    loop {
        let input = read_from_stdin(
            "Choose operation [deposit, withdraw, send, print, quit], confirm with return:",
        );
        match input.as_str() {
            "order" => {
                // let price = read_from_stdin("Price:").parse();
                // let amount = read_from_stdin("Amount:").parse();
                // let side = read_from_stdin("Side:").parse();
                // let signer = read_from_stdin("Signer:").parse();
                //
                // trading_platform.order(Order{
                //     price,
                //     amount,
                //     side,
                //     signer
                // });
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
