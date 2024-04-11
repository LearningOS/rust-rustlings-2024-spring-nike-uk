// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

pub struct ReportCard<T> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

pub trait Grade {
    fn print_grade(&self) -> String;
}

impl Grade for f32 {
    fn print_grade(&self) -> String {
        format!("{:.1}", self)
    }
}

impl Grade for &'static str {
    fn print_grade(&self) -> String {
        self.to_string()
    }
}

impl<T: Grade> ReportCard<T> {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name,
            &self.student_age,
            self.grade.print_grade()
        )
    }
}
