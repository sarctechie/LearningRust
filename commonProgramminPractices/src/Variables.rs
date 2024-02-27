//lesson plan for Common Programming Practices

//1. Variables and Mutability:
//Immutable: a value is immutable when a variable gets a value and it bound to it

fn main(){
    let x = 5;
    println!("The value of x is: {x}");
    let x = 6;
    println!("The value of x is: {x}");
}
//the above function wont complile because x is immutable
//Error: cannot assign twice to immutable variables 'x'

//mut keyword will make variable mutable, i.e, can change value later

fn main(){
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

//the above function will print The value of x is: 5 and The value of x is: 6
