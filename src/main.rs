use input_loop::input_loop;

fn main() {
    //let mut vec = Vec::new();
    println!("ax^2 + bx + c\n");
    let mut a: i32 = input_loop("Please input a");
    let mut b: i32 = input_loop("Please input b");
    let mut c: i32 = input_loop("Please input c");
    println!("{a}x^2 + {b}x + {c}");
    let gcf: i32 = get_gcf(a, b, c);
    println!("\nGCF: {gcf}");
    a /= gcf;
    b /= gcf;
    c /= gcf;
    println!("\n{gcf}({a}x^2 + {b}x + {c})");
    //factor(a);
}

fn get_gcf(a:i32, b:i32, c:i32) -> i32 {
    let mut gcf: i32 = 1;
    let mut d = 20;
    if a > b && a > c {
        d = a;
    }
    else if b > a && b > c {
        d = b;
    }
    else {
        d = c;
    }

    for n in 2..d {
        if (a % n == 0) && (b % n == 0) && (c % n == 0) {
            gcf = n;
        }
    }
    return gcf;
}

fn factor(n: i32){// -> Vec<i32> {
    let mut factors: Vec<i32> = [1i32, n]; 
    println!("{:?}", factors);

}

fn prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false; 
        }
    }
    true 
}
