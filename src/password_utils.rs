const PASSWORD_SIZE: usize = 32;

pub fn make_password_complient(src: &[u8]) -> [u8; PASSWORD_SIZE] {
    let mut result = [0u8; PASSWORD_SIZE];

    let mut pos = 0;
    let mut result_pos = 0;
    while result_pos < PASSWORD_SIZE {
        result[result_pos] = *src.get(pos).unwrap();

        pos += 1;
        result_pos += 1;

        if pos == src.len() {
            pos = 0;
        }
    }

    result
}
