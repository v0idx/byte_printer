// need to convert f64 -> string
// bytes to written!
//need to find out if the byte is negative
use std::cmp;


fn is_negative(num: f64) -> bool {
    if num.is_sign_negative() {
        return true;
    } else {
        return false;
    }
}

pub fn convert(num: f64, bin: bool) -> String {
    let mut negative = "";
    if is_negative(num) {
        negative = "-";
    }
    //define 2 lists of units
    let dec_units = vec!["B","KB","MB","GB","TB","PB","EB"];
    let bin_units = vec!["B","KiB","MiB","GiB","TiB","PiB","EiB"];

    //need to work out which set of units we're using

    if bin {
        //binary case (MiB)
        if num < 1024_f64 {
            //just display as bytes
            return format!("{}{} {}", negative, num, "B");
        } else {
            //now the fun bit
            let delim = 1024_f64;
            //quite interesting method from pretty_bytes
            let exp = cmp::min((num.ln() / delim.ln()).floor() as i32, (bin_units.len() - 1) as i32);
            let out = format!("{:.2}", num/delim.powi(exp)).parse::<f64>().unwrap();
            let unit = bin_units[exp as usize];
            return format!("{}{} {}", negative, out, unit);
        }
    } else {
        //decimal case (MB)
        if num < 1000_f64 {
            return format!("{}{} {}", negative, num, "B");
        } else {
            let delim = 1000_f64;
            let exp = cmp::min((num.ln() / delim.ln()).floor() as i32, (dec_units.len() - 1) as i32);
            let out = format!("{:.2}", num/delim.powi(exp)).parse::<f64>().unwrap();
            let unit = dec_units[exp as usize];
            return format!("{}{} {}", negative, out, unit);
        }

    }
}

fn main() {
    
    

}