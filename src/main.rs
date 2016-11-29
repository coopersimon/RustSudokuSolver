use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main()
{
      let mut sudoku = Vec::new();
      // load from file if necessary
      if let Some(file_name) = env::args().nth(1)
      {
            let file_contents = match read_file(file_name)
            {
                  Ok(s) => s,
                  Err(err) => 
                  {
                        println!("couldn't read file: {}", err);
                        return;
                  }
            };
            // copy sudoku into vector
            sudoku = file_contents.split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect();

            if sudoku.len() != 81
            {
                  println!("sudoku isn't correct size");
                  return;
            }
      }
      else
      {
            println!("Input sudoku. Use \'0\' to represent blanks.");
            // input sudoku
            for _ in 0..81
            {
                  let mut num = String::new();
                  io::stdin().read_line(&mut num)
                        .expect("failed to read");

                  let num: u32 = num.trim().parse()
                        .expect("not a number");

                  sudoku.push(num);
            }
      }

      // fill first row, recursively fill other rows
      let blank = Vec::new();
      if !fill_sudoku(&mut sudoku, 0, &blank)
      {
            println!("invalid sudoku");
      }
      else
      {
            print_sudoku(&sudoku);
      }
      // if returns true, print sudoku
      // if false, invalid

}

fn fill_sudoku(mut sudoku: &mut Vec<u32>, square: usize, row_blanks: &Vec<u32>) -> bool
{
      // check if we're done.
      if square >= 80
      {
            return true;
      }

      // possible values for the row's remaining blanks
      let input_blanks;

      // if at the start of a row, need to calculate missing values
      if square % 9 == 0
      {
            // work out missing numbers in row and fill vector
            let mut row_blanks = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            //let mut solve_row = start
            for x in square..(square + 9)
            {
                  // remove number
                  row_blanks.retain(|&y| y != sudoku[x]);
            }

            input_blanks = row_blanks.clone();
      }
      // otherwise just copy in from input
      else
      {
            input_blanks = row_blanks.clone();
      }

      // possible values for the square's remaining blanks
      let mut blanks = input_blanks.clone();

      // if blank is already filled...
      if sudoku[square] != 0
      {
            // simply check next square.
            return fill_sudoku(&mut sudoku, square + 1, &input_blanks);
      }

      // loop until return.
      loop
      {
            // if all options have been exhausted, finish looping
            if blanks.is_empty()
            {
                  sudoku[square] = 0;
                  return false;
            }

            sudoku[square] = blanks.pop().unwrap();

            // check if the value is invalid according to the column and box
            if !check_column(&sudoku, square % 9) ||
               !check_box(&sudoku, (square / 27) * 3, ((square % 9) / 3) * 3)
            {
                  continue;
            }

            // work out blanks for next square
            let mut next_blanks = input_blanks.clone();
            next_blanks.retain(|&x| x != sudoku[square]);

            // rest of row is valid, return true
            if fill_sudoku(&mut sudoku, square + 1, &next_blanks)
            {
                  return true;
            }
      }
}

// check the column is valid
fn check_column(sudoku: &Vec<u32>, col_num: usize) -> bool
{
      let mut found: Vec<u32> = Vec::new();
      for y in 0..9
      {
            let index = col_num + y*9;
            if found.contains(&sudoku[index])
            {
                  return false;
            }
            else if sudoku[index] != 0
            {
                  found.push(sudoku[index]);
            }
      }
      true
}

// check the box is valid
fn check_box(sudoku: &Vec<u32>, row_num: usize, col_num: usize) -> bool
{
      let mut found: Vec<u32> = Vec::new();
      let offset = row_num * 9;
      for x in 0..3
      {
            for y in 0..3
            {
                  let index = offset + col_num + x + y*9;
                  if found.contains(&sudoku[index])
                  {
                        return false;
                  }
                  else if sudoku[index] != 0
                  {
                        found.push(sudoku[index]);
                  }
            }
      }
      true
}

fn read_file(file_name: String) -> Result<String, String>
{
      let mut input_file = try!(File::open(file_name).map_err(|e| e.to_string()));
      let mut file_contents = String::new();
      try!(input_file.read_to_string(&mut file_contents).map_err(|e| e.to_string()));
      Ok(file_contents)
}

fn print_sudoku(sudoku: &Vec<u32>) -> bool
{
      if sudoku.len() != 81
      {
            return false;
      }

      for y in 0..9
      {
            let offset = y * 9;
            for x in 0..9
            {
                  print!("{} ", sudoku[offset + x]);
            }
            print!("\n");
      }

      true
}
