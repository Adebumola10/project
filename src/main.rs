/*Trying to understand the Rust control flow better by building simple project with them.
I will first build a project that will check the grades of the each student whether they passed or fail.
Then i will be using thye various control flow types to perfofm various things like; Loop over some students grade and perform some function using the loop, while loop,for loop.
We will also incoporates the break and continue while wokring on this simple grading system.
We will also using the matching type system to even do more with this simple grading system.
*/

//creating students variables to be computed
fn main() {
    let mut _score = 100;
    let _first_student: i32 = 100;
    let _second_student: i32 = 90;
    let _third_student: i32 = 80;
    let _fourth_student: i32 = 70;
    let _fifth_student: i32 = 60;
    let _sixth_student: f32 = 50.63;
    let _seventh_student: f32 = 40.88;
    let _eight_student: f32 = 30.64;
    let _nineth_student: f32 = 20.78;
    let _tenth_student: f32 = 15.56;

    //Using if control statement to check if the students fail or passed by printing ou fail if below 40 and pass if above 40.

    if _first_student >= _score {
        println!("First_Student passed!");
    }
}