use docx_rust::DocxFile;
use docx_rust::document::BodyContent;
use std::path::Path;

pub fn read_tables_from_word(file_path: &str) -> Result<Vec<&BodyContent>, Box<dyn std::error::Error>> {
    // Kiểm tra file tồn tại
    if !Path::new(file_path).exists() {
        return Err("File không tồn tại!".into());
    }

    // Đọc và parse file docx
    let doc_file = DocxFile::from_file(file_path)?;
    let docx = doc_file.parse()?;
    
    // Lọc và lấy ra các phần tử là bảng
    let tables: Vec<&BodyContent> = docx.document.body.content
        .iter()
        .filter(|content| matches!(content, BodyContent::Table(_)))
        .collect();
    
    Ok(tables)
}

fn main() {
    // Đường dẫn tới file docx của bạn
    let file_path = "test.docx";  // Thay đổi đường dẫn này theo file của bạn
    
    match read_tables_from_word(file_path) {
        Ok(tables) => {
            println!("Đã tìm thấy {} bảng trong file", tables.len());
            
            // In thông tin chi tiết về từng bảng
            for (index, table) in tables.iter().enumerate() {
                println!("Bảng #{}", index + 1);
                if let BodyContent::Table(table_data) = table {
                    println!("Số hàng: {}", table_data.rows.len());
                    // Có thể thêm các thông tin khác về bảng ở đây
                }
            }
        },
        Err(e) => println!("Lỗi khi đọc file: {}", e),
    }
}
