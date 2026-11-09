// `sqlx::migrate!` embeds the migrations at compile time; rebuild when they change.
fn main() {
    println!("cargo:rerun-if-changed=migrations");
}
