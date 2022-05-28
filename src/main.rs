pub mod functions;
pub mod structs;

fn main() {
    let samples = functions::create_samples();

    let question: i8 = functions::get_question().expect(
        "The first argument of the program has to be an i8 value."
    );

    let found_b = functions::find_sample_b_by_question(
        question,
        &samples
    );


    if found_b.is_some() {
        println!("The sample was found. It's index is {}", found_b.unwrap());
        return;
    }

    println!("Could not find the sample with blueprint '{}'", question);
}
