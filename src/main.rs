fn main() {
    if let Err(err) = nucle::run() {
        panic!("{}", err);
    }
}
