use tfhe::{generate_keys, ConfigBuilder};

fn main() {
    let config = ConfigBuilder::default().build();
    let _ = generate_keys(config);

    println!("Done.");
}
