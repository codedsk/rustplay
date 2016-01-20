// lifetime elision

/*
   The following function declaration is illegal because the lifetime
   of the returned string borrow is implicitly defined as the same
   lifetime as the function:

   fn get_str() -> &str;

   When the function is destroyed, the string will be destroyed and
   its borrow cannot exist. It can be fixed by explicitly setting a
   lifetime to the output reference as shown below. It is important
   to note that 'a does not represent the lifetime of the function.
   It is a generic parameter passed through the function declaration
   so it can be used by borrows associated with the function.
 */

fn get_str<'a>() -> &'a str {
    let s: &'a str = "hello";
    s
}

/*
   We could also write the function definition as follows:

   fn get_str<'a>() -> &'a str {
       let s = "hello";
       &s
   }

   Is the lifetime of &s, at the end of the function, the same as the
   lifetime of the output &'a str? Is the lifetime
   of &s set by the compiler as it is compiling the code and realizes
   &s is the function output?
 */


fn main() {
    let s: &str;
    s = get_str();
    println!("{}",s);
}
