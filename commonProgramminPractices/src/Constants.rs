//Constants are values that are bound to a name and are not allowed to change
//Do not use mut with constants as they are by default immutable
// 'const' keyword should be used and the value must be annotatetd. 
//Do not use 'let' 
//Constants can be declared globally so they can be used by different parts of the code
//Constants can be set only to a constant expression, not the result of a value computed at runtime. 

const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;

//naming convention in Rust for constants is to use all uppercase with underscores between words. 
//constants are valid for the entirety of the program, within the scope
//Naming hardcoded values used throughout your program as constants is useful in conveying the meaning of that value to future maintainers of the code. It also helps to have only one place in your code you would need to change if the hardcoded value needed to be updated in the future.