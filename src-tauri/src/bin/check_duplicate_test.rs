use app::services::check_duplicate::check_duplicate_questions;

fn main() {
    // Các câu để test
    let text1 = "Hello ";
    let text2 = "Hello ";
    
    // Gọi hàm check_duplicate_questions và xử lý kết quả
    match check_duplicate_questions(text1, text2) {
        Ok(similarity) => {
            println!("Độ tương đồng giữa hai câu là: {:.4}", similarity);
            println!("Phần trăm tương đồng: {:.2}%", similarity * 100.0);
        },
        Err(e) => println!("Có lỗi xảy ra: {}", e),
    }
} 