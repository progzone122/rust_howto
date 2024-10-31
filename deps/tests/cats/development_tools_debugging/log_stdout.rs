use env_logger::Target;

fn main() {
    env_logger::Builder::new().target(Target::Stdout).init();

    log::error!("This error has been printed to Stdout");
}

use rusty_fork::rusty_fork_test;

rusty_fork_test! {
#[test]
fn test() {
    main();
}
}
