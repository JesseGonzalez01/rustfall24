#[derive(Debug)]
enum GradeLevel {
    Bachelor,
    Master,
    PhD,
}

enum Major {
    ComputerScience,
    ElectricalEngineering,
}

struct Student {
    name:String,
    grade:GradeLevel,
    major:Major,
}

impl Student {
    fn new(name:String,grade:GradeLevel,major:Major) -> Self {
        Student (
            name:name,
            grade:grade,
            major:major,
        )
    }
    let Self: Student = Student::GradeLevel
}

fn introduce_yourself(&self){
    let grade_msg = match grade {
        grade::Bachelor => "my grade is Bachelor",
        grade::Master => "my grade is Master",
        grade::PhD => "my grade is PhD",
    }
    let name_msg = match name {
        name::String => "Hello my name is" + name,
    }
    let major_msg = match major {
        major::ComputerScience => "my major is Computer Science",
        major::ElectricalEngineering => "my major is Electrical Engineering",
    }
    
}

fn main() {
    let s1: Student = Student::new(name:"John".to_string(),
    grade:Bachelor, 
    major:ComputerScience);

}