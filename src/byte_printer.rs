// Byte Printer - 2026
// Copyright (c) 2022, Eloise Nash
// Rust utility to convert a number of bytes (f64) to a formatted String e.g. "300 KiB"

use std::cmp;

// Function to return a boolean showing whether the input f64 is negatively signed
// in: f64
//out: bool
fn is_negative(num: f64) -> bool {
    num.is_sign_negative()
}

// Function to convert an input number of bytes to a human readable string
// in: f64, bool
// out: String
pub fn convert(num: f64, bin: bool) -> String {
    let mut negative = "";
    if is_negative(num) {
        negative = "-";
    }
    // Define 2 lists of units
    let dec_units = ["B", "KB", "MB", "GB", "TB", "PB", "EB"];
    let bin_units = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];

    // Need to work out which set of units we're using, based on whether the `bin` flag is true or false
    // true is for bin_units
    // false is for dec_units

    if bin {
        // Variable naming:
        // delim -> delimiter
        // exp -> exponent
        // out -> output string

        // Binary case (MiB)
        if num < 1024_f64 {
            // Just display as bytes
            format!("{}{} {}", negative, num, "B")
        } else {
            //now the fun bit
            let delim = 1024_f64;
            // Quite interesting method from pretty_bytes: https://github.com/banyan/rust-pretty-bytes/tree/master
            let exp = cmp::min(
                (num.ln() / delim.ln()).floor() as i32,
                (bin_units.len() - 1) as i32,
            );
            let out = format!("{:.2}", num / delim.powi(exp))
                .parse::<f64>()
                .unwrap();
            let unit = bin_units[exp as usize];
            format!("{}{} {}", negative, out, unit)
        }
    } else {
        // Decimal case (MB)
        if num < 1000_f64 {
            format!("{}{} {}", negative, num, "B")
        } else {
            let delim = 1000_f64;
            // Calculate the exponent to raise the delimiter to the power of.
            // The lower value of either ln(num) / ln(delim) or 6.
            let exp = cmp::min(
                (num.ln() / delim.ln()).floor() as i32,
                (dec_units.len() - 1) as i32,
            );
            // Using the above calculate values, format the amount of bytes to the smallest possible representation
            // with the listed units.
            let out = format!("{:.2}", num / delim.powi(exp))
                .parse::<f64>()
                .unwrap();
            let unit = dec_units[exp as usize];
            format!("{}{} {}", negative, out, unit)
        }
    }
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_is_negative() {
        let pos_num: f64 = 32.0;
        let neg_num: f64 = -32.0;

        assert!(!is_negative(pos_num));

        assert!(is_negative(neg_num));
    }

    #[test]
    fn test_convert() {
        let mut to_convert: f64 = 4096.0;
        let mut bin_flag: bool = true;

        assert_eq!(convert(to_convert, bin_flag), "4 KiB");

        to_convert = 3000.0;
        bin_flag = false;

        assert_eq!(convert(to_convert, bin_flag), "3 KB");

        to_convert = 900.0;

        assert_eq!(convert(to_convert, bin_flag), "900 B");
    }
}
