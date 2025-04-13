mod helper;

#[cfg(test)]
mod resolving_and_binding {
    use crate::helper::run_case;

    #[test]
    fn identifier_resolution_1() {
        run_case("tests/test_cases/run/Resolving & Binding/Identifier Resolution - 1");
    }

    #[test]
    fn identifier_resolution_2() {
        run_case("tests/test_cases/run/Resolving & Binding/Identifier Resolution - 1");
    }

    #[test]
    fn identifier_resolution_3() {
        run_case("tests/test_cases/run/Resolving & Binding/Identifier Resolution - 1");
    }

    #[test]
    fn identifier_resolution_4() {
        run_case("tests/test_cases/run/Resolving & Binding/Identifier Resolution - 1");
    }

    #[test]
    fn self_initialization_1() {
        run_case("tests/test_cases/run/Resolving & Binding/Self Initialization - 1");
    }

    #[test]
    fn self_initialization_2() {
        run_case("tests/test_cases/run/Resolving & Binding/Self Initialization - 2");
    }

    #[test]
    fn self_initialization_3() {
        run_case("tests/test_cases/run/Resolving & Binding/Self Initialization - 3");
    }

    #[test]
    fn self_initialization_4() {
        run_case("tests/test_cases/run/Resolving & Binding/Self Initialization - 4");
    }

    #[test]
    fn variable_redeclaration_1() {
        run_case("tests/test_cases/run/Resolving & Binding/Variable Redeclaration - 1");
    }

    #[test]
    fn variable_redeclaration_2() {
        run_case("tests/test_cases/run/Resolving & Binding/Variable Redeclaration - 2");
    }

    #[test]
    fn variable_redeclaration_3() {
        run_case("tests/test_cases/run/Resolving & Binding/Variable Redeclaration - 3");
    }

    #[test]
    fn variable_redeclaration_4() {
        run_case("tests/test_cases/run/Resolving & Binding/Variable Redeclaration - 4");
    }

    #[test]
    fn invalid_return_1() {
        run_case("tests/test_cases/run/Resolving & Binding/Invalid Return - 1");
    }

    #[test]
    fn invalid_return_2() {
        run_case("tests/test_cases/run/Resolving & Binding/Invalid Return - 2");
    }

    #[test]
    fn invalid_return_3() {
        run_case("tests/test_cases/run/Resolving & Binding/Invalid Return - 3");
    }

    #[test]
    fn invalid_return_4() {
        run_case("tests/test_cases/run/Resolving & Binding/Invalid Return - 4");
    }
}
