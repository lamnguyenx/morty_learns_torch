#[macro_export]
macro_rules! dbgts {
    ($val:expr) => {{
        // Get the file and line information
        let file = std::file!();
        let line = std::line!();

        // Create a reference to the value

        let prefix = "    ";
        let value = format!("{}", &$val)
            .lines()
            .map(|line| format!("{}{}", prefix, line))
            .collect::<Vec<_>>()
            .join("\n");

        // Print the variable name, value, file, and line number
        println!("\n{}:{}\n{} =\n {}", file, line, stringify!(&$val), value);
    }};
}
