mod helper;

#[cfg(test)]
mod inheritance {
    use crate::helper::run_case;

    #[test]
    fn class_hierarchy_1() {
        run_case("tests/test_cases/run/Inheritance/Class Hierarchy - 1");
    }

    #[test]
    fn class_hierarchy_2() {
        run_case("tests/test_cases/run/Inheritance/Class Hierarchy - 2");
    }

    #[test]
    fn class_hierarchy_3() {
        run_case("tests/test_cases/run/Inheritance/Class Hierarchy - 3");
    }

    #[test]
    fn class_hierarchy_4() {
        run_case("tests/test_cases/run/Inheritance/Class Hierarchy - 4");
    }

    #[test]
    fn inheriting_methods_1() {
        run_case("tests/test_cases/run/Inheritance/Inheriting Methods - 1");
    }

    #[test]
    fn inheriting_methods_2() {
        run_case("tests/test_cases/run/Inheritance/Inheriting Methods - 2");
    }

    #[test]
    fn inheriting_methods_3() {
        run_case("tests/test_cases/run/Inheritance/Inheriting Methods - 3");
    }

    #[test]
    fn inheriting_methods_4() {
        run_case("tests/test_cases/run/Inheritance/Inheriting Methods - 4");
    }
}
