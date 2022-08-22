// Générer le n-ième nombre de Fibonacci.

fn main() {
    println!("La {} valeur de la suite de fibonacci est {}",5 ,fibonacci(5));
}

fn fibonacci(n: i32) -> i32 {
    let mut a = 1;
    let mut b = 2;
    for _i in 1..n {
        a = b;
        b = a + b;
    }
    return a;
}