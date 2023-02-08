pub mod parsers;

use core::fmt::Debug;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::{Path, PathBuf};

pub struct StartPaymentData {
    pub account_nr: String,
    pub timestamp: DateTime<Utc>,
    pub currency: String,
}

pub struct PaymentData {
    pub amount: Decimal,
    pub reference: String,
}

impl StartPaymentData {
    fn new(account_nr: String, timestamp: DateTime<Utc>, currency: String) -> Self {
        StartPaymentData {
            account_nr,
            timestamp,
            currency,
        }
    }
}

impl PaymentData {
    fn new(amount: Decimal, reference: String) -> Self {
        PaymentData { amount, reference }
    }
}

pub fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path> + Debug,
{
    let file = match File::open(&filename) {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e);
            panic!("Failed to open {:?}", &filename);
        }
    };

    Ok(BufReader::new(file).lines())
}

pub fn payment_file_handler(path: PathBuf) {
    // Create a BufReader for the file in order to iterate lines
    let lines = read_lines(&path).expect("Failed to parse lines from buffer.");

    // Check file ending to choose parser
    // Pattern matching to split outputs for data
    // If time: add this parsing to a function and use a match statement here for cleaner code
    let (start_payment_data, payment_data) = if path
        .to_str()
        .unwrap()
        .to_lowercase()
        .contains("_betalningsservice.txt")
    {
        parsers::payment_service(lines)
    } else if path
        .to_str()
        .unwrap()
        .to_lowercase()
        .contains("_inbetalningstjansten.txt")
    {
        parsers::invoice_service(lines)
    } else {
        panic!("Unknown file ending, exiting.")
    };

    // Consumes data structures StartPaymentData and Vec<PaymentData> to call fake java interface
    // Call "fake" java interface in output
    println!(
        "payments.startPaymentBundle({}, {}, {})",
        start_payment_data.account_nr, start_payment_data.timestamp, start_payment_data.currency
    );

    for data in payment_data {
        // Call "fake" java interface in output
        println!("payments.payment({}, {})", data.amount, data.reference);
    }
    // Call "fake" java interface in output
    println!("payments.endPaymentBundle()")
}
