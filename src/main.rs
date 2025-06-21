use std::io::{self, Write};
use std::fs;
use genpdf::{elements, fonts::{FontData, FontFamily}, style, Element};

struct Student {
    name: String,
    total_marks: f32,
    num_subjects: u32,
}

fn calculate_average(total_marks: f32, num_subjects: u32) -> f32 {
    if num_subjects == 0 {
        0.0
    } else {
        total_marks / num_subjects as f32
    }
}

fn assign_grade(average: f32) -> char {
    if average >= 90.0 {
        'A'
    } else if average >= 75.0 {
        'B'
    } else if average >= 60.0 {
        'C'
    } else {
        'D'
    }
}

fn generate_pdf_report(student: &Student, average: f32, grade: char) -> Result<(), Box<dyn std::error::Error>> {
    let font_data = fs::read("./fonts/Arial.ttf")?;
    
    // Create a font family with the same font for all styles
    let font_family = FontFamily {
        regular: FontData::new(font_data.clone(), None)?,
        bold: FontData::new(font_data.clone(), None)?,
        italic: FontData::new(font_data.clone(), None)?,
        bold_italic: FontData::new(font_data, None)?,
    };

    let mut doc = genpdf::Document::new(font_family);
    doc.set_title("Student Report Card");

    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    doc.push(elements::Paragraph::new("Student Report Card")
        .styled(style::Style::new().bold().with_font_size(24)));
    doc.push(elements::Break::new(1));

    let mut table = elements::TableLayout::new(vec![1, 2]);
    table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));

    let mut row = table.row();
    row.push_element(elements::Paragraph::new("Name:"));
    row.push_element(elements::Paragraph::new(student.name.clone()));
    row.push().expect("Invalid table row");

    let mut row = table.row();
    row.push_element(elements::Paragraph::new("Total Marks:"));
    row.push_element(elements::Paragraph::new(student.total_marks.to_string()));
    row.push().expect("Invalid table row");

    let mut row = table.row();
    row.push_element(elements::Paragraph::new("Number of Subjects:"));
    row.push_element(elements::Paragraph::new(student.num_subjects.to_string()));
    row.push().expect("Invalid table row");
    
    let mut row = table.row();
    row.push_element(elements::Paragraph::new("Average:"));
    row.push_element(elements::Paragraph::new(format!("{:.2}", average)));
    row.push().expect("Invalid table row");

    let mut row = table.row();
    row.push_element(elements::Paragraph::new("Grade:"));
    row.push_element(elements::Paragraph::new(grade.to_string())
        .styled(style::Style::new().bold()));
    row.push().expect("Invalid table row");

    doc.push(table);

    doc.render_to_file("report_card.pdf")?;

    Ok(())
}


fn main() {
    println!("-- Student Grade Reporter --");

    let mut name = String::new();
    let mut total_marks_str = String::new();
    let mut num_subjects_str = String::new();

    print!("Enter student's name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).expect("Failed to read name");
    
    print!("Enter total marks: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut total_marks_str).expect("Failed to read total marks");

    print!("Enter number of subjects: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut num_subjects_str).expect("Failed to read number of subjects");

    let student = Student {
        name: name.trim().to_string(),
        total_marks: total_marks_str.trim().parse().expect("Please type a number for total marks!"),
        num_subjects: num_subjects_str.trim().parse().expect("Please type a number for subjects!"),
    };

    let average = calculate_average(student.total_marks, student.num_subjects);
    let grade = assign_grade(average);

    println!("\nGenerating PDF report card...");

    match generate_pdf_report(&student, average, grade) {
        Ok(_) => println!("Successfully generated report_card.pdf"),
        Err(e) => eprintln!("Error generating PDF: {}", e),
    }
}
