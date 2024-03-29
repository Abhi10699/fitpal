use std::io::{self, LineWriter, Write, Read};
use std::fs::{File, self};
use chrono::prelude::*;

#[derive(Debug)]
struct Date {
    day_of_week: u32,
    day_of_month: u32,
    month_of_year: u32,
    year: i32
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

    fn now() -> Self {
        let date_now : DateTime<Local> = Local::now();
        Date {
            day_of_month:  date_now.day(),
            day_of_week: date_now.weekday().number_from_sunday(),
            month_of_year: date_now.month(),
            year: date_now.year()
        }
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
            date: Date::now()
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

    fn load_workout(file_path: &str) -> Self{

        let mut file = File::open(file_path).unwrap();
        let mut file_string = String::new();
        let _ = file
            .read_to_string(&mut file_string)
            .expect("Error Reading the file");

        let file_string = file_string.trim();
        let mut string_arr = file_string.split("\n\n").into_iter();

        let workout_name: Vec<&str> = string_arr.next().unwrap().split(":").collect();
        let mut workout = Workout::new(workout_name[1].trim().to_string());
        for chunk in string_arr {
            let exercises: Vec<&str> = chunk.split("\n").collect();            

            let exercise_name: Vec<&str> = exercises[0].split(":").collect();
            let exercise_sets: Vec<&str> = exercises[1].split(":").collect();
            let exercise_repetition: Vec<&str> = exercises[2].split(":").collect();
            let exercise_rest: Vec<&str> = exercises[3].split(":").collect();

            let exercise = Exercise {
               name: String::from(exercise_name[1].trim()),
               sets: exercise_sets[1].trim().parse().expect("Error Parsing Sets"),
               repetition: exercise_repetition[1].trim().parse().expect("Error Parsing Reps"),
               rest_between_sets:  exercise_rest[1].trim().parse().expect("Error parsing rest")
            };
            workout.add_exercise(exercise);
        }

        println!("{:#?}", workout);
        return workout;
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


fn read_workouts(workout_dir: &str) -> Vec<Workout> {
    let dir_contents = fs::read_dir(workout_dir).expect("Invalid directory").into_iter();
    let mut workouts:Vec<Workout> = Vec::new();
    for item in dir_contents  {
        let path = item.unwrap();
        let workout = Workout::load_workout(path.path().to_str().unwrap());
        workouts.push(workout);
    }

    return workouts;
}

fn app_menu(){
    // TODO: Implement application menu selection
}


fn main() {
    println!("Fitness Pal");
    //let mut workout = Workout::new("Back".to_string());

    
    //println!("{:#?}", workout);

    //workout.save_workout();

    let workouts = read_workouts("./workouts");
    
    //for _ in 1..5 {
    //    let exercice = Exercise::new();
    //    workout.add_exercise(exercice)
    //}

    
    println!("{:#?}", workouts);

}
