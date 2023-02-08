use chrono::prelude::*;
use rust_decimal::Decimal;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::str::FromStr;
use substring::Substring;

use super::{PaymentData, StartPaymentData};

pub fn payment_service(mut lines: Lines<BufReader<File>>) -> (StartPaymentData, Vec<PaymentData>) {
    // Parse first line for metadata to startPaymentBundle
    let first_line = lines
        .next()
        .unwrap()
        .expect("Failed to read first line of buffer.");

    // Parse account number
    let account_nr = first_line.substring(1, 16).trim().to_string(); // Get char 2..=16 and turn to String

    // Parse date
    let date_string = format!("{}000000+0000", first_line.substring(40, 48).trim());

    let dt = DateTime::parse_from_str(date_string.as_str(), "%Y%m%d%H%M%S%z")
        .expect("Failed to parse date")
        .with_timezone(&Utc);

    // Parse currency
    let currency = first_line.substring(48, 51).trim().to_string();

    let start_data = StartPaymentData::new(account_nr, dt, currency);

    let mut payment_data_vec = Vec::with_capacity(30);

    // Iterate lines
    for line in lines.flatten() {
        // Parse amount
        let amount = Decimal::from_str(line.substring(1, 15).replace(',', ".").trim())
            .expect("Failed to parse string to Decimal");

        // Parse reference
        let reference = line.substring(15, 50).trim().to_string();

        payment_data_vec.push(PaymentData::new(amount, reference));
    }
    // Return data
    (start_data, payment_data_vec)
}

pub fn invoice_service(mut lines: Lines<BufReader<File>>) -> (StartPaymentData, Vec<PaymentData>) {
    // Parse first line for metadata to startPaymentBundle
    let first_line = lines
        .next()
        .unwrap()
        .expect("Failed to read first line of buffer.");

    // Parse account number
    let account_nr = first_line.substring(10, 24).trim().to_string(); // Get char 2..=16 and turn to String

    // Parse date - since not in file default is now. Timezone could potentially be hardcoded.
    let dt = Utc::now();

    // Parse currency - since not in file default is "SEK".
    let currency = "SEK".to_string();

    let start_data = StartPaymentData::new(account_nr, dt, currency);

    let mut payment_data_vec = Vec::with_capacity(30);

    for line in lines.flatten() {
        // Check for end
        if line.substring(0, 2) == "99" {
            break;
        }

        // Parse amount
        let amount_raw = line.substring(2, 22).to_string();
        let amount = parse_decimal_n(amount_raw, 2);

        // Parse reference
        let reference = line.substring(40, 65).trim().to_string();

        payment_data_vec.push(PaymentData::new(amount, reference));
    }
    // Return data
    (start_data, payment_data_vec)
}

fn parse_decimal_n(mut amount_raw: String, decimals: usize) -> Decimal {
    amount_raw = format!(
        "{},{}",
        amount_raw.substring(0, amount_raw.len() - decimals),
        amount_raw.substring(amount_raw.len() - decimals, amount_raw.len())
    );
    let mut lead_zeros = 0;
    for c in amount_raw.chars() {
        match c {
            '0' => lead_zeros += 1,
            _ => break,
        }
    }
    amount_raw = amount_raw
        .substring(lead_zeros, amount_raw.len() - 1)
        .to_string();

    Decimal::from_str(amount_raw.replace(',', ".").as_str())
        .expect("Failed to parse string to Decimal")
}
