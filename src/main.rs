use std::io;
fn main () {
   println!("Ти в калькуляторі!!!");
   println!("Введи перше число: ");
   let mut t1 = String::new();
   io::stdin().read_line(&mut t1).unwrap();
   let t1: f64 = t1.trim().parse().unwrap();
   
   println!("обери дію яку будеш робити: ");
   println!("1: (+)");
   println!("2: (-)");
   println!("3: (*)");
   println!("4: (/)");
   let mut u = String::new();
   io::stdin().read_line(&mut u).unwrap();
   let clean_u = u.trim();
   

   println!("Введи друге число: ");
   let mut t2 = String::new();
   io::stdin().read_line(&mut t2).unwrap();
   let t2: f64 = t2.trim().parse().unwrap();
  
   match clean_u {
       "1" => println!("Ось твоя відповідь {}", t1 + t2),
       "2" => println!("Ось твоя відповідь {}", t1 - t2),
       "3" => println!("Ось твоя відповідь {}", t1 * t2),
       "4" => println!("Ось твоя відповідь {}", t1 / t2),
        _ => println!("Ти вів не число:("),
   }
}

