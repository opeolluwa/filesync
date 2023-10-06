pub fn execute_shell_command(command: &str) -> std::io::Result<String> {
    let mut cmd = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .spawn()?;

    let stdout = cmd.stdout.take().unwrap();
    let reader = std::io::BufReader::new(stdout);

    let output = std::io::BufRead::lines(reader)
        .collect::<std::io::Result<Vec<String>>>()?
        .join("\n");

    let status = cmd.wait()?;

    if status.success() {
        Ok(output)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Command '{}' failed with exit code: {}", command, status),
        ))
    }
}
