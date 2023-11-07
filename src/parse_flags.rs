#[derive(Debug, PartialEq)]
pub enum Flag {
    CaseSensitive,
    CaseInsensitive,
}

pub fn parse_flags(args: &[String]) -> Result<Flag, String> {
    let separated_flags: Vec<&String> = args
        .iter()
        .filter(|&f| f.starts_with("-"))
        .collect();

    if separated_flags.len() == 0 {
        return Ok(Flag::CaseSensitive);
    } else if separated_flags.len() > 1 {
        return Err(String::from("To many arguments"));
    }

    match &*separated_flags[0].to_owned() {
        "-i" => Ok(Flag::CaseInsensitive),
        "-s" => Ok(Flag::CaseSensitive),
        _ => Err(format!("Flag {} is unknown", separated_flags[0])),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_flags() {
        assert_eq!(Ok(Flag::CaseSensitive), parse_flags(&[]));
    }

    #[test]
    fn to_many_arguments() {
        assert_eq!(
            Err(String::from("To many arguments")),
            parse_flags(&[
                "-i".to_string(),
                "-s".to_string(),
                "query".to_string(),
                "file/test.txt".to_string()
            ])
        );
    }

    #[test]
    fn proper_flags() {
        assert_eq!(
            Ok(Flag::CaseSensitive),
            parse_flags(&[
                "-s".to_string(),
                "query".to_string(),
                "file/test.txt".to_string()
            ])
        );

        assert_eq!(
            Ok(Flag::CaseInsensitive),
            parse_flags(&[
                "-i".to_string(),
                "query".to_string(),
                "file/test.txt".to_string()
            ])
        )
    }
}
