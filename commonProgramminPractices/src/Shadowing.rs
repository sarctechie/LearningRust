//Declaring a new variable with the same name as a previous variable is known as shadowing. 
//It means that the second variable is what the compiler sees when the variable is used. 
//The second variable overshadows the first, taking anu uses of the variable name to itself until either it itself gets shadowed or the scope ends. 


fn main(){
    let x =5;
    let x = x+1;

    {
        let x = x*2;
        println!("The value of x in inner scope is: {x}");

    }

    println!("The value of x in outer scope is: {x}");
}

//The ouput would be:
//The value of x in the inner scope is: 12
//The value of x is: 6

//difference between mut and shadowing is that we can change the type of the value but reuse the same name
 let spaces = " ";
 let spaces = spaces.len();

 //first spaces is string type
 //second spaces is NUMBER TYPE