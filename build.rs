use vergen_gitcl::{Emitter, GitclBuilder};

fn main() {
    // Configure git instructions
    let git = GitclBuilder::default()
        .branch(true)
        .commit_date(true)
        .commit_message(true)
        .commit_timestamp(true)
        .describe(true, true, None)
        .sha(true)
        .build()
        .unwrap();

    // Emit the instructions
    Emitter::default()
        .add_instructions(&git)
        .unwrap()
        .emit()
        .unwrap();
}
