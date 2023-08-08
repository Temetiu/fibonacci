fn main() {
    println!("n-th Fibonacci number");

    let fibo_number: u32 = fibo_classic(2);
    println!("{fibo_number}");
}


// classic method with a while loop
fn fibo_classic(n: usize) -> u32 {
    let mut fibo_serie: Vec<u32> =  Vec::new();
    fibo_serie.push(1);
    fibo_serie.push(1);
    for i in 2..n {
        fibo_serie.push(fibo_serie[i - 1] + fibo_serie[i - 2]);
    }
    return fibo_serie[n as usize]
}

// recursive style
fn fibo(n: u32) -> u32 {
    let mut nth_value: u32 = 0;
    if n == 0 || n == 1 {
        nth_value = 1;
    }
    else {
        nth_value += fibo(n-1);
    }
    nth_value
}