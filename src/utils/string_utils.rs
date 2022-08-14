//Taken from https://doc.rust-lang.org/src/alloc/string.rs.html#2517-2536, recoded to avoid importing it and reduce cart size
pub fn int_to_string(num: i32) -> String {
    let mut buf = String::with_capacity(4);
    if num < 0 {
        buf.push('-');
    }
    let mut n: u32 = if num < 0 { -num as u32 } else { num as u32 };

    if n >= 10 {
        if n >= 100 {
            if n >= 1000 {
                if n >= 10000 {
                    buf.push((b'0' + ((n / 10000) as u8)) as char);
                    n %= 10000;
                }
                buf.push((b'0' + ((n / 1000) as u8)) as char);
                n %= 1000;
            }
            buf.push((b'0' + ((n / 100) as u8)) as char);
            n %= 100;
        }
        buf.push((b'0' + ((n / 10) as u8)) as char);
        n %= 10;
    }
    buf.push((b'0' + (n as u8)) as char);
    buf
}
