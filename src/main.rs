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
    println!("{a}");
    let (afact1, afact2): (Vec<i32>, Vec<i32>) = factor(a);
    println!("{:?}\n{:?}", afact1, afact2);
}


fn factor(num: i32) -> (Vec<i32>, Vec<i32>) {
    let mut fact1 = Vec::new(); 
    let mut fact2 = Vec::new(); 

    for n in 1..num + 1 {
        if num % n == 0 {
            if ! (fact1.contains(&n) || fact2.contains(&n)) {
                fact1.push(n);
                fact2.push(num / n);
            }
        }
    }

    for i in 0..fact1.len() {
        println!("{} x {}", fact1[i], fact2[i]);
    }
    return (fact1, fact2);
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
