use std::{
    fs,
    process::{Command, Stdio},
};

fn capture_output(input: &str) -> (String, String, i32) {
    let output = Command::new("run_test.sh")
        .arg("run")
        .arg(input)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    (stdout, stderr, exit_code)
}

pub fn run_case(case: &str) {
    let (stdout, stderr, exit_code) = capture_output(format!("{case}/program.lox").as_str());

    let expected_out = fs::read_to_string(format!("{case}/out.txt").as_str()).unwrap();
    let expected_err = fs::read_to_string(format!("{case}/err.txt").as_str()).unwrap();
    let expected_code = fs::read_to_string(format!("{case}/code.txt").as_str())
        .unwrap()
        .parse::<i32>()
        .unwrap();

    assert_eq!(stdout, expected_out);
    assert_eq!(stderr, expected_err);
    assert_eq!(exit_code, expected_code);
}
