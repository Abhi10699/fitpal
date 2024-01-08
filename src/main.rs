use std::io::{self, LineWriter, Write};
use std::fs::File;
#[derive(Debug)]
struct Date {
    day_of_week: i8,
    day_of_month: i8,
    month_of_year: i8,
    year: i16
}

#[derive(Debug)]
struct Workout {
    name: String,
    exercise: Vec<Exercise>,
}

impl Workout {
    fn new(name: String) -> Self {
        Workout {
            name: name,
            exercise: Vec::new(),
        }
    }

    fn add_exercise(&mut self,session: Exercise) {
        self.exercise.push(session)
    }

    fn save_workout(&self){
        let file_name = format!("{workoutName}.txt",workoutName=self.name);
        let file = File::create(file_name).unwrap();
        let mut file = LineWriter::new(file);


        let _ = file.write(format!("Workout Name: {}\n", self.name).as_bytes());
    
        for exercise in &self.exercise {
           let sets_str = format!(
                "\tExercise: {exercise_name}\tSets: {num_sets}\n\tRepetitions: {num_reps}\n\tRest: {rest}\n\n", 
                exercise_name=exercise.name, 
                num_sets = exercise.sets,
                num_reps = exercise.repetition,
                rest = exercise.rest_between_sets
            );
           file.write(sets_str.as_bytes());
        }
        
    }

}

#[derive(Debug)]
struct Exercise {
    name: String,
    sets: i8,
    repetition: i8,
    rest_between_sets: i16
}
impl Exercise {
    fn new() -> Self {
        let mut name : String = String::new();
        let mut sets : String = String::new();
        let mut repetition: String = String::new();
        let mut rest_between_sets: String = String::new();

        println!("[+] Enter Your Exercise Name: ");
        io::stdin().read_line(&mut name).unwrap();

        println!("[+] Enter Your No. Of Sets: ");
        io::stdin().read_line(&mut sets).unwrap();

        println!("[+] Enter Your Repetitions: ");
        io::stdin().read_line(&mut repetition).unwrap();

        println!("[+] How much rest did you take in between of each reps: ");
        io::stdin().read_line(&mut rest_between_sets).unwrap();

        Exercise {
            name: name,
            rest_between_sets: rest_between_sets.trim().parse().expect("Input is not an integer"),
            sets: sets.trim().parse().expect("Input is not an integer"),
            repetition: repetition.trim().parse().expect("Repetition is not an integer")
        }
    }
}

fn main() {
    println!("Fitness Pal");
    let mut workout = Workout::new("Back".to_string());

    for i in 1..5 {
        let exercice = Exercise::new();
        workout.add_exercise(exercice)
    }

    println!("{:#?}", workout);

    workout.save_workout();

}
