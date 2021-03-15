use std::collections::HashMap;

fn main() {
    let students = vec!["Alice".to_string(), "Bob".to_string()];
    let grades = vec![90, 85];
    println!("The first student is: {}", &students[0]);
    let mut student_grades = students
        .into_iter()
        .zip(grades.into_iter())
        .collect::<HashMap<_, _>>();
    println!("The student grades are: {:#?}", student_grades);

    let carol_name = "Carol".to_string();
    let carols_grade = 95;
    student_grades.insert(carol_name, carols_grade);
    println!("The student grades are: {:#?}", student_grades);
    println!("The new student's grade is {}", carols_grade);
    // The following fails because String doesn't implement the Copy trait
    // println!("The new student's name is {}", carol_name);

    solve_vector();
}

fn solve_vector() {
    let v1 = vec![111, 25, 30];
    println!("The mean of {:?} is {:.2}", v1, mean(&v1));
    let mut v2 = vec![1, 3, 4, 4, 4, 2, 3];
    let v3 = vec![5, 6, 6, 8];
    let v4: Vec<u32> = Vec::new();
    let mut v5 = Vec::new();
    v5.push(3);
    println!(
        "The vectors are {:?}, {:?}, {:?} and {:?}",
        &v2, &v3, &v4, &v5,
    );
    v2.sort();
    println!(
        "The medians are {:?}, {:?}, {:?} and {:?}",
        median(&v2),
        median(&v3),
        median(&v4),
        median(&v5),
    );
    println!(
        "The modes are {:?}, {:?}, {:?} and {:?}",
        mode(&v2),
        mode(&v3),
        mode(&v4),
        mode(&v5),
    );
}

fn mean(vector: &Vec<u32>) -> f64 {
    let length = vector.len();
    let mut sum = 0;
    for number in vector {
        sum += number;
    }
    sum as f64 / length as f64
}

fn median(vector: &Vec<u32>) -> Option<f64> {
    let length = vector.len();
    if length == 0 {
        return None;
    }
    let middle = length / 2;
    let parity = length % 2;
    match parity {
        0 => Some((vector[middle] + vector[middle - 1]) as f64 / 2.),
        _ => Some(vector[middle] as f64),
    }
}

fn mode(vector: &Vec<u32>) -> Option<u32> {
    let mut counts = HashMap::new();
    for value in vector {
        let entry = counts.entry(*value).or_insert(0usize);
        *entry += 1;
    }
    let mut mode_value = None;
    let mut mode_key = None;
    for (key, value) in counts {
        if mode_value == None || value >= mode_value.unwrap() {
            mode_value = Some(value);
            mode_key = Some(key);
        }
    }
    mode_key
}
