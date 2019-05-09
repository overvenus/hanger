fn main() {
    entry();
}

fn hello() {
    print!("hello")
}

#[hanger::hook(before = hello, after = bye)]
fn world() {
    print!("world")
}

fn bye() {
    print!("\nbye\n")
}

#[hanger::hook(world)]
fn entry() {}
