pub enum InputVal
{
    HexadecimalNumber(i64),
    OctalNumber(i64),
    DecimalNumber(i64),
    BinaryNumber(i64),
    ASCIIChar(i64),
}

impl InputVal
{
    pub fn decide(string: String) -> Result<InputVal, String>
    {
        let lowerstring = string.to_lowercase();
        let num: i64;
        if lowerstring.starts_with("0x")
        {
            num = match i64::from_str_radix(lowerstring.trim_start_matches("0x"), 16)
            {
                Ok(x) => x,
                Err(x) => return Err(x.to_string()),
            };
            return Ok(InputVal::HexadecimalNumber(num));
        }
        else if lowerstring.starts_with("0o")
        {
            num = match i64::from_str_radix(lowerstring.trim_start_matches("0o"), 8)
            {
                Ok(x) => x,
                Err(x) => return Err(x.to_string()),
            };
            return Ok(InputVal::OctalNumber(num));
        }
        else if lowerstring.starts_with("0b")
        {
            num = match i64::from_str_radix(lowerstring.trim_start_matches("0b"), 2)
            {
                Ok(x) => x,
                Err(x) => return Err(x.to_string()),
            };
            return Ok(InputVal::BinaryNumber(num));
        }
        else if lowerstring.is_ascii() && string.len() == 1
        {
            num = string.chars().nth(0).unwrap() as i64;
            return Ok(InputVal::ASCIIChar(num));
        }
        else
        {
            num = match string.parse()
            {
                Ok(x) => x,
                Err(x) => return Err(x.to_string()),
            };
            return Ok(InputVal::DecimalNumber(num));
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn input_char_decide_hex()
    {
        let num = InputVal::decide(String::from("0xA")).unwrap();
        assert!(match num
        {
            InputVal::HexadecimalNumber(x) =>
            {
                if x == 0xA
                {
                    true
                }
                else
                {
                    false
                }
            }
            InputVal::OctalNumber(_) => false,
            InputVal::DecimalNumber(_) => false,
            InputVal::BinaryNumber(_) => false,
            InputVal::ASCIIChar(_) => false,
        })
    }
    #[test]
    fn input_char_decide_binary()
    {
        let num = InputVal::decide(String::from("0b00000111")).unwrap();
        assert!(match num
        {
            InputVal::BinaryNumber(x) =>
            {
                if x == 0b00000111
                {
                    true
                }
                else
                {
                    false
                }
            }
            _ => false,
        })
    }
    #[test]
    fn input_char_decide_octal()
    {
        let num = InputVal::decide(String::from("0o33")).unwrap();
        assert!(match num
        {
            InputVal::OctalNumber(x) =>
            {
                if x == 0o33
                {
                    true
                }
                else
                {
                    false
                }
            }
            _ => false,
        })
    }
    #[test]
    fn input_char_decide_decimal()
    {
        let num = InputVal::decide(String::from("77")).unwrap();
        assert!(match num
        {
            InputVal::DecimalNumber(x) =>
            {
                if x == 77
                {
                    true
                }
                else
                {
                    false
                }
            }
            _ => false,
        })
    }
    #[test]
    fn input_char_decide_ascii()
    {
        let num = InputVal::decide(String::from("A")).unwrap();
        assert!(match num
        {
            InputVal::ASCIIChar(x) =>
            {
                if x == 65
                {
                    true
                }
                else
                {
                    false
                }
            }
            _ => false,
        })
    }
}
