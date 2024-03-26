use std::process::Command;

fn main() {
    println!("LLamaTricker: The LLama2 convincer!");

    //Create children
    let mut c1 = Command::new("/usr/local/bin/ollama")
        .spawn()
        .expect("failed to execute child");

    let c1er = c1.wait().expect("failed to wait on child");
    assert!(c1er.success());
}
