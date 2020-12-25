
pub fn f() {
    let card_key = 6930903;
    let door_key = 19716708;

    let card_loop = loop_size(card_key, 7);
    println!("{}", card_loop);
    let mut encryption_key = 1;
    for _ in 0..card_loop {
        encryption_key = transform(encryption_key, door_key);
    }
    println!("{}", encryption_key);
}

fn transform(n: u64, subject_number: u64) -> u64 {
    (n * subject_number) % 20201227
}

fn loop_size(pub_key: u64, subject_number: u64) -> u64 {
    let mut n = 1;
    let mut loop_size = 0;

    loop {
        loop_size += 1;
        n = transform(n, subject_number);
        if n == pub_key {
            break;
        }
    }

    loop_size
}
