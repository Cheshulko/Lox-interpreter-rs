mod helper;

#[cfg(test)]
mod functions {
    use crate::helper::run_case;

    #[test]
    fn functions_without_arguments_1() {
        run_case("tests/test_cases/run/Functions/Functions without arguments - 1");
    }

    #[test]
    fn functions_without_arguments_2() {
        run_case("tests/test_cases/run/Functions/Functions without arguments - 2");
    }

    #[test]
    fn functions_without_arguments_3() {
        run_case("tests/test_cases/run/Functions/Functions without arguments - 3");
    }

    #[test]
    fn functions_without_arguments_4() {
        run_case("tests/test_cases/run/Functions/Functions without arguments - 4");
    }

    #[test]
    fn functions_with_arguments_1() {
        run_case("tests/test_cases/run/Functions/Functions with arguments - 1");
    }

    #[test]
    fn functions_with_arguments_2() {
        run_case("tests/test_cases/run/Functions/Functions with arguments - 2");
    }

    #[test]
    fn functions_with_arguments_3() {
        run_case("tests/test_cases/run/Functions/Functions with arguments - 3");
    }

    #[test]
    fn functions_with_arguments_4() {
        run_case("tests/test_cases/run/Functions/Functions with arguments - 4");
    }

    #[test]
    fn syntax_errors_1() {
        run_case("tests/test_cases/run/Functions/Syntax errors - 1");
    }

    #[test]
    fn syntax_errors_2() {
        run_case("tests/test_cases/run/Functions/Syntax errors - 2");
    }

    #[test]
    fn syntax_errors_3() {
        run_case("tests/test_cases/run/Functions/Syntax errors - 3");
    }

    #[test]
    fn syntax_errors_4() {
        run_case("tests/test_cases/run/Functions/Syntax errors - 4");
    }

    #[test]
    fn return_statements_1() {
        run_case("tests/test_cases/run/Functions/Return statements - 1");
    }

    #[test]
    fn return_statements_2() {
        run_case("tests/test_cases/run/Functions/Return statements - 2");
    }

    #[test]
    fn return_statements_3() {
        run_case("tests/test_cases/run/Functions/Return statements - 3");
    }

    #[test]
    fn return_statements_4() {
        run_case("tests/test_cases/run/Functions/Return statements - 4");
    }

    #[test]
    fn higher_order_functions_1() {
        run_case("tests/test_cases/run/Functions/Higher order functions - 1");
    }

    #[test]
    fn higher_order_functions_2() {
        run_case("tests/test_cases/run/Functions/Higher order functions - 2");
    }

    #[test]
    fn higher_order_functions_3() {
        run_case("tests/test_cases/run/Functions/Higher order functions - 3");
    }

    #[test]
    fn higher_order_functions_4() {
        run_case("tests/test_cases/run/Functions/Higher order functions - 4");
    }

    #[test]
    fn runtime_errors_1() {
        run_case("tests/test_cases/run/Functions/Runtime errors - 1");
    }

    #[test]
    fn runtime_errors_2() {
        run_case("tests/test_cases/run/Functions/Runtime errors - 2");
    }

    #[test]
    fn runtime_errors_3() {
        run_case("tests/test_cases/run/Functions/Runtime errors - 3");
    }

    #[test]
    fn runtime_errors_4() {
        run_case("tests/test_cases/run/Functions/Runtime errors - 4");
    }

    #[test]
    fn function_scope_1() {
        run_case("tests/test_cases/run/Functions/Function scope - 1");
    }

    #[test]
    fn function_scope_2() {
        run_case("tests/test_cases/run/Functions/Function scope - 2");
    }

    #[test]
    fn function_scope_3() {
        run_case("tests/test_cases/run/Functions/Function scope - 3");
    }

    #[test]
    fn function_scope_4() {
        run_case("tests/test_cases/run/Functions/Function scope - 4");
    }

    #[test]
    fn closures_1() {
        run_case("tests/test_cases/run/Functions/Closures - 1");
    }

    #[test]
    fn closures_2() {
        run_case("tests/test_cases/run/Functions/Closures - 2");
    }

    #[test]
    fn closures_3() {
        run_case("tests/test_cases/run/Functions/Closures - 3");
    }

    #[test]
    fn closures_4() {
        run_case("tests/test_cases/run/Functions/Closures - 4");
    }
}
