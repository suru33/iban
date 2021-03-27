use rand::Rng;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IBANMetaData {
    pub country: String,
    pub code: String,
    pub sepa: bool,
    pub length: u8,
    pub account_check: bool,
    pub branch: bool,
    pub format: String,
}

fn mod97(head: &str, tail: &str) -> u32 {
    if tail.len() > 7 {
        let h = &(format!("{:02}", head.parse::<u32>().unwrap() % 97) + &tail[0..7]);
        let t = &tail[7..];
        mod97(h, t)
    } else {
        let h_val = head.parse::<u32>().unwrap() % 97;
        let t_val = (h_val.to_string() + tail).parse::<u32>().unwrap();
        98 - (t_val % 97)
    }
}

impl IBANMetaData {
    pub fn get(&self) -> String {
        let mut rng = rand::thread_rng();

        let format = self.format[4..].to_string() + &self.format[0..4];
        let mut s = String::new();
        let mut digits: Vec<i32> = Vec::new();
        for ch in format.chars() {
            if ch == '#' {
                let d = &rng.gen_range(0..10);
                digits.push(*d);
                s.push_str(&d.to_string());
            } else if ch.is_alphabetic() {
                s.push_str(&(ch as u8 - 55).to_string());
            } else {
                s.push(ch);
            }
        }

        let checksum = format!("{:02}", mod97(&s[0..9], &s[9..]));

        let mut iban = self.format.replacen("00", &checksum, 1);
        let mut i = digits.iter();
        while iban.contains('#') {
            iban = iban.replacen('#', &i.next().unwrap().to_string(), 1);
        }
        iban
    }

    pub fn get_pretty(&self) -> String {
        let mut iban = String::new();
        for (i, ch) in self.get().chars().enumerate() {
            iban.push(ch);
            if (i + 1) % 4 == 0 {
                iban.push(' ');
            }
        }
        iban.trim().to_string()
    }
}