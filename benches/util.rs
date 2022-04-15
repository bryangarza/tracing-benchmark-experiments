use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub(crate) struct RandString {
    pub length: usize,
    pub s: String,
}

pub(crate) fn gen_rand_strings() -> Vec<RandString> {
    let mut strings = vec![];
    for length in [8, 16, 256, 512, 1024, 2048, 4096].iter() {
        let rand_string = RandString {
            length: *length,
            s: gen_rand_string(*length),
        };
        strings.push(rand_string)
    }
    strings
}

fn gen_rand_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
