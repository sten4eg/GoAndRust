fn main() {
    for _i in 0..100_000_000 {
        let mut q: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let length = q.len() as i16;
        let mut data: Vec<u8> = vec![0, 0];

        data.append(&mut q);
        data[0] = (length & 0xff) as u8;
        data[1] = (length >> 8) as u8;
    }

    println!("ok");
}
