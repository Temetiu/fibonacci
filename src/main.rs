fn main() {
    println!("n-th Fibonacci number");

    let mut index: usize = 20;
    
    for i in 0..index+1 {
        let fibo_number: usize = fibo_classic(i);
        println!("Fibo_{i} = {fibo_number}");
    }
}

// method with loop
fn fibo_classic(n: usize) -> usize {
    let mut fibo_n0: usize = 1;
    let mut fibo_n1: usize = 1;
    let mut index: usize = 3;
    if n == 0 {
        0
    }
    else if n == 1 || n == 2 {
        1
    }
    else {
        let mut fibo_number: usize = 0;
        while index < n + 1 {
            fibo_number = fibo_n0 + fibo_n1;
            fibo_n0 = fibo_n1;
            fibo_n1 = fibo_number;
            index += 1;
        }
        fibo_number
    }
}

// method with a while loop and vector to store each value
fn fibo_vector(n: usize) -> u32 {
    let mut fibo_serie: Vec<u32> =  vec![0; n];
    //fibo_serie.push(1);
    //fibo_serie.push(1);
    fibo_serie[0] = 1;
    fibo_serie[0] = 1;
    for i in 2..n {
        println!("i = {i}");
        //fibo_serie.push(fibo_serie[i - 1] + fibo_serie[i - 2]);
        fibo_serie[i] = fibo_serie[i - 1] + fibo_serie[i - 2];
    }
    return fibo_serie[n as usize]
}

// recursive style
fn fibo_recursive(n: u32) -> u32 {
    let mut nth_value: u32 = 0;
    if n == 0 || n == 1 {
        nth_value = 1;
    }
    else {
        nth_value += fibo_recursive(n-1);
    }
    nth_value
}