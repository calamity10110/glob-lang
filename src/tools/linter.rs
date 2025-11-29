// Linter for Universal Language (simplified version)

#[derive(Debug, Clone, PartialEq)]
pub enum LintLevel {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone)]
pub struct LintIssue {
    pub level: LintLevel,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub rule: String,
    pub auto_fix: Option<String>,
}

// Linter module - checks code for common issues
#[allow(dead_code)]
pub struct Linter {
    issues: Vec<LintIssue>,
}

impl Default for Linter {
    fn default() -> Self {
        Self::new()
    }
}

impl Linter {
    pub fn new() -> Self {
        Linter { issues: Vec::new() }
    }

    pub fn lint_file(&mut self, content: &str) -> Vec<LintIssue> {
        self.issues.clear();

        for (line_num, line) in content.lines().enumerate() {
            // Check for naming conventions
            if line.contains("def ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let name = parts[1];
                    if !self.is_snake_case(name) {
                        self.add_issue(
                            LintLevel::Warning,
                            format!("Variable '{}' should use snake_case", name),
                            line_num + 1,
                            0,
                            "naming-convention",
                            Some(self.to_snake_case(name)),
                        );
                    }
                }
            }

            // Check for trailing whitespace
            if line.ends_with(' ') || line.ends_with('\t') {
                self.add_issue(
                    LintLevel::Info,
                    "Line has trailing whitespace".to_string(),
                    line_num + 1,
                    line.len(),
                    "trailing-whitespace",
                    Some(line.trim_end().to_string()),
                );
            }
        }

        self.issues.clone()
    }

    fn is_snake_case(&self, name: &str) -> bool {
        name.chars()
            .all(|c| c.is_lowercase() || c.is_numeric() || c == '_')
    }

    fn to_snake_case(&self, name: &str) -> String {
        let mut result = String::new();
        for (i, c) in name.chars().enumerate() {
            if c.is_uppercase() {
                if i > 0 {
                    result.push('_');
                }
                result.push(c.to_lowercase().next().unwrap());
            } else {
                result.push(c);
            }
        }
        result
    }

    fn add_issue(
        &mut self,
        level: LintLevel,
        message: String,
        line: usize,
        column: usize,
        rule: &str,
        auto_fix: Option<String>,
    ) {
        self.issues.push(LintIssue {
            level,
            message,
            line,
            column,
            rule: rule.to_string(),
            auto_fix,
        });
    }

    pub fn get_issues(&self) -> &[LintIssue] {
        &self.issues
    }

    pub fn has_errors(&self) -> bool {
        self.issues
            .iter()
            .any(|issue| issue.level == LintLevel::Error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linter_creation() {
        let linter = Linter::new();
        assert_eq!(linter.issues.len(), 0);
    }

    #[test]
    fn test_naming_convention() {
        let mut linter = Linter::new();
        let code = "def BadName = 42";

        let issues = linter.lint_file(code);
        assert!(issues.iter().any(|i| i.rule == "naming-convention"));
    }

    #[test]
    fn test_snake_case_check() {
        let linter = Linter::new();
        assert!(linter.is_snake_case("valid_name"));
        assert!(linter.is_snake_case("another_valid_123"));
        assert!(!linter.is_snake_case("InvalidName"));
        assert!(!linter.is_snake_case("badName"));
    }

    #[test]
    fn test_to_snake_case() {
        let linter = Linter::new();
        assert_eq!(linter.to_snake_case("CamelCase"), "camel_case");
        assert_eq!(linter.to_snake_case("HTTPServer"), "h_t_t_p_server");
    }

    #[test]
    fn test_trailing_whitespace() {
        let mut linter = Linter::new();
        let code = "def x = 10  \ndef y = 20";

        let issues = linter.lint_file(code);
        assert!(issues.iter().any(|i| i.rule == "trailing-whitespace"));
    }
}
