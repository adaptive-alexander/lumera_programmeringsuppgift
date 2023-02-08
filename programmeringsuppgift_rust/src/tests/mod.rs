#[cfg(test)]
mod test_parsers {
    use crate::payment_file_handler::{
        parsers::{invoice_service, payment_service},
        read_lines,
    };
    use std::path::PathBuf;
    use chrono::{DateTime, NaiveDate, Utc};
    use rust_decimal_macros::dec;

    #[test]
    fn payment_service_t() {
        // Path buffer to pass
        let path = PathBuf::from("./test_data/Exempelfil_betalningsservice.txt");

        // Create line iterator
        let lines = read_lines(path).expect("Failed to parse lines from buffer.");

        let (start_payment_data, payment_data) = payment_service(lines);

        assert_eq!(start_payment_data.account_nr, "5555 5555555555".to_string());
        assert_eq!(start_payment_data.timestamp, DateTime::<Utc>::from_utc(NaiveDate::from_ymd_opt(2011,3,15).unwrap().and_hms_opt(0,0,0).unwrap(),Utc));
        assert_eq!(start_payment_data.currency, "SEK".to_string());
        assert_eq!(payment_data[0].amount, dec!(3000));
        assert_eq!(payment_data[0].reference, "1234567890".to_string());
    }

    #[test]
    fn invoice_service_t() {
        // Path buffer to pass
        let path = PathBuf::from("./test_data/Exempelfil_inbetalningstjansten.txt");

        // Create line iterator
        let lines = read_lines(path).expect("Failed to parse lines from buffer.");

        let (start_payment_data, payment_data) = invoice_service(lines);

        assert_eq!(start_payment_data.account_nr, "12341234567897".to_string());
        assert_eq!(start_payment_data.currency, "SEK".to_string());
        assert_eq!(payment_data[0].amount, dec!(4000));
        assert_eq!(payment_data[0].reference, "9876543210".to_string());
    }
}
