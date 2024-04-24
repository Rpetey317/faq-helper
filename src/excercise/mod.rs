pub struct Excercise {
    assignature: String,
    tags: Vec<String>,
    path: String,
}

impl Excercise {
    fn new(assignature: String, path: String) -> Excercise {
        Excercise {
            assignature,
            tags: Vec::new(),
            path,
        }
    }

    fn add_tag(&mut self, tag: String) {
        self.tags.push(tag);
    }

    fn remove_tag(&mut self, tag: String) {
        self.tags.retain(|t| t != &tag);
    }

    fn is_for_assignature(&self, assignature: &str) -> bool {
        self.assignature == assignature
    }

    fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&tag.to_string())
    }

    fn get_path(&self) -> &str {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_exc_1() -> Excercise {
        Excercise::new(
            "Álgebra Lineal".to_string(),
            "tests/testfiles/exc1.md".to_string(),
        )
    }

    #[test]
    fn test_01_excercise_belongs_to_assignature() {
        let exc = test_exc_1();
        assert_eq!(exc.is_for_assignature("Álgebra Lineal"), true);
    }

    #[test]
    fn test_02_can_add_tags_to_excercise() {
        let mut exc = test_exc_1();
        exc.add_tag("Matrices".to_string());
        assert_eq!(exc.has_tag("Matrices"), true);
    }

    #[test]
    fn test_03_excercise_can_have_multiple_tags() {
        let mut exc = test_exc_1();
        exc.add_tag("Matrices".to_string());
        exc.add_tag("Determinantes".to_string());
        assert_eq!(exc.has_tag("Matrices"), true);
    }

    #[test]
    fn test_04_can_remove_tags_from_excercise() {
        let mut exc = test_exc_1();
        exc.add_tag("Matrices".to_string());
        assert_eq!(exc.has_tag("Matrices"), true);
        exc.remove_tag("Matrices".to_string());
        assert_eq!(exc.has_tag("Matrices"), false);
    }

    #[test]
    fn test_05_excercise_has_path() {
        let exc = test_exc_1();
        assert_eq!(exc.get_path(), "tests/testfiles/exc1.md");
    }
}
