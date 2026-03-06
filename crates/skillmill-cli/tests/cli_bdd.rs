use assert_cmd::cargo::cargo_bin_cmd;
use cucumber::{World, given, then, when};

#[derive(Debug, Default, World)]
struct CliWorld {
    output: Option<std::process::Output>,
}

#[given("the skillmill CLI is available")]
fn cli_available(_world: &mut CliWorld) {}

#[when("I run \"skillmill --help\"")]
fn run_help(world: &mut CliWorld) {
    let output = cargo_bin_cmd!("skillmill")
        .arg("--help")
        .output()
        .expect("run skillmill --help");
    world.output = Some(output);
}

#[then("the command succeeds")]
fn command_succeeds(world: &mut CliWorld) {
    let output = world.output.as_ref().expect("output available");
    assert!(output.status.success(), "command failed");
}

#[then("the output includes \"skillmill\"")]
fn output_includes_skillmill(world: &mut CliWorld) {
    let output = world.output.as_ref().expect("output available");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.to_lowercase().contains("skillmill"));
}

fn main() {
    futures::executor::block_on(CliWorld::run("tests/features"));
}
