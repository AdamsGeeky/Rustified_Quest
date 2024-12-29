fn main() {
    let zeros: [u8; 6] = [0, 0, 0, 0, 0, 0];
    let zeros_short: [u8; 6] = [0; 6];

    assert_eq!(zeros, zeros_short);

    let arr =  [1,2,3,4,5,6];
    let slice = &arr[2..5];
    println!("arr = {:?}", arr);
    println!("slice = {:?}", slice);

    let mut xs = [2,4,6];
    let _mut_slice = &mut xs[0..2];
}
