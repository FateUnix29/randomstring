mod tests {
    #[cfg(test)]
    mod tests {
        //use super::*;

        #[test]
        fn test_random_vector_generation() {
            let len: usize = 53;
            let min: i64 = -5;
            let max: i64 = 5;
            let thing: Vec<i64> = randomvector::randvec(len, min, max);

            println!("length: {}, min: {}, max: {}", len, min, max);

            // print it
            let mut printstr: String = String::new();
            for i in thing {
                printstr.push_str(&i.to_string());
                printstr.push_str(", ");
            }
            // make it a slice and remove trailing ", "
            let printstr = printstr.as_str()[0..printstr.len() - 2].to_string();
            println!("{}", printstr);

            // Flush stdout
            use std::io::{stdout, Write};
            stdout().write_all(b"\n").unwrap();

            assert!(true);
        }
    }
}
