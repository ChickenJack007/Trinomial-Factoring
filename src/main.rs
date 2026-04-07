use input_loop::input_loop;

fn main() {
    let mut equation: [i32; 3] = [1; 3];
    let mut running: bool = true;
    while running{
        println!("ax^2 + bx + c\n");
        equation[0] = input_loop("Please input a"); 
        equation[1] = input_loop("Please input b"); 
        equation[2] = input_loop("Please input c"); 
        println!("\n{}x^2 + {}x + {}", equation[0], equation[1], equation[2]);
        if input_loop::<char>("Is this correct? [y/n]") == 'y'{
            running = false;
        }
    }

    let gcf: i32 = get_gcf(equation);
    println!("\nGCF: {gcf}");
    for i in 0..3 {
        equation[i] /= gcf;
    }

    println!("\n{}({}x^2 + {}x + {})", gcf, equation[0], equation[1], equation[2]);
    let (afact1, afact2): (Vec<i32>, Vec<i32>) = factor(equation[0]);
    let (cfact1, cfact2): (Vec<i32>, Vec<i32>) = factor(equation[2]);
    let afacts: [Vec<i32>; 2] = [afact1, afact2];
    let cfacts: [Vec<i32>; 2] = [cfact1, cfact2];
    solve(afacts, cfacts, equation[1]);
}

fn solve(afacts: [Vec<i32>; 2], cfacts: [Vec<i32>; 2], b:i32) {
    for a in 0..afacts[1].len() {
        for n in 0..cfacts[0].len() {
            let equation = (afacts[0][a] * cfacts[1][n]) + (afacts[1][a] * cfacts[0][n]);
            //println!("B = {equation}");
            if equation == b.abs() {
                println!("({}x + {})({}x + {})", afacts[0][a], cfacts[0][n], afacts[1][a], cfacts[1][n]);
                break;
            }
        }
    }
}

fn factor(num: i32) -> (Vec<i32>, Vec<i32>) {
    let mut fact1 = Vec::new(); 
    let mut fact2 = Vec::new(); 

    for n in 1..num.abs() + 1 {
        if num % n == 0 {
            fact1.push(n);
            fact2.push(num / n);
        }
    }
    return (fact1, fact2);
}

fn get_gcf(num: [i32; 3]) -> i32 {
    let mut gcf: i32 = 1;
    let mut large_num: i32 = 1;
    for number in num {
        if large_num < number.abs() {
            large_num = number.abs();
        }
    }
    for n in 2..large_num {
        if (num[0] % n == 0) && (num[1] % n == 0) && (num[2] % n == 0) {
            gcf = n;
        }
    }
    return gcf;
}
