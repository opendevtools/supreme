use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PKG: Regex = Regex::new(r"([\w@/-]+)\{([\w\-,]+)\}").unwrap();
}

fn split_packages(caps: regex::Captures) -> Option<Vec<String>> {
    let base = caps.get(1)?.as_str();

    Some(
        caps.get(2)?
            .as_str()
            .split(',')
            .map(|pkg| format!("{}{}", base, pkg))
            .collect(),
    )
}

pub fn packages(s: &str) -> Vec<String> {
    s.split_whitespace()
        .flat_map(|s| match PKG.captures(s) {
            Some(caps) => split_packages(caps).unwrap(),
            None => vec![s.to_string()],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_package_install() {
        assert_eq!(packages("prettier"), vec!["prettier"]);
    }

    #[test]
    fn parse_multiple_packages_install() {
        assert_eq!(packages("prettier meow"), vec!["prettier", "meow"]);
    }

    #[test]
    fn parse_multiple_packages_with_expansion() {
        assert_eq!(
            packages("prettier meow @testing-library/{react,cypress}"),
            vec![
                "prettier",
                "meow",
                "@testing-library/react",
                "@testing-library/cypress"
            ]
        );
    }

    #[test]
    fn parse_all_forms() {
        assert_eq!(
            packages(
                "prettier meow @testing-library/{react,jest-dom,react-hooks,cypress} eslint-plugin-{prettier,react}"
            ),
            vec![
                "prettier",
                "meow",
                "@testing-library/react",
                "@testing-library/jest-dom",
                "@testing-library/react-hooks",
                "@testing-library/cypress",
                "eslint-plugin-prettier",
                "eslint-plugin-react",
            ]
        );
    }
}
