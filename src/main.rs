use std::io;
fn main () {
   println!("Ти в калькуляторі!!!");
   println!("Введи перше число: ");
   let t1: f64 = loop {
   let mut t1 = String::new();
   io::stdin().read_line(&mut t1).expect("Помилка вводу");

   match t1.trim().parse() {
   Ok(number)=> break number,
   Err(_)=> println!("Напиши число!!!"),
}
};
   
   
   
   println!("обери дію яку будеш робити: ");
   println!("1: (+)");
   println!("2: (-)");
   println!("3: (*)");
   println!("4: (/)");
   let u: u32 = loop {
   let mut u = String::new();
   io::stdin().read_line(&mut u);

   match u.trim().parse() {
   Ok(number)=> break number,
   Err(_)=> println!("Напиши число!!!"),
}
};
   
   


   println!("Введи друге число: ");
   let t2: f64 = loop {
   let mut t2 = String::new();
   io::stdin().read_line(&mut t2);

   match t2.trim().parse() {
   Ok(number)=> break number,
   Err(_)=> println!("Напиши число!!!"),
}
};
  
   match u {
       1 => println!("Ось твоя відповідь {}", t1 + t2),
       2 => println!("Ось твоя відповідь {}", t1 - t2),
       3 => println!("Ось твоя відповідь {}", t1 * t2),
       4 => println!("Ось твоя відповідь {}", t1 / t2),
        _ => println!("Ти вів не число:("),
   }
}

