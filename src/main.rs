use std::io::{stdin,stdout,Write};
use std::fs::{File,create_dir_all};//Needed for file operations and creating directories
use std::path::PathBuf;//Used to handle file paths correctly
use printpdf::*;//The library for creating the PDF

//This function handles all the PDF creation logic.
fn generate_pdf_report(save_path:&PathBuf,name:&str,marks_list:&Vec<i32>,total:i32,average:f32,grade:&str){
    //Create a new PDF document, A4 size, with a title.
    let (doc,page1,layer1)=PdfDocument::new("Student Report Card",Mm(210.0),Mm(297.0),"Layer 1");
    let current_layer=doc.get_page(page1).get_layer(layer1);

    //Load a basic font like Helvetica.
    let font=doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
    let font_bold=doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();

    //--- Start writing text to the PDF ---
    current_layer.use_text("Student Report Card",24.0,Mm(75.0),Mm(250.0),&font_bold);//Title
    current_layer.use_text(format!("Student Name: {}",name),14.0,Mm(20.0),Mm(230.0),&font);

    //--- Add individual marks ---
    current_layer.use_text("Individual Marks:",14.0,Mm(20.0),Mm(215.0),&font_bold);
    let mut y_position=205.0;
    for(i,mark) in marks_list.iter().enumerate(){
        current_layer.use_text(format!("Subject {}: {}",i+1,mark),12.0,Mm(25.0),Mm(y_position),&font);
        y_position-=10.0;
    }

    //--- Add summary at the bottom ---
    current_layer.use_text(format!("Total Marks Obtained: {}",total),14.0,Mm(20.0),Mm(y_position-5.0),&font);
    current_layer.use_text(format!("Average: {:.2}%",average),14.0,Mm(20.0),Mm(y_position-15.0),&font);
    current_layer.use_text(format!("Final Grade: {}",grade),20.0,Mm(20.0),Mm(y_position-30.0),&font_bold);

    //Save the PDF to the hardcoded file path.
    let file=File::create(save_path).unwrap();
    doc.save(&mut std::io::BufWriter::new(file)).unwrap();
}


fn main(){
    let mut name=String::new();
    print!("Enter your name: ");
    let _=stdout().flush();
    stdin().read_line(&mut name).unwrap();

    let mut subjects_input=String::new();
    print!("Enter how many subjects you have: ");
    let _=stdout().flush();
    stdin().read_line(&mut subjects_input).unwrap();
    let subject_count:i32=subjects_input.trim().parse().unwrap();

    let mut marks_list=Vec::new();
    for i in 0..subject_count{
        let mut mark_input=String::new();
        print!("Enter marks for subject {}: ",i+1);
        let _=stdout().flush();
        stdin().read_line(&mut mark_input).unwrap();
        let mark:i32=mark_input.trim().parse().unwrap();
        marks_list.push(mark);
    }

    let total:i32=marks_list.iter().sum();
    let average=total as f32/subject_count as f32;

    let grade=if average>=90.0{
        "A"
    }else if average>=75.0{
        "B"
    }else if average>=60.0{
        "C"
    }else{
        "D"
    };

    //--- Define the hardcoded save location ---
    let output_dir="reports";
    create_dir_all(output_dir).unwrap();//Ensure the 'reports' directory exists
    let mut save_path=PathBuf::from(output_dir);
    save_path.push("report_card.pdf");

    //--- Call the function to generate the PDF ---
    generate_pdf_report(&save_path,name.trim(),&marks_list,total,average,grade);

    println!("\nSuccess! Report card saved to: {}",save_path.display());
}