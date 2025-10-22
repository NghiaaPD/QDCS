use docx_rust::{DocxFile, document::{BodyContent, TableCell, ParagraphContent, RunContent, TableRowContent, Paragraph, Run, Text}};
use std::collections::HashSet;

pub fn filter_docx_questions(
    input_path: &str,
    output_path: &str,
    keep_qn_ids: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let keep_set: HashSet<String> = keep_qn_ids.iter().cloned().collect();

    let doc_file = DocxFile::from_file(input_path)?;
    let docx = doc_file.parse()?;

    let mut new_body: Vec<BodyContent> = Vec::new();
    let mut found_questions: Vec<String> = Vec::new();
    let mut all_questions: Vec<String> = Vec::new();

    for element in &docx.document.body.content {
        if let BodyContent::Table(table) = element {
            // Kiểm tra hàng đầu tiên của bảng để lấy QN
            if let Some(first_row) = table.rows.first() {
                if let Some(TableRowContent::TableCell(first_cell)) = first_row.cells.get(0) {
                    let qn_text = extract_cell_text(first_cell).trim().to_string();
                    if let Some(id) = qn_text.strip_prefix("QN=") {
                        let id = id.trim().to_string();
                        all_questions.push(id.clone());
                        
                        if keep_set.contains(&id) {
                            found_questions.push(id.clone());
                            new_body.push(element.clone());
                        }
                        continue;
                    }
                }
            }
            // Nếu không phải bảng câu hỏi (không có QN=), giữ lại nguyên (ví dụ: tiêu đề, hướng dẫn)
            new_body.push(element.clone());
        } else {
            // Giữ lại các phần không phải bảng (ví dụ: đoạn văn bản ngoài bảng)
            new_body.push(element.clone());
        }
    }

    // Tính toán câu không trùng
    let not_found_questions: Vec<String> = all_questions
        .iter()
        .filter(|q| !keep_set.contains(*q))
        .cloned()
        .collect();

    // Tạo 2 dòng thông báo
    let duplicate_message = format!(
        "Số lượng câu trùng: {} - Câu hỏi: [{}]",
        found_questions.len(),
        found_questions.join(", ")
    );
    
    let non_duplicate_message = format!(
        "Số lượng câu không trùng: {} - Câu hỏi: [{}]",
        not_found_questions.len(),
        not_found_questions.join(", ")
    );

    // Tạo các đoạn văn cho thông báo
    let duplicate_paragraph = create_text_paragraph(&duplicate_message);
    let non_duplicate_paragraph = create_text_paragraph(&non_duplicate_message);

    // Chèn các thông báo vào đầu document
    let mut final_body = vec![
        BodyContent::Paragraph(duplicate_paragraph),
        BodyContent::Paragraph(non_duplicate_paragraph),
    ];
    final_body.extend(new_body);

    // Tạo Docx mới với body đã lọc
    let mut new_docx = docx.clone();
    new_docx.document.body.content = final_body;

    // Ghi ra file mới (đúng hàm là write_file)
    new_docx.write_file(output_path)?;

    Ok(())
}

// Đọc text từ TableCell
fn extract_cell_text(cell: &TableCell) -> String {
    let mut acc = String::new();
    for cell_content in &cell.content {
        let docx_rust::document::TableCellContent::Paragraph(p) = cell_content;
        for para_content in &p.content {
            if let ParagraphContent::Run(run) = para_content {
                for run_content in &run.content {
                    if let RunContent::Text(t) = run_content {
                        acc.push_str(&t.text);
                    }
                }
            }
        }
    }
    acc
}

// Tạo một đoạn văn text đơn giản
fn create_text_paragraph(text: &str) -> Paragraph {
    let text_content = Text {
        text: text.to_string().into(),
        ..Default::default()
    };
    
    let run = Run {
        content: vec![RunContent::Text(text_content)],
        ..Default::default()
    };
    
    Paragraph {
        content: vec![ParagraphContent::Run(run)],
        ..Default::default()
    }
}

