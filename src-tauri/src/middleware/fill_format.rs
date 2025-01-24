use docx_rust::DocxFile;
use docx_rust::document::{BodyContent, ParagraphContent, RunContent, TableRowContent, TableCellContent};
use std::path::Path;

pub fn read_docx_content(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(file_path).exists() {
        return Err("File không tồn tại!".into());
    }

    let docx = DocxFile::from_file(file_path)?;
    let docx = docx.parse()?;

    for element in &docx.document.body.content {
        if let BodyContent::Table(table) = element {
            let mut question_id = String::new();
            let mut question_text = String::new();
            let mut answers = Vec::new();
            let mut correct_answer = String::new();
            let mut is_next_answer = false; 

            if let Some(first_row) = table.rows.first() {
                for (cell_idx, cell) in first_row.cells.iter().enumerate() {
                    let cell_text = match cell {
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
                    };

                    match cell_idx {
                        0 => {
                            if let Some(id) = cell_text.trim().strip_prefix("QN=") {
                                question_id = id.trim().to_string();
                            }
                        },
                        1 => question_text = cell_text.trim().to_string(),
                        _ => {}
                    }
                }
            }

            if question_id.is_empty() {
                return Err("File sai format: Thiếu ID câu hỏi (QN=)".into());
            }
            if question_text.is_empty() {
                return Err(format!("File sai format: Câu hỏi {} thiếu nội dung", question_id).into());
            }

            for row in table.rows.iter().skip(1) {
                let mut row_text = Vec::new();
                
                for cell in row.cells.iter() {
                    let cell_text = match cell {
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
                    };
                    
                    let cell_text = cell_text.trim();
                    if !cell_text.is_empty() {
                        if cell_text == "ANSWER:" {
                            is_next_answer = true;
                        } else if is_next_answer {
                            correct_answer = cell_text.to_string();
                            is_next_answer = false;
                        } else {
                            row_text.push(cell_text.to_string());
                        }
                    }
                }

                if let Some(first) = row_text.first() {
                    if first.starts_with(|c: char| c.is_ascii_lowercase() && c.is_alphabetic()) 
                       && first.len() <= 2 
                    {
                        if let Some(second) = row_text.get(1) {
                            answers.push(format!("{} {}", first, second));
                        }
                    }
                }
            }

            println!("Câu hỏi {}: {}", question_id, question_text);
            println!("Các đáp án:");
            for answer in &answers {
                println!("- {}", answer);
            }
            println!("Đáp án đúng: {}", correct_answer);
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
