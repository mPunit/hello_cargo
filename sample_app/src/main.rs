use core::str;

fn main() {
    let bytestring1: &[u8; 24] = b"thi \xE3\x82\x88 is a byte string";
    let bytestring2: &[u8; 26] = b"thi \xE3 \x82 \x88 is a byte string";

    println!("bytestring1 = {:?}", bytestring1);
    match str::from_utf8(bytestring1) {
        Ok(s) => println!("Successfull: {}", s),
        Err(e) => println!("Unsuccessfull: {}", e),
    }

    println!("");

    println!("bytestring2 = {:?}", bytestring2);
    match str::from_utf8(bytestring2) {
        Ok(s) => println!("Successfull: {}", s),
        Err(e) => println!("Unsuccessfull: {}", e),
    }

    /* OUTPUT
    bytestring1 = [116, 104, 105, 32, 227, 130, 136, 32, 105, 115, 32, 97, 32, 98, 121, 116, 101, 32, 115, 116, 114, 105, 110, 103]
    Successfull: thi よ is a byte string

    bytestring2 = [116, 104, 105, 32, 227, 32, 130, 32, 136, 32, 105, 115, 32, 97, 32, 98, 121, 116, 101, 32, 115, 116, 114, 105, 110, 103]
    Unsuccessfull: invalid utf-8 sequence of 1 bytes from index 4
    */
}
