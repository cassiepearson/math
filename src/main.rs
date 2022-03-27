mod errors;
mod general;
mod number_theory;

use git_version::git_version;

const GIT_VERSION: &str = git_version!();

fn main() {
    println!("Version: {:?}", GIT_VERSION);
}
