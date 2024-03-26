use async_process::{Command, Stdio};
use std::io::{ Read, Write};
use std::thread::sleep;
use std::time;

async fn io_hander(mut stdin: async_process::ChildStdin, mut stdout: async_process::ChildStdout){
    // Read from stdout
    let mut buf = [0; 1024];
    while let Ok(n) = stdout.read(&mut buf).await {
        if n == 0 {
            break; // End of stream
        }
        // Process the output data (e.g., print it)
        println!("Child output: {:?}", &buf[..n]);
    }

    // Write to stdin (if needed)
    let input_data = "Your input data\n";
    stdin.write_all(input_data.as_bytes()).await.expect("Failed to write to stdin");
}

fn main(){
    println!("LLamaTricker: The LLama2 convincer!");

    //Create children
    let mut c1 = Command::new("/usr/local/bin/ollama")
        .stdin(async_process::Stdio::piped())
        .stdout(async_process::Stdio::piped())
        .arg("run")
        .arg("llama2")
        .spawn()
        .expect("failed to execute child");

    let c1er = c1.wait().expect("failed to wait on child");
    assert!(c1er.success());

    let input_text = "\\bye";
    if let Some(mut stdin) = c1.stdin.take() {
        stdin.write_all(input_text.as_bytes()).expect("Failed to write to stdin");
    }

    io_hander(c1.stdin.unwrap(), c1.stdout.unwrap());

    println!("Program complete!");
}