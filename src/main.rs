use std::process::Command;

fn main() -> Result<(), isize> {
    let mut cmd = std::env::args();
    cmd.next();
    let cmd = cmd.collect::<Vec<_>>();
    // Extra argument that restarts the application only when the application has failed
    let failed = cmd
        .iter()
        .next()
        .map(|arg| *arg == "--failed")
        .unwrap_or(false);

    let Some(proc) = cmd.first().cloned() else {
        println!("You need to provide a valid program: keep-alive [--failed] <program>");
        return Err(-1);
    };
    let mut args = cmd.iter().cloned();
    args.next();
    if failed {
        args.next();
    }
    let args = args.collect::<Vec<_>>();

    println!("Keeping alive command: {:?}", cmd);

    while let Ok(output) = Command::new(&proc).args(&args).output() {
        if failed && output.status.success() {
            return Ok(());
        }

        println!(
            "Child died with exit code {:?}, rebooting",
            output.status.code()
        );
    }

    Err(-2)
}
