use std::io::{self, LineWriter, Write, Read};
use std::fs::File;


#[derive(Debug)]
struct Date {
    day_of_week: i8,
    day_of_month: i8,
    month_of_year: i8,
    year: i16
}

impl Date {
    fn get_date_string(&self) -> String{
        format!(
            "{year}_{month}_{day}", 
            year=self.year, 
            month=self.month_of_year, 
            day=self.day_of_month
        )
    }
}

#[derive(Debug)]
struct Workout {
    name: String,
    exercise: Vec<Exercise>,
    date: Date
}

impl Workout {
    fn new(name: String) -> Self {
        Workout {
            name: name,
            exercise: Vec::new(),
            date: Date {
                day_of_month: 8,
                day_of_week: 1,
                month_of_year: 1,
                year: 2023
            }
        }
    }

    fn add_exercise(&mut self,session: Exercise) {
        self.exercise.push(session)
    }

    fn save_workout(&self){
        let file_name = format!(
            "{workout_date}.txt",
            workout_date=self.date.get_date_string()
        );

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
           let _ = file.write(sets_str.as_bytes());
        }
    }

    fn load_workout(file_path: &str){
        let mut file = File::open(file_path).unwrap();
        let mut file_string = String::new();
        let bytes_read = file
            .read_to_string(&mut file_string)
            .expect("Error Reading the file");
        println!("{:#?}", file_string);
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
    //let mut workout = Workout::new("Back".to_string());

    //for _ in 1..5 {
    //    let exercice = Exercise::new();
    //    workout.add_exercise(exercice)
    //}

    //println!("{:#?}", workout);

    //workout.save_workout();

    let _ = Workout::load_workout("./2023_1_8.txt");


}
