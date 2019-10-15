fn fibo_imperative(n: u32) -> u32 {
    let mut res = 1;
    let mut prev = 0;

    if n == 0 {
        return 0;
    }

    for _ in 1..n {
        res += prev;
        prev = res - prev;
    }

    res
}

fn fibo_functional(n: u32) -> u32 {
    fn fibo_f_body(n0: u32, n1: u32, n: u32) -> u32 {
        if n == 0 {
            return n0;
        }
        fibo_f_body(n1, n0 + n1, n - 1)
    }
    fibo_f_body(0, 1, n)
}

fn fibo_test() {
    for i in 0..10 {
        println!("i : {}, f : {}", fibo_imperative(i), fibo_functional(i));
    }
}

fn facto_i(n: u32) -> u32 {
    let mut res = 1;

    for n in 1..=n {
        res *= n;
    }

    res
}

fn facto_f(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    n * facto_f(n - 1)
}

fn facto_test() {
    for i in 0..10 {
        println!("i : {}, f : {}", facto_i(i), facto_f(i));
    }
}
