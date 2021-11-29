#[macro_export] 
macro_rules! lire_entier {
    ($msg:expr) => {{
        use std::io::BufRead;
        println!("{}", $msg);
        let mut buffer = String::new();
        let stdin = std::io::stdin();
        {
            let mut handle = stdin.lock();
            loop {
                handle.read_line(&mut buffer).expect("read_line failed");
                if let Some(new_len) = buffer.as_str().find(&['\r', '\n'][..]) {
                    buffer.truncate(new_len);
                }
                if let Ok(x) = buffer.parse::<isize>() {
                    break x;
                } else {
                    println!("L'entrée n'est pas un entier. Veuillez rééssayer.");
                    buffer.clear();
                }
            }
        }
    }};
}
