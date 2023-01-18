use serde::{Deserialize, Serialize};

pub struct Workout
{
    pub workout: Vec<Exercise>,
    pub date: u8,
}

impl Workout
{
    pub fn new(_exercises: Vec<Exercise>, _date: u8) -> Self
    {
        return Workout {
            workout: _exercises,
            date: _date,
        };
    }
}

// A set is a particular exercise performed a number of repetitions at a particular weight.
// For example, a set of benchpress, perfomred 5 repetitions at weight 100 kilo
// exercise: benchpress
// reps: 100 kilo, 5 repetitions)
#[derive(Serialize)]
pub struct Set
{
    pub target_reps: Reps,
    // todo!(pub accomplished_reps: u8,)
}

// A struct to represent repetitions of a particular exercise.
#[derive(Serialize, Debug)]
pub struct Reps
{
    pub repetitions: u8,
    pub weight: u8,
}

impl Reps
{
    pub fn new(_repetitions: u8, _weight: u8) -> Self
    {
        return Reps {
            repetitions: _repetitions,
            weight: _weight,
        };
    }
}

#[derive(Serialize)]
pub struct Exercise
{
    pub name: BarbellExercise,
    pub sets: Vec<Set>,
}

impl Exercise
{
    // "Benchpress 3 sets of 5 reps at 100 lb"
    pub fn new(_name: BarbellExercise, _sets: u8, _reps: u8, _weight: u8) -> Self
    {
        let the_sets: Vec<Set> = Vec::with_capacity(_sets as usize);
        return Exercise {
            name: _name,
            sets: the_sets,
        };
    }

    pub fn print_exercise(&self)
    {
        // Serialize it to a JSON string.
        let exer: &str = &serde_json::to_string(&self).unwrap();
        println!("{}", exer);
    }
}

#[derive(Serialize)]
pub enum BarbellExercise
{
    Benchpress,
    Clean,
    Deadlift,
    Press,
    Squat,
}


// TESTS ///////////////

/// testing new constructor for Reps structure
#[test]
fn reps_new()
{
    let repetitions = 20;
    let weight = 10;
    let tmp = Reps::new(repetitions, weight);
    assert_eq!(repetitions, tmp.repetitions);
    assert_eq!(weight, tmp.weight);
}
