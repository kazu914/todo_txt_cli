use chrono::Local;
use regex::Regex;

pub fn get_today() -> String {
    Local::today().format("%Y-%m-%d").to_string()
}

pub fn is_valid_date(target: &str) -> bool {
    let re = Regex::new(r"^\d{4}-(\d{1}|\d{2})-(\d{1}|\d{2})$").unwrap();
    re.is_match(target)
}

pub fn is_project(target: &str) -> bool {
    let re = Regex::new(r"^\+").unwrap();
    re.is_match(target)
}

pub fn is_context(target: &str) -> bool {
    let re = Regex::new(r"^@").unwrap();
    re.is_match(target)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod is_valid_date {
        #[test]
        fn match_2000_1_1() {
            let target = "2000-1-1";
            assert!(super::is_valid_date(target));
        }

        #[test]
        fn match_2000_01_01() {
            let target = "2000-01-01";
            assert!(super::is_valid_date(target));
        }

        #[test]
        fn match_2000_01_1() {
            let target = "2000-01-1";
            assert!(super::is_valid_date(target));
        }

        #[test]
        fn unmatch_2000_01_() {
            let target = "2000-01-";
            assert!(!super::is_valid_date(target));
        }

        #[test]
        fn unmatch_2000_01_001() {
            let target = "2000-01-001";
            assert!(!super::is_valid_date(target));
        }
    }

    mod is_project {
        #[test]
        fn project() {
            let target = "+project";
            assert!(super::is_project(target));
        }

        #[test]
        fn not_project() {
            let target = "project";
            assert!(!super::is_project(target));
        }
    }

    mod is_context {
        #[test]
        fn project() {
            let target = "@context";
            assert!(super::is_context(target));
        }

        #[test]
        fn not_project() {
            let target = "context";
            assert!(!super::is_context(target));
        }
    }
}
