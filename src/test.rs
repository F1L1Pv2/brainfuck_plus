// use super::*;

macro_rules! check_example {
    ($filename: literal, $expected_output: literal) => {
        std::process::Command::new("cargo")
            .args(["run", format!("./examples/{}.bf", $filename).as_str()])
            .output()
            .expect("failed to execute process");

        // pray that it compiles

        let output = std::process::Command::new(format!("./examples/{}", $filename))
            .output()
            .expect("failed to execute process");

        // compare the status code
        assert!(output.status.success());

        // compare the output
        assert_eq!(output.stdout, $expected_output.as_bytes());
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn hello_world() {
        check_example!("hello", "Hello World!\n");
    }
    #[test]
    fn hello_world_tapes() {
        check_example!("hello_using_tapes", "Hello, World!\n");
    }
    #[test]
    fn hello_world_syscall() {
        check_example!("hello_syscall", "Hello, World!\n");
    }
    #[test]
    fn test_stack() {
        check_example!("test_stack", "0");
    }
    #[test]
    fn ifdef() {
        check_example!("ifdef", "Y\nL\n");
    }

    
}
