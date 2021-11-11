fn main() {
    println!("Hello, world!");
    let x={let y=2;y*5};
    another_function(x,6);
    let x=five();
    let x=plus_one(x);
    println!("А теперь {}",x);
}

fn another_function(x:i32,y:i32){
    println!("Еще одна функция");
    println!("Значение x равно {}",x);
    println!("Значение y равно {}",y);
}

fn five()->i32{
    5
}
fn plus_one(x:i32)->i32{
    x+1
}