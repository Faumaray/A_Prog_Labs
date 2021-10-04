use quick_xml::events::Event;
use std::env;
use quick_xml::{Reader, Writer};
use quick_xml::se::Serializer;
use quick_xml::de::{from_str};
use core::panic;
use std::fs::File;
use std::io::{Read, Write};
use rand::*;
use serde::{Serialize,Deserialize};
#[derive(Serialize, Deserialize)]
#[serde(rename = "group")]
struct Group
{
    student: Vec<Student>
}
impl Group
{
    fn rec(&mut self)
    {
        let mut new_list: Vec<Student> = Vec::new();
        for student in self.student.clone()
        {
            new_list.push(student.clone().recalculate());
        }
        self.student = new_list;
    }
}
#[derive(Serialize, Deserialize, Clone)]
struct Student
{
    firstname: String,
    lastname: String,
    groupnumber: String,
    subject: Vec<Subject>,
    average: Average
}
impl Student
{
    fn new(firstname: String, lastname: String, groupnumber: String, subject: Vec<Subject>) -> Student
    {
        let mut average = 0.0;
        for subject in &subject
        {
            average += subject.mark as f32;
        }
        average = average / (&subject).len() as f32;
        Student { firstname, lastname, groupnumber, subject, average: Average(average)  }
    }
    fn recalculate(&mut self)-> Student
    {
        let mut average = 0.0;
        for subject in &self.subject
        {
            average += subject.mark as f32;
        }
        average = average / (self.subject).len() as f32;
        self.average = Average(average);
        self.to_owned()
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Average(f32);
#[derive(Serialize, Deserialize, Clone)]
struct Subject
{
    title: String,
    mark: usize
}
fn help() {
    eprintln!("lab_1.exe file_name.xml type_of_interaction(1=generate;2=check)");
}
fn main()-> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => {
            let file_in = &args[1];
            match args[2].parse::<usize>().unwrap() {
                1 =>{
                    generate_and_write_xml(file_in.as_str())?;
                    Ok(())
                },
                2 =>{
                    read_check_change_xml(file_in.as_str())?;
                    Ok(())
                },
                _ => {
                    eprintln!("type_of_interaction(1=generate;2=check)");
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Wrong type of interaction",
                    ));
                }
            }
            
        },
        _ => {
            // show a help message
            eprintln!("Provided {} args", args.len() - 1);
            help();
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Wrond amount of args",
            ));
        }
    }
}
const DATA: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" ?>\n<!DOCTYPE group SYSTEM \"task1.dtd\">\n\n";
fn read_check_change_xml(file_name: &str)->std::io::Result<()>
{
    let mut file = File::open(file_name).unwrap();
    let mut content = String::new(); 
    file.read_to_string(&mut content)?;
    let mut for_des = String::new();
    let mut current = usize::MIN;
    for line in content.lines()
    {
        if current >= 3
        {
            for_des.push_str(line);
        }
        current +=1;
    }
    let mut xml: Group = from_str(for_des.as_str()).unwrap();
    xml.rec();
    let mut buffer = Vec::new();
    let writer = Writer::new_with_indent(&mut buffer, b' ', 4);
    let mut ser = Serializer::with_root(writer, Some("group"));
    xml.serialize(&mut ser).unwrap();
    let str = std::str::from_utf8(&buffer).unwrap();
    buffer = prettify_xml(str).as_bytes().to_vec();
    file = File::create(file_name).unwrap();
    file.write_all(DATA.as_bytes())?;
    file.write_all(&buffer)?;
    Ok(())
}
fn generate_and_write_xml(file_name: &str) -> std::io::Result<()> {
    let subject_list: [String;5] = ["Математика".to_string(),"Русский язык".to_string(),"Физика".to_string(),"Химия".to_string(),"Информатика".to_string() ];
    let firstname_list: [String;8] = ["Марк".to_string(),"Клим".to_string(),"Тимофей".to_string(),"Александр".to_string(),"Иван".to_string(),"Давид".to_string(),"Кирилл".to_string(), "Егор".to_string()];
    let lastname_list: [String;8] = ["Иванов".to_string(),"Дроздов".to_string(),"Новиков".to_string(),"Борисов".to_string(),"Фролов".to_string(),"Прохоров".to_string(),"Гаврилов".to_string(),"Борисов".to_string()];
    let groupname_list: [String;9] = ["РПИС-91".to_string(),"РПИС-92".to_string(),"МОИС-91".to_string(),"ИВТ-91".to_string(),"ИВТ-92".to_string(),"ИВТ-93".to_string(),"ИВТ-94".to_string(),"ИСТ-91".to_string(),"ИСТ-92".to_string()];
    let group_number = rand::thread_rng().gen_range(0..9) as usize;
    let mut buffer = Vec::new();
    let writer = Writer::new_with_indent(&mut buffer, b' ', 4);
    let mut ser = Serializer::with_root(writer, Some("group"));
    let mut students = Vec::new();
    for _i in 0..4
    {
        let mut sub = Vec::new();
        for j in 0..5
        {
            
            let mark = rand::thread_rng().gen_range(1..=5) as usize;
            let title = subject_list[j].clone();
            sub.push(Subject{
                title,
                mark,
            });
        }
        students.push(Student::new(firstname_list[rand::thread_rng().gen_range(0..8)].clone(),
        lastname_list[rand::thread_rng().gen_range(0..8)].clone(),
        groupname_list[group_number].clone(),
            sub
        ));
    }
    Group {
        student: students
    }.serialize(&mut ser).unwrap();
    let str = std::str::from_utf8(&buffer).unwrap();
    let mut file = File::create(file_name).unwrap();
    buffer = prettify_xml(str).as_bytes().to_vec();
    file.write_all(DATA.as_bytes())?;
    file.write_all(&buffer)?;
    Ok(())
}
fn prettify_xml(xml: &str) -> String {
    let mut buf = Vec::new();

    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut writer = Writer::new_with_indent(Vec::new(), b' ', 4);

    loop {
        let ev = reader.read_event(&mut buf);

        match ev {
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Ok(event) => writer.write_event(event),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        }
        .expect("Failed to parse XML");

        // If we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }

    let result = std::str::from_utf8(&*writer.into_inner())
        .expect("Failed to convert a slice of bytes to a string slice")
        .to_string();

    result
}