#[derive(Debug)]
enum GradeLevel {
    Bachelor,
    Master,
    PhD,
}

#[derive(Debug)]
enum Major {
    ComputerScience,
    ElectricalEngineering,
}

#[derive(Debug)]
struct Student {
    name: String,
    grade: GradeLevel,
    major: Major,
}

impl Student {
    fn new(name: String, grade: GradeLevel, major: Major) -> Self {
        Student { name, grade, major }
    }

    fn introduce_yourself(&self) {
        let grade_msg = match &self.grade {
            GradeLevel::Bachelor => "Bachelor's degree",
            GradeLevel::Master => "Master's degree",
            GradeLevel::PhD => "PhD",
        };

        let major_msg = match &self.major {
            Major::ComputerScience => "Computer Science",
            Major::ElectricalEngineering => "Electrical Engineering",
        };

        println!("Hello, my name is {}. I am pursuing a {} in {}.", self.name, grade_msg, major_msg);
    }
}

fn main() {
    let s1 = Student::new(
        "Jesse".to_string(),
        GradeLevel::Bachelor,
        Major::ComputerScience,
    );
    s1.introduce_yourself();
}
