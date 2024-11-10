/*Trying to understand the Rust control flow better by building simple project with them.
I will first build a project that will check the grades of the each student whether they passed or fail.
Then i will be using thye various control flow types to perfofm various things like; Loop over some students grade and perform some function using the loop, while loop,for loop.
We will also incoporates the break and continue while wokring on this simple grading system.
We will also using the matching type system to even do more with this simple grading system.
*/

//creating students variables to be computed
fn main() {
    let _student_score:Vec<f32>= vec![100.0, 60.0, 45.6, 34.3, 15.8, 57.1, 40.2, 13.6, 12.4, 17.3];
    //Using if control and for loop statement to check if the students fail or passed by printing ou fail if below 40 and pass if above 40.

    for (index, &score) in _student_score.iter().enumerate() {
        if score > 40.0 {
            println!("Student {} passed with score: {}", index + 1, score);
        } else {
            println!("Student {} failed with score: {}", index + 1, score);
        }
    } 
}