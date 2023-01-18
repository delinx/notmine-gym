pub mod structs;

fn main()
{
    let repetitions = 20;
    let weight = 10;
    let tmp = structs::Reps::new(repetitions, weight);
    println!("Hello, world! We got a Rep {:?}", tmp);
}
