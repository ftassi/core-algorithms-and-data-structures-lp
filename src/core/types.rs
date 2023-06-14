use std::{cmp::Reverse, error::Error};

/// Simplified side of a position as well as order.
#[derive(Clone, PartialOrd, PartialEq, Eq, Debug, Ord)]
pub enum Side {
    /// Want to buy
    Buy,
    /// Want to sell
    Sell,
}

impl TryFrom<String> for Side {
    type Error = ();
    fn try_from(string: String) -> Result<Self, ()> {
        match string.to_lowercase().as_str() {
            "buy" => Ok(Self::Buy),
            "sell" => Ok(Self::Sell),
            _ => Err(())
        }
    }
}

/// An order for a specified symbol to buy or sell an amount at a given price.
#[derive(Clone, PartialEq, Eq)]
pub struct Order {
    /// Max/min price (depending on the side)
    pub price: u64,
    /// Number of units to trade
    pub amount: u64,
    /// The side of the order book (buy or sell)
    pub side: Side,
    /// The account signer
    pub signer: String,
}

impl Order {
    /// Convert an [`Order`] into a [`PartialOrder`] with the added parameters
    pub fn into_partial_order(self, ordinal: u64, remaining: u64) -> PartialOrder {
        let Order {
            price,
            amount,
            side,
            signer,
        } = self;
        PartialOrder {
            price,
            amount,
            remaining,
            side,
            signer,
            ordinal,
        }
    }

    pub fn required_fund_to_fulfill(&self) -> Option<u64> {
        match self.side {
            Side::Sell => None,
            Side::Buy => self.amount.checked_mul(self.price),
        }
    }
}

/// A position represents an unfilled order that is kept in the system for later filling.
#[derive(Clone, PartialEq, Debug, Eq, Ord)]
pub struct PartialOrder {
    /// Price per unit
    pub price: u64,
    /// Initial number of units in the order
    pub amount: u64,
    /// Remaining number of units after potential matches
    pub remaining: u64,
    /// Buy or sell side of the book
    pub side: Side,
    /// Signer of the order
    pub signer: String,
    /// Sequence number
    pub ordinal: u64,
}

impl PartialOrd for PartialOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // this reverses the comparison to create a min heap
        Reverse(self.ordinal).partial_cmp(&Reverse(other.ordinal))
    }
}

/// A receipt issued to the caller for accepting an [`Order`]
#[derive(Clone, PartialOrd, PartialEq, Eq, Debug)]
pub struct Receipt {
    /// Sequence number
    pub ordinal: u64,

    /// Matches that happened immediately
    pub matches: Vec<PartialOrder>,
}

impl PartialOrder {
    /// Splits one [`PartialOrder`] into two by taking a defined `take` amount
    pub fn take_from(pos: &mut PartialOrder, take: u64, price: u64) -> PartialOrder {
        pos.remaining -= take;
        let mut new = pos.clone();
        new.amount = take;
        new.price = price;
        new
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn side_should_parse_from_string() {
        assert_eq!("buy".to_string().try_into(), Ok(Side::Buy));
        assert_eq!("bUy".to_string().try_into(), Ok(Side::Buy));
        assert_eq!("sell".to_string().try_into(), Ok(Side::Sell));
        assert_eq!("seLl".to_string().try_into(), Ok(Side::Sell));

        assert_eq!(Side::try_from("wrong".to_string()), Err(()));
}

    #[test]
    fn sell_order_do_not_require_fund_to_be_fullfilled() {
        let order = Order {
            price: 10,
            amount: 5,
            side: Side::Sell,
            signer: "BOB".to_string(),
        };

        assert_eq!(order.required_fund_to_fulfill(), None);
    }
    #[test]
    fn buy_order_do_require_fund_to_be_fullfilled() {
        let order = Order {
            price: 10,
            amount: 5,
            side: Side::Buy,
            signer: "BOB".to_string(),
        };

        assert_eq!(order.required_fund_to_fulfill(), Some(50));
    }
}
