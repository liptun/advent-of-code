type LevelsVec = Vec<u8>;

#[derive(Debug)]
pub enum ReportCreationError {
    ParseError,
    EmptyReport,
}

#[derive(Debug)]
pub enum ReportSafety {
    Safe,
    Unsafe,
}

#[derive(Debug, PartialEq)]
pub struct Report {
    pub levels: LevelsVec,
}

impl Report {
    pub fn new(levels: LevelsVec) -> Self {
        Self {
            levels
        }
    }

    pub fn parse(s: String) -> Result<Self, ReportCreationError> {
        let levels_parse: Vec<Result<u8, ReportCreationError>> = s
            .split_whitespace()
            .map(|s| s.parse().map_err(|_| ReportCreationError::ParseError))
            .collect();

        let mut levels: LevelsVec = Vec::new();

        for level in levels_parse {
            match level {
                Ok(l) => levels.push(l),
                Err(e) => return Err(e),
            }
        }

        if levels.len() == 0 {
            return Err(ReportCreationError::EmptyReport);
        }

        Ok(Self { levels })
    }

    fn test_inc_or_dec(&self) -> bool {
        let mut iter = self.levels.iter().peekable();
        let is_descending = self.levels[0] > self.levels[1];

        while let Some(a) = iter.next() {
            if let Some(b) = iter.peek() {
                if is_descending && a < b || !is_descending && a > b {
                    return false;
                }
            }
        }

        true
    }

    fn test_diff(&self) -> bool {
        let mut iter = self.levels.iter().peekable();

        while let Some(a) = iter.next() {
            if let Some(b) = iter.peek() {
                let diff = a.abs_diff(**b);
                if diff < 1 || diff > 3 {
                    return false;
                }
            }
        }

        true
    }

    pub fn test_safelty(&self) -> ReportSafety {
        if self.test_diff() == false ||  self.test_inc_or_dec() == false {
            return ReportSafety::Unsafe;
        }
        ReportSafety::Safe
    }

    pub fn test_safelty_with_problem_dampener(&self) -> ReportSafety {
        if let ReportSafety::Safe = self.test_safelty() {
            return ReportSafety::Safe;
        }

        for i in 0..self.levels.len() {
           let mut levels = self.levels.clone(); 
           levels.remove(i);
           let report = Report::new(levels);
           if let ReportSafety::Safe = report.test_safelty() {
               return ReportSafety::Safe;
           }
        }

        

        ReportSafety::Unsafe
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_creation_error_test() {
        assert!(matches!(
            Report::parse("a 6 4 2 1".to_string()),
            Err(ReportCreationError::ParseError)
        ));
        assert!(matches!(
            Report::parse("6 4 2 x 1".to_string()),
            Err(ReportCreationError::ParseError)
        ));
        assert!(matches!(
            Report::parse("6 4 2b 1".to_string()),
            Err(ReportCreationError::ParseError)
        ));
        assert!(matches!(
            Report::parse("6 4 3.14 1".to_string()),
            Err(ReportCreationError::ParseError)
        ));
        assert!(matches!(
            Report::parse("6 4 3,14 1".to_string()),
            Err(ReportCreationError::ParseError)
        ));
        assert!(matches!(
            Report::parse("6 4 -14 1".to_string()),
            Err(ReportCreationError::ParseError)
        ));
        assert!(matches!(
            Report::parse("a".to_string()),
            Err(ReportCreationError::ParseError)
        ));
        assert!(matches!(
            Report::parse(" ".to_string()),
            Err(ReportCreationError::EmptyReport)
        ));
    }

    #[test]
    fn report_creation_succes_test() {
        let report = Report::parse("7 6 4 2 1".to_string()).expect("Should create valid reports");
        assert_eq!(report.levels.len(), 5);
    }

    #[test]
    fn report_safelty_test() {
        assert!(matches!(
            Report::parse("7 6 4 2 1".to_string())
                .expect("Should create valid reports")
                .test_safelty(),
            ReportSafety::Safe
        ));
        assert!(matches!(
            Report::parse("1 2 7 8 9".to_string())
                .expect("Should create valid reports")
                .test_safelty(),
            ReportSafety::Unsafe
        ));
        assert!(matches!(
            Report::parse("9 7 6 2 1".to_string())
                .expect("Should create valid reports")
                .test_safelty(),
            ReportSafety::Unsafe
        ));
        assert!(matches!(
            Report::parse("1 3 2 4 5".to_string())
                .expect("Should create valid reports")
                .test_safelty(),
            ReportSafety::Unsafe
        ));
        assert!(matches!(
            Report::parse("8 6 4 4 1".to_string())
                .expect("Should create valid reports")
                .test_safelty(),
            ReportSafety::Unsafe
        ));
        assert!(matches!(
            Report::parse("1 3 6 7 9".to_string())
                .expect("Should create valid reports")
                .test_safelty(),
            ReportSafety::Safe
        ));
    }

    #[test]
    fn report_safelty_problem_dampener() {
        assert!(matches!(
            Report::parse("7 6 4 2 1".to_string())
                .expect("Should create valid reports")
                .test_safelty_with_problem_dampener(),
            ReportSafety::Safe
        ));
        assert!(matches!(
            Report::parse("1 2 7 8 9".to_string())
                .expect("Should create valid reports")
                .test_safelty_with_problem_dampener(),
            ReportSafety::Unsafe
        ));
        assert!(matches!(
            Report::parse("9 7 6 2 1".to_string())
                .expect("Should create valid reports")
                .test_safelty_with_problem_dampener(),
            ReportSafety::Unsafe
        ));
        assert!(matches!(
            Report::parse("1 3 2 4 5".to_string())
                .expect("Should create valid reports")
                .test_safelty_with_problem_dampener(),
            ReportSafety::Safe
        ));
        assert!(matches!(
            Report::parse("8 6 4 4 1".to_string())
                .expect("Should create valid reports")
                .test_safelty_with_problem_dampener(),
            ReportSafety::Safe
        ));
        assert!(matches!(
            Report::parse("1 3 6 7 9".to_string())
                .expect("Should create valid reports")
                .test_safelty_with_problem_dampener(),
            ReportSafety::Safe
        ));
    }
}
