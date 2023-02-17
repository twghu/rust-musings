#[allow(unused_variables)]
fn main() {
    let mut lights: [bool; 100] = [false; 100];

    for p in 0..lights.len() {
        // for s in lights.iter_mut().step_by(p+1) {
        for (i,s) in lights[p..].iter_mut().enumerate().step_by(p+1) {
            *s = ! *s;
            // println!("p is {p} and setting index {} to {}", i+p, *s);
        }
    }

    // slices and map call would be neater here.
    for i in 0..lights.len() {
        print!("{}", if lights[i] { '*' } else { ' ' });
        if (i+1) % 10 == 0  {
            println!("");
        }
    }

    println!("64th is {}", lights[63]);
}

