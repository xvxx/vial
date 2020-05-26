fn main() {
    println!(
        "cargo:rustc-env=BUILD_DATE={}",
        sh("date +%Y-%m-%d_%H:%M:%S%p")
    );
}

fn sh(args: &str) -> String {
    use std::process::Command;

    let args: Vec<_> = args.split(' ').collect();
    let cmd = args[0];
    let args: Vec<_> = args.iter().skip(1).collect();

    if let Ok(output) = Command::new(cmd).args(&args).output() {
        if !output.status.success() {
            eprintln!("Error running {} {:?}", cmd, args);
            return "???".to_string();
        }
        String::from_utf8(output.stdout).unwrap()
    } else {
        String::new()
    }
}
