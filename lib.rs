
pub mod filerw {
    use std::fs::File;
    use std::io::BufReader;

 pub fn rf (f:&str) ->  BufReader<File> {
                         let f1= File::open(f).expect("could not open the file");
                         let _f3 = BufReader::new(f1);
                         return _f3;
                       }

 pub fn wf (f:&str) -> File {
     let f1 = File::create(f).expect("unable to create a file");
     return f1;
 }
}




pub mod system_date {

extern crate chrono;
use chrono::{DateTime, Utc};

pub fn get_date() -> (i32,i32,i32) {
let now: DateTime<Utc> = Utc::now();
let Y = now.format("%Y").to_string();
let D = now.format("%d").to_string();
let M = now.format("%m").to_string();

let y: i32 = Y.parse().unwrap();
let m: i32 = M.parse().unwrap();
let d: i32 = D.parse().unwrap();
return (y,m,d);
}
}




pub mod oper {
use std::fs::File;
use std::io::Write;
use std::io::{BufReader, BufRead};
struct Person {
dob: String,
Fin: String,
Lan: String,

}
pub fn line_by_line (f3:BufReader<File>,f1:&mut File,y:i32,m:i32,d:i32) {

for line in f3.lines() {

 let s = line.unwrap();
  //println!("{}",s);
  let tokens: Vec<&str> = s.split(",").collect();

  let person = Person { dob: String::from(tokens[2]),
     Fin: String::from(tokens[1]),
     Lan: String::from(tokens[0]),
    };

  let tokens2: Vec<&str> = (person.dob).split("-").collect();

  let year: i32 = tokens2[0].parse().unwrap();
  let mon: i32 = tokens2[1].parse().unwrap();
  let dat: i32 = tokens2[2].parse().unwrap();


  let mut age = y - year;
  age = age - 1;
  if mon < m {
      age = age + 1;
  }

  if mon == m {
      if dat < d {
          age = age + 1;
      }
  }
  writeln!(f1, "{}, {}, {}", person.Fin, person.Lan, age).expect("could not write into the file");
}
}
}
