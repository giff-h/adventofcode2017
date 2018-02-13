fn steps1(address: u32) -> u16 {
    if address == 1 { return 0 }

    let mut corner: u32 = 1;
    let mut square_index: u16 = 1;
    let edge_center: u32;
    let diff: u16;

    loop {
        for _ in 1..5 {
            corner += square_index as u32 * 2;
            if corner >= address {
                edge_center = corner - square_index as u32;
                diff = (address as i32 - edge_center as i32).abs() as u16;
                return square_index + diff
            }
        }
        square_index += 1;
    }
}

pub fn run() {
    assert_eq!(steps1(1), 0);
    assert_eq!(steps1(12), 3);
    assert_eq!(steps1(23), 2);
    assert_eq!(steps1(1024), 31);

    let address: u32 = 277678;
    println!("{}", steps1(address));
}
