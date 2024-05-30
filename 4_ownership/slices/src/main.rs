fn main() {
    let str = "abcHello Worldcba";

    let slice = slicer(str);
    println!("{slice}");

    let slice = slicer(slice); // shadow
    println!("{slice}");

    let slice = slicer(slice);
    println!("{slice}");
}

fn slicer(s: &str) -> &str {
    let bytes = s.as_bytes();

    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == bytes[bytes.len() - 1] {
            return &s[i + 1..bytes.len() - 1];
        }
        i += 1;
    }
    s
}
