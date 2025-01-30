use docx_rust::document::Document;
use docx_rust::DocxFile;
use std::fs::File;
use std::io::Read;

pub struct QuestionData {
    pub qn: String,
    pub question: String,
    pub options: Vec<String>,
    pub answer: String,
    pub mark: String,
    pub unit: String,
    pub lo: String,
    pub mix_choices: String,
    pub creator_reviewer: String,
    pub editor: String,
    pub reference: String,
}

pub fn extract_questions_from_docx(file_path: &str) -> Result<Vec<QuestionData>, Box<dyn std::error::Error>> {
    // Đọc và parse document
    let doc_file = DocxFile::from_file(file_path)?;
    let doc = doc_file.parse()?;
    
    let mut questions = Vec::new();
    
    // Lặp qua các bảng trong document
    for table in doc.document.body.tables() {
        let mut current_question = QuestionData {
            qn: String::new(),
            question: String::new(),
            options: Vec::new(),
            answer: String::new(),
            mark: String::new(),
            unit: String::new(),
            lo: String::new(),
            mix_choices: String::new(),
            creator_reviewer: String::new(),
            editor: String::new(),
            reference: String::new(),
        };

        // Lặp qua các hàng trong bảng
        for (row_idx, row) in table.rows().iter().enumerate() {
            if row.cells().len() < 2 {
                continue;
            }

            let key = row.cells()[0].text().trim().to_string();
            let value = row.cells()[1].text().trim().to_string();

            match key.as_str() {
                "QN=" => current_question.qn = value,
                "Which of the following best describes an array?" => current_question.question = value,
                "a." | "b." | "c." | "d." => {
                    if !value.is_empty() {
                        current_question.options.push(value);
                    }
                }
                "ANSWER:" => current_question.answer = value,
                "MARK:" => current_question.mark = value,
                "UNIT:" => current_question.unit = value,
                "LO:" => current_question.lo = value,
                "MIX CHOICES:" => current_question.mix_choices = value,
                "CREATOR-REVIEWER:" => current_question.creator_reviewer = value,
                "EDITOR:" => current_question.editor = value,
                "REFERENCE:" => current_question.reference = value,
                _ => {}
            }
        }

        questions.push(current_question);
    }

    Ok(questions)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "C:/Users/Admin/Downloads/test2.docx"; // Thay đổi đường dẫn file của bạn
    
    match extract_questions_from_docx(file_path) {
        Ok(questions) => {
            for (i, q) in questions.iter().enumerate() {
                println!("\n=== Câu hỏi {} ===", i + 1);
                println!("QN: {}", q.qn);
                println!("Câu hỏi: {}", q.question);
                println!("Các lựa chọn:");
                for (j, option) in q.options.iter().enumerate() {
                    println!("  {}. {}", (b'a' + j as u8) as char, option);
                }
                println!("Đáp án: {}", q.answer);
                println!("Điểm: {}", q.mark);
                println!("Unit: {}", q.unit);
                println!("LO: {}", q.lo);
                println!("Mix choices: {}", q.mix_choices);
                println!("Creator-Reviewer: {}", q.creator_reviewer);
                println!("Editor: {}", q.editor);
                println!("Reference: {}", q.reference);
            }
        }
        Err(e) => println!("Lỗi khi đọc file: {}", e),
    }
    
    Ok(())
}

