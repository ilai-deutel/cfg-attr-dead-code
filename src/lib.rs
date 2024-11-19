#[path = "util.rs"] mod util_lib;

mod util;

pub fn run() -> i32 {
    return util_lib::get_random_number();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(run(), 4);
    }
}
