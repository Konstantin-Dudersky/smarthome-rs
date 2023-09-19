use env_vars::load_config;

fn main() {
    let config = load_config();
    println!("{:#?}", config);
}
