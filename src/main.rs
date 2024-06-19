use std::process::Command;

fn main() -> Result<(), isize> {
    let mut cmd = std::env::args();
    cmd.next();
    let cmd = cmd.collect::<Vec<_>>();

    let Some(proc) = cmd.first().cloned() else {
        println!("You need to provide a valid program: keep-alive <program>");
        return Err(-1);
    };
    let mut args = cmd.iter().cloned();
    args.next();
    let args = args.collect::<Vec<_>>();

    println!("Keeping alive command: {:?}", cmd);

    while let Ok(output) = Command::new(&proc).args(&args).output() {
        if output.status.success() {
            return Ok(());
        }

        println!(
            "Child died with exit code {:?}, rebooting",
            output.status.code()
        );
    }

    Err(-2)
}
