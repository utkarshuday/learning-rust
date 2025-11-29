use std::collections::HashMap;

pub struct School {
    students: HashMap<String, u32>,
}

impl School {
    pub fn new() -> School {
        Self {
            students: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if !self.students.contains_key(student) {
            self.students.insert(student.to_string(), grade);
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut ans = self.students.values().copied().collect::<Vec<_>>();
        ans.sort();
        ans.dedup();

        ans
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut x = self
            .students
            .iter()
            .filter(|(_, grade_value)| **grade_value == grade)
            .map(|(name_key, _)| name_key.to_string())
            .collect::<Vec<String>>();
        x.sort();

        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grade_is_empty_if_no_students_in_the_roster() {
        let s = School::new();
        assert_eq!(s.grade(1), Vec::<String>::new())
    }
    #[test]
    #[ignore]
    fn grade_is_empty_if_no_students_in_that_grade() {
        let mut s = School::new();
        s.add(2, "Peter");
        s.add(2, "Zoe");
        s.add(2, "Alex");
        s.add(3, "Jim");
        assert_eq!(s.grade(1), Vec::<String>::new())
    }
    #[test]
    #[ignore]
    fn student_not_added_to_same_grade_more_than_once() {
        let mut s = School::new();
        s.add(2, "Blair");
        s.add(2, "James");
        s.add(2, "James");
        s.add(2, "Paul");
        assert_eq!(s.grade(2), vec!["Blair", "James", "Paul"])
    }
    #[test]
    #[ignore]
    fn student_not_added_to_multiple_grades() {
        let mut s = School::new();
        s.add(2, "Blair");
        s.add(2, "James");
        s.add(3, "James");
        s.add(3, "Paul");
        assert_eq!(s.grade(2), vec!["Blair", "James"])
    }
    #[test]
    #[ignore]
    fn student_not_added_to_other_grade_for_multiple_grades() {
        let mut s = School::new();
        s.add(2, "Blair");
        s.add(2, "James");
        s.add(3, "James");
        s.add(3, "Paul");
        assert_eq!(s.grade(3), vec!["Paul"])
    }
    #[test]
    #[ignore]
    fn students_are_sorted_by_name_in_a_grade() {
        let mut s = School::new();
        s.add(5, "Franklin");
        s.add(5, "Bradley");
        s.add(1, "Jeff");
        assert_eq!(s.grade(5), vec!["Bradley", "Franklin"])
    }
    #[test]
    #[ignore]
    fn grades_for_empty_school() {
        let s = School::new();
        assert_eq!(s.grades(), vec![])
    }
    #[test]
    #[ignore]
    fn grades_for_one_student() {
        let mut s = School::new();
        s.add(2, "Aimee");
        assert_eq!(s.grades(), vec![2])
    }
    #[test]
    #[ignore]
    fn grades_for_several_students_are_sorted() {
        let mut s = School::new();
        s.add(2, "Aimee");
        s.add(7, "Logan");
        s.add(4, "Blair");
        assert_eq!(s.grades(), vec![2, 4, 7])
    }
    #[test]
    #[ignore]
    fn grades_when_several_students_have_the_same_grade() {
        let mut s = School::new();
        s.add(2, "Aimee");
        s.add(2, "Logan");
        s.add(2, "Blair");
        assert_eq!(s.grades(), vec![2])
    }
}
