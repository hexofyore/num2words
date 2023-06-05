use crate::{num2words::Num2Err, Currency, Language};
use num_bigfloat::BigFloat;

pub struct Nepali;

const UNITS: [&'static str; 100] = [
    "शून्य",
    "एक",
    "दुई",
    "तीन",
    "चार",
    "पाँच",
    "छ",
    "सात",
    "आठ",
    "नौ",
    "दश",
    "एघार",
    "बाह्र",
    "तेह्र",
    "चौध",
    "पन्ध्र",
    "सोह्र",
    "सत्र",
    "अठार",
    "उन्नाइस",
    "बीस",
    "एक्काइस",
    "बाइस",
    "त्येइस",
    "चौबीस",
    "पच्चीस",
    "छब्बीस",
    "सत्ताइस",
    "अठ्ठाइस",
    "उनन्तीस",
    "तीस",
    "एक्तीस",
    "बत्तीस",
    "तेत्तिस",
    "चौंतीस",
    "पैंतीस",
    "छत्तीस",
    "सर्तीस",
    "अर्तीस",
    "उनन्चालीस",
    "चालीस",
    "एकचालीस",
    "बयालीस",
    "त्रिचालीस",
    "चवालीस",
    "पैंतालीस",
    "छयालीस",
    "सर्चालीस",
    "अर्चालीस",
    "उनन्चास",
    "पचास",
    "एकाउन्न",
    "बाउन्न",
    "त्रिपन्न",
    "चौवन्न",
    "पचपन्न",
    "छ्प्पन्न",
    "सन्ताउन्न",
    "अन्ठाउन्न",
    "उनान्साठ्ठी",
    "साठ्ठी",
    "एकसट्ठी",
    "बैसट्ठी",
    "त्रिसट्ठी",
    "चौंसठ्ठी",
    "पैंसठ्ठी",
    "छैंसठ्ठी",
    "सर्सठ्ठी",
    "अर्सठ्ठी",
    "उनन्सत्तरी",
    "सत्तरी",
    "एकत्तर",
    "बहत्तर",
    "त्रीहत्तर",
    "चौरत्तर",
    "पचहत्तर",
    "छयत्तर",
    "सतहत्तर",
    "अठहत्तर",
    "उनास्सी",
    "अस्सी",
    "एकास्सी",
    "बयास्सी",
    "त्रियास्सी",
    "चैरस्सी",
    "पचास्सी",
    "छायास्सी",
    "सत्तास्सी",
    "अठास्सी",
    "उनन्नाब्बे",
    "नब्बे",
    "एकान्नब्बे",
    "बयान्नब्बे",
    "त्रियान्नब्बे",
    "चौरान्नब्बे",
    "पन्चान्नब्बे",
    "छायान्नब्बे",
    "सन्तान्नब्बे",
    "अन्ठान्नब्बे",
    "उनन्सय",
];
const _MAX_PAISA: i64 = 99;
const NEPALI_ORDINALS: [&'static str; 103] = [
    "पहिलो",
    "दोस्रो",
    "तेस्रो",
    "चौथो",
    "पाँचौँ",
    "छैटौँ",
    "सातौँ",
    "आठौँ",
    "नवौँ",
    "दसौँ",
    "एघारौँ",
    "बाह्रौँ",
    "तेह्रौँ",
    "चौधौँ",
    "पन्ध्रौँ",
    "सोह्रौँ",
    "सत्रौँ",
    "अठारौँ",
    "उन्नाइसौँ",
    "बिसौँ",
    "एक्काइसौँ",
    "बाइसौँ",
    "तेइसौँ",
    "चौबिसौँ",
    "पच्चिसौँ",
    "छब्बिसौँ",
    "सत्ताइसौँ",
    "अट्ठाइसौँ",
    "उनन्तिसौँ",
    "तिसौँ",
    "एकतिसौँ",
    "बत्तिसौँ",
    "तेत्तिसौँ",
    "चौतिसौँ",
    "पैँतिसौँ",
    "छत्तिसौँ",
    "सैँतिसौँ",
    "अठतिसौँ",
    "उनन्चालिसौँ",
    "चालिसौँ",
    "एकचालिसौँ",
    "बयालिसौँ",
    "त्रिचालिसौँ",
    "चवालिसौँ",
    "पैँतालिसौँ",
    "छयालिसौँ",
    "सतचालिसौँ",
    "अठचालिसौँ",
    "उनन्चासौँ",
    "पचासौँ",
    "एकाउन्नौँ",
    "बाउन्नौँ",
    "त्रिपन्नौँ",
    "चवन्नौँ",
    "पचपन्नौँ",
    "छपन्नौँ",
    "सन्ताउन्नौँ",
    "अन्ठाउन्नौँ",
    "उनन्साठिऔँ",
    "साठिऔँ",
    "एकसट्ठिऔँ",
    "बयसट्ठिऔँ",
    "त्रिसट्ठिऔँ",
    "चौसट्ठिऔँ",
    "पैँसट्ठिऔँ",
    "छयसट्ठिऔँ",
    "सतसट्ठिऔँ",
    "अठसट्ठिऔँ",
    "उनन्सत्तरिऔँ",
    "सत्तरिऔँ",
    "एकहत्तरौँ",
    "बहत्तरौँ",
    "त्रिहत्तरौँ",
    "चौहत्तरौँ",
    "पचहत्तरौँ",
    "छयहत्तरौँ",
    "सतहत्तरौँ",
    "अठहत्तरौँ",
    "उनासिऔँ",
    "असिऔँ",
    "एकासिऔँ",
    "बयासिऔँ",
    "त्रियासिऔँ",
    "चौरासिऔँ",
    "पचासिऔँ",
    "छयासिऔँ",
    "सतासिऔँ",
    "अठासिऔँ",
    "उनान्नब्बेऔँ",
    "नब्बेऔँ",
    "एकान्नबेऔँ",
    "बयान्नब्बेऔँ",
    "त्रियानब्बेऔँ",
    "चौरान्नब्बेऔँ",
    "पन्चान्नब्बेऔँ",
    "छयान्नब्बेऔँ",
    "सन्तान्नब्बेऔँ",
    "अन्ठान्नब्बेऔँ",
    "उनान्सयौँ",
    "सयौँ",
    "हजारौँ",
    "लाखौँ",
    "करोडौँ",
];

const NP_SCALE: [&'static str; 6] = ["सय", "हजार", "लाख", "करोड", "अर्ब", "खर्ब"];

impl Nepali {
    pub fn new() -> Self {
        Self {}
    }

    fn split_np(&self, num: BigFloat) -> Vec<u64> {
        let mut splits = Vec::new();
        let mut num = num.to_u128().unwrap();

        splits.push((num % 100) as u64);
        num /= 100;

        splits.push((num % 10) as u64);
        num /= 10;
        while num != 0 {
            splits.push((num % 100) as u64);
            num /= 100;
        }
        splits
    }
    fn int_to_cardinal(&self, mut num: BigFloat) -> Result<String, Num2Err> {
        if num.is_zero() {
            return Ok(String::from(UNITS[0]));
        }

        let mut words = vec![];
        if num.is_negative() {
            words.push(String::from("-"));
            num = -num;
        }

        for (i, &split) in self.split_np(num).iter().enumerate().rev() {
            if split == 0 {
                continue;
            }
            words.push(String::from(UNITS[split as usize]));
            if i != 0 {
                words.push(NP_SCALE[i - 1].to_string());
            }
            if i > NP_SCALE.len() {
                return Err(Num2Err::CannotConvert);
            }
        }
        Ok(words.join(" "))
    }

    fn float_to_cardinal(&self, num: BigFloat) -> Result<String, Num2Err> {
        let integer_part = num.int();
        let mut words: Vec<String> = vec![];

        if !integer_part.is_zero() {
            let integral_word = self.int_to_cardinal(integer_part)?;
            words.push(integral_word);
        }

        let fractional_part = num.frac();
        if !fractional_part.is_zero() {
            words.push(String::from("दशमलव"));
        }
        let mut fractional_words = vec![];
        let mut fractional_part = if let Some(val) = fractional_part.to_u128() {
            val
        } else {
            0
        };
        while fractional_part != 0 {
            let digit = fractional_part % 10;
            fractional_words.push(UNITS[digit as usize].to_string());
            fractional_part /= 10;
        }
        words.extend(fractional_words.iter().rev().cloned());
        Ok(words.join(" "))
    }
}

impl Language for Nepali {
    fn to_cardinal(&self, num: BigFloat) -> Result<String, Num2Err> {
        if num.is_inf_pos() {
            Ok(String::from("अनन्त"))
        } else if num.is_inf_neg() {
            Ok(String::from("-अनन्त"))
        } else if num.frac().is_zero() {
            self.int_to_cardinal(num)
        } else {
            self.float_to_cardinal(num)
        }
    }

    fn to_ordinal(&self, num: BigFloat) -> Result<String, Num2Err> {
        if !num.frac().is_zero() {
            return Err(Num2Err::FloatingOrdinal);
        }
        let splits = self.split_np(num);
        let mut i = 0;
        println!("{:?}", splits);
        for split in splits.iter() {
            if *split != 0 {
                break;
            }
            i += 1;
        }
        if i == 0 {
            let last_num = splits[0];
            let cardinal_word = self.to_cardinal(num)?;
            let mut words = cardinal_word
                .split_whitespace()
                .collect::<Vec<_>>()
                .to_owned();
            words.pop();
            words.push(NEPALI_ORDINALS[last_num as usize - 1]);
            Ok(words.join(" "))
        } else {
            i += 100 - 2;
            let cardinal_word = self.to_cardinal(num)?;
            let mut words = cardinal_word
                .split_whitespace()
                .collect::<Vec<_>>()
                .to_owned();
            words.pop();
            words.push(NEPALI_ORDINALS[i as usize]);
            Ok(words.join(" "))
        }
    }

    fn to_ordinal_num(&self, _num: BigFloat) -> Result<String, Num2Err> {
        Err(Num2Err::CannotConvert)
    }

    fn to_year(&self, num: BigFloat) -> Result<String, Num2Err> {
        if !num.frac().is_zero() {
            return Err(Num2Err::FloatingYear);
        }
        let cardinal_word = self.int_to_cardinal(num)?;
        let mut words = vec![cardinal_word];
        words.push(String::from("साल"));
        Ok(words.join(" "))
    }

    fn to_currency(&self, num: BigFloat, _currency: Currency) -> Result<String, Num2Err> {
        if num.is_inf_pos() {
            Ok(String::from("अनन्त"))
        } else if num.is_inf_neg() {
            Ok(String::from("-अनन्त"))
        } else if num.frac().is_zero() {
            let words = self.int_to_cardinal(num)?;
            Ok(format!("{} {}", words, String::from("रुपैयाँ")))
        } else {
            let integral_part = num.int();
            let paisa = (num * BigFloat::from(100)).int() % BigFloat::from(100);
            let paisa_words = self.int_to_cardinal(paisa)?;
            let paisa_suffix = String::from("पैसा");
            let integral_word = self.to_currency(integral_part, Currency::NPR)?;
            if paisa.is_zero() {
                Ok(integral_word)
            } else if integral_part.is_zero() {
                Ok(format!("{} {}", paisa_words, paisa_suffix))
            } else {
                Ok(format!(
                    "{} {} {}",
                    integral_word, paisa_words, paisa_suffix
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Lang, Num2Words};

    use super::*;
    #[test]
    fn nepali_test() {
        let a = Nepali::new();
        let res = a.to_ordinal(123456000.into()).unwrap();
        println!("split: {}", res);
        println!(
            "Final: {}",
            Num2Words::new(4232).lang(Lang::Nepali).to_words().unwrap()
        )
        // assert_eq!(1, 1);
    }
}
