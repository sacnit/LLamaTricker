use std::process::{Command, Stdio};
use std::io::{self, Read, Write};

fn main(){
    println!("LLamaTricker: The LLama2 convincer!");

    //Create children
    let mut c1 = Command::new("/usr/local/bin/ollama")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("run")
        .arg("llama2")
        .spawn()
        .expect("failed to execute child");

    let c1er = c1.wait().expect("failed to wait on child");
    assert!(c1er.success());

    let input_text = "Hello, Ollama!";
    if let Some(mut stdin) = c1.stdin.take() {
        stdin.write_all(input_text.as_bytes()).expect("Failed to write to stdin");
    }

    let status = c1.wait().expect("Failed to wait for Ollama process");

    let mut output_text = String::new();
    if let Some(mut stdout) = c1.stdout.take() {
        let mut buffer = [0; 1]; // Read one byte at a time
        loop {
            match stdout.read(&mut buffer) {
                Ok(0) => break, // End of output
                Ok(_) => output_text.push(buffer[0] as char),
                Err(err) => {
                    eprintln!("Error reading from stdout: {}", err);
                    break;
                }
            }
        }
    } else {
        eprintln!("Failed to open stdout for Ollama process");
    }

    let status = c1.wait().expect("Failed to wait for Ollama process");
    if status.success() {
        println!("Ollama output: {}", output_text);
    } else {
        println!("Ollama process exited with an error");
    }
}