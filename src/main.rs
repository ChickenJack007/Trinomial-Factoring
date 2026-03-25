use input_loop::input_loop;

fn main() {
    println!("ax^2 + bx + c\n");
    let mut equation: [i32; 3] = [1; 3];
    equation[0] = input_loop("Please input a"); 
    equation[1] = input_loop("Please input b"); 
    equation[2] = input_loop("Please input c"); 
    println!("\n{}x^2 + {}x + {}", equation[0], equation[1], equation[2]);

    let gcf: i32 = get_gcf(equation);
    println!("\nGCF: {gcf}");
    for i in 0..3 {
        equation[i] /= gcf;
    }

    println!("\n{}({}x^2 + {}x + {})", gcf, equation[0], equation[1], equation[2]);
    let (afact1, afact2): (Vec<i32>, Vec<i32>) = factor(equation[0]);
    for i in 0..afact1.len() {
        println!("{} x {}", afact1[i], afact2[i]);
    }
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

    return (fact1, fact2);
}

fn get_gcf(num: [i32; 3]) -> i32 {
    let mut gcf: i32 = 1;

    let mut large_num: i32 = 1;
    for number in num {
        if large_num < number {
            large_num = number;
        }
    }

    for n in 2..large_num {
        if (num[0] % n == 0) && (num[1] % n == 0) && (num[2] % n == 0) {
            gcf = n;
        }
    }
    return gcf;
}
