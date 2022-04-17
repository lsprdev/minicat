use std::io::BufReader;
    use std::io::BufRead;
    use std::fs::File;
                    
    fn main() {   
        let f = File::open("poem.txt").unwrap();
        let file = BufReader::new(&f);
        println!("\n");
        for (num, line) in file.lines().enumerate() {
            let l = line.unwrap();
            println!("{} - {}", num, l);
        }           
        println!("\n");
    }
