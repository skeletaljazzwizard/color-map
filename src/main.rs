fn main() {
    let res = soup::run();
    match res {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            eprint!("{}", e);
            std::process::exit(1);
        }
    }
}
