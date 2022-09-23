use std::fmt;
use std::io;

enum Symbol {
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    Unknown,
}

fn str_to_symbol(s: &str) -> Result<Symbol, ()> {
    match s {
        "1" => Ok(Symbol::S1),
        "2" => Ok(Symbol::S2),
        "3" => Ok(Symbol::S3),
        "4" => Ok(Symbol::S4),
        "5" => Ok(Symbol::S5),
        "6" => Ok(Symbol::S6),
        "7" => Ok(Symbol::S7),
        "8" => Ok(Symbol::S8),
        "9" => Ok(Symbol::S9),
        _ => Err(()),
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::S1 => write!(f, "1"),
            Symbol::S2 => write!(f, "2"),
            Symbol::S3 => write!(f, "3"),
            Symbol::S4 => write!(f, "4"),
            Symbol::S5 => write!(f, "5"),
            Symbol::S6 => write!(f, "6"),
            Symbol::S7 => write!(f, "7"),
            Symbol::S8 => write!(f, "8"),
            Symbol::S9 => write!(f, "9"),
            Symbol::Unknown => write!(f, " "),
        }
    }
}

fn print_sudoku(grid: &Vec<Symbol>, col_chars: &[char; 9], row_chars: &[char; 9]) {
    print!("    │");
    for (i, val) in col_chars.iter().enumerate() {
        if i > 0 && i % 3 == 0 {
            print!("  │");
        }
        print!("  {}", val);
    }
    println!();
    println!("────┼───────────┼───────────┼──────────");
    let mut cur_col = 0;
    let mut cur_row = 0;
    for i in grid {
        if cur_col == 0 {
            print!(" {}  ", row_chars[cur_row]);
        }
        if cur_col % 3 == 0 {
            print!("│  ");
        }
        print!("{}  ", i);
        cur_col = cur_col + 1;
        if cur_col == 9 {
            cur_col = 0;
            cur_row = cur_row + 1;
            println!();
            if cur_row == 9 {
                println!("────┴───────────┴───────────┴──────────");
            } else if cur_row % 3 == 0 {
                println!("────┼───────────┼───────────┼──────────");
            }
        }
    }
}

fn main() {
    let col_chars = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I'];
    let row_chars = ['J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R'];

    let mut given: Vec<Symbol> = Vec::new();
    for _x in 0..81 {
        given.push(Symbol::Unknown);
    }

    println!();
    println!("Willkommen zu Rustic Sudocu!");
    println!();
    print_sudoku(&given, &col_chars, &row_chars);
    println!();

    loop {
        println!("Welche Zahlen sind vorgegeben? (z. B. \"BL9\")");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // TODO: validate if 3 chars are entered
        let input = input.to_uppercase();
        println!("Enterred: {}", input);
        let mut input_iterate = input.chars();
        let col_char = input_iterate.next().unwrap();
        let col = col_chars.iter().position(|&r| r == col_char).unwrap();
        let row_char = input_iterate.next().unwrap();
        let row = row_chars.iter().position(|&r| r == row_char).unwrap();
        let target_index = row * 9 + col;
        let val = str_to_symbol(input_iterate.next().unwrap().to_string().as_str());
        let val = match val {
            Ok(symbol) => symbol,
            Err(error) => panic!("{:?} is not a valid number for Sudocus", error),
        };

        println!(
            "Column: {}, Row: {}, Number: {} (index: {})",
            col, row, val, target_index
        );
        println!();

        given.remove(target_index);
        given.insert(target_index, val);

        print_sudoku(&given, &col_chars, &row_chars);
    }
}
