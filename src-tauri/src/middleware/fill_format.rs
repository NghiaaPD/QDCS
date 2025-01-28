use docx_rust::DocxFile;
use docx_rust::document::{BodyContent, ParagraphContent, RunContent, TableRowContent, TableCellContent};
use std::path::Path;

fn extract_cell_text(cell: &TableRowContent) -> String {
    match cell {
        TableRowContent::TableCell(cell_data) => {
            cell_data.content.iter().fold(String::new(), |acc, content| {
                let TableCellContent::Paragraph(p) = content;
                acc + &p.content.iter().fold(String::new(), |acc, run| {
                    if let ParagraphContent::Run(r) = run {
                        acc + &r.content.iter().fold(String::new(), |acc, text| {
                            if let RunContent::Text(t) = text {
                                acc + &t.text
                            } else {
                                acc
                            }
                        })
                    } else {
                        acc
                    }
                })
            })
        },
        _ => String::new()
    }
}

#[derive(Default)]
struct Question {
    id: String,
    text: String,
    answers: Vec<String>,
    correct_answer: String,
}

pub fn read_docx_content(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(file_path).exists() {
        return Err("File không tồn tại!".into());
    }

    let doc_file = DocxFile::from_file(file_path)?;
    let docx = doc_file.parse()?;

    for element in &docx.document.body.content {
        if let BodyContent::Table(table) = element {
            let mut question = Question::default();
            let mut is_next_answer = false;
            let mut correct_answer_letter = String::new();

            // Xử lý hàng đầu tiên
            if let Some(first_row) = table.rows.first() {
                for (cell_idx, cell) in first_row.cells.iter().enumerate() {
                    let cell_text = extract_cell_text(cell).trim().to_string();
                    match cell_idx {
                        0 => if let Some(id) = cell_text.strip_prefix("QN=") {
                            question.id = id.trim().to_string();
                        },
                        1 => question.text = cell_text,
                        _ => {}
                    }
                }
            }

            if question.id.is_empty() {
                return Err("File sai format: Thiếu ID câu hỏi (QN=)".into());
            }
            if question.text.is_empty() {
                return Err(format!("File sai format: Câu hỏi {} thiếu nội dung", question.id).into());
            }

            for row in table.rows.iter().skip(1) {
                let mut row_text = Vec::new();
                
                for cell in &row.cells {
                    let cell_text = extract_cell_text(cell).trim().to_string();
                    if !cell_text.is_empty() {
                        match cell_text.as_str() {
                            "ANSWER:" => is_next_answer = true,
                            text if is_next_answer => {
                                correct_answer_letter = text.to_string();
                                is_next_answer = false;
                            },
                            _ => row_text.push(cell_text),
                        }
                    }
                }

                if let Some(first) = row_text.first() {
                    if first.starts_with(|c: char| c.is_ascii_lowercase() && c.is_alphabetic()) 
                       && first.len() <= 2 
                    {
                        if let Some(second) = row_text.get(1) {
                            let answer = format!("{} {}", first, second);
                            if first.trim().starts_with(&correct_answer_letter.trim().to_lowercase()) {
                                question.correct_answer = second.clone();
                            }
                            question.answers.push(answer);
                        }
                    }
                }
            }
            println!("Câu hỏi {}: {}", question.id, question.text);
            println!("Các đáp án:");
            for answer in &question.answers {
                println!("- {}", answer);
            }
            println!("Đáp án đúng: {}", question.correct_answer);
            println!("-------------------");
        }
    }
    
    Ok(())
}

// fn main() {
//     let file_path = "C:\\Users\\Admin\\Downloads\\test.docx"; 
//     match read_docx_content(file_path) {
//         Ok(_) => println!("Đã đọc file thành công"),
//         Err(e) => println!("Lỗi khi đọc file: {}", e),
//     }
// }