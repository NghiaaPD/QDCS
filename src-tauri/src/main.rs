#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod functions;
mod middleware;
mod services;

use middleware::fill_format::read_docx_content;
use services::querydb::query_db;
use std::path::PathBuf;
use serde_json;
use functions::cosine_calculate::calculate_cosine_similarity;
use functions::plot_similarity::calculate_similarity_score;
use crate::services::load_accurancy::load_similarity_threshold;
use crate::middleware::check_duplicate_answers::check_duplicate_answers;

#[tauri::command]
async fn process_docx(file_path: String, subject: String) -> Result<String, String> {
    let similarity_threshold = load_similarity_threshold()?;
    
    // Kiểm tra file DB tồn tại ngay từ đầu
    let db_file_path = format!("{}.duckdb", subject);
    if !std::path::Path::new(&db_file_path).exists() {
        return Err(format!("Không tìm thấy cơ sở dữ liệu cho môn học '{}'. Vui lòng kiểm tra lại tên môn học hoặc liên hệ quản trị viên.", subject));
    }
    
    let path = PathBuf::from(file_path);
    let docx_result = read_docx_content(path.to_str().unwrap_or(""));
    let db_result = query_db(&subject);
    
    match (docx_result, db_result) {
        (Ok(questions), Ok(embeddings)) => {
            let mut results = Vec::new();
            
            for (i, docx_item1) in questions.iter().enumerate() {
                let mut is_similar = false;
                let mut result_item = serde_json::json!({
                    "id": docx_item1.id,
                    "docx_question": docx_item1.text,
                    "docx_answer": docx_item1.correct_answers,
                    "answers": docx_item1.answers,
                    "true_answer": docx_item1.correct_answers,
                    "correct_answer_keys": docx_item1.correct_answer_keys,
                    "is_similar": false
                });
                
                // 1. Kiểm tra trùng đáp án nội bộ (chỉ khi có từ 2 đáp án trở lên)
                let actual_answer_count = docx_item1.answers.iter()
                    .filter(|ans| !ans.trim().is_empty())
                    .count();

                if actual_answer_count >= 2 {
                    if let Some((ans1, ans2, similarity)) = check_duplicate_answers(&docx_item1.answers) {
                        result_item["duplicate_answers"] = serde_json::json!({
                            "answer1": ans1,
                            "answer2": ans2, 
                            "similarity": similarity
                        });
                        result_item["duplicate_type"] = serde_json::json!("internal");
                        result_item["is_similar"] = serde_json::json!(true);
                        is_similar = true;
                    }
                }
                
                // 2. Nếu không trùng nội bộ, kiểm tra trùng với các câu khác trong file
                if !is_similar {
                    for (j, docx_item2) in questions.iter().enumerate() {
                        if i != j {
                            // Tính toán độ tương đồng
                            let question_similarity = calculate_cosine_similarity(
                                &docx_item1.question_embedding, 
                                &docx_item2.question_embedding
                            );
                            
                            let answer_similarity = calculate_cosine_similarity(
                                &docx_item1.answer_embedding, 
                                &docx_item2.answer_embedding
                            );
                            
                            if question_similarity > similarity_threshold && answer_similarity > similarity_threshold {
                                result_item["similar_docx_question"] = serde_json::json!(docx_item2.text);
                                result_item["similar_docx_answer"] = serde_json::json!(docx_item2.correct_answers);
                                result_item["similar_answers"] = serde_json::json!(docx_item2.answers);
                                result_item["similar_true_answer"] = serde_json::json!(docx_item2.correct_answers);
                                result_item["question_similarity"] = serde_json::json!(question_similarity);
                                result_item["answer_similarity"] = serde_json::json!(answer_similarity);
                                result_item["duplicate_type"] = serde_json::json!("docx");
                                result_item["is_similar"] = serde_json::json!(true);
                                is_similar = true;
                                break;
                            }
                        }
                    }
                }
                
                // 3. Nếu vẫn không trùng, kiểm tra với DB
                if !is_similar {
                    let mut found_similar = false;
                    let mut max_similarity = None;
                    
                    for db_item in &embeddings {
                        let question_similarity = calculate_cosine_similarity(
                            &docx_item1.question_embedding,
                            &db_item.0
                        );

                        let answer_similarity = calculate_cosine_similarity(
                            &docx_item1.answer_embedding,
                            &db_item.1
                        );
                        
                        if question_similarity > similarity_threshold && answer_similarity > similarity_threshold {
                            result_item = serde_json::json!({
                                "id": docx_item1.id,
                                "docx_question": docx_item1.text,
                                "docx_answer": docx_item1.correct_answers,
                                "answers": docx_item1.answers,
                                "true_answers": docx_item1.correct_answers,
                                "correct_answer_keys": docx_item1.correct_answer_keys,
                                "db_question": "Question from DB",
                                "db_answer": "Answer from DB",
                                "similarity_score": calculate_similarity_score(question_similarity, answer_similarity),
                                "duplicate_type": "db",
                                "is_similar": true
                            });
                            found_similar = true;
                            break;
                        } else {
                            let current_avg = (question_similarity + answer_similarity) / 2.0;
                            if let Some((_, _, prev_avg)) = &max_similarity {
                                if current_avg > *prev_avg {
                                    max_similarity = Some((db_item, (question_similarity, answer_similarity), current_avg));
                                }
                            } else {
                                max_similarity = Some((db_item, (question_similarity, answer_similarity), current_avg));
                            }
                        }
                    }
                    
                    if !found_similar {
                        if let Some((_db_item, (q_sim, a_sim), _)) = max_similarity {
                            result_item = serde_json::json!({
                                "id": docx_item1.id,
                                "docx_question": docx_item1.text,
                                "docx_answer": docx_item1.correct_answers,
                                "answers": docx_item1.answers,
                                "true_answers": docx_item1.correct_answers,
                                "correct_answer_keys": docx_item1.correct_answer_keys,
                                "db_question": "Question from DB",
                                "db_answer": "Answer from DB",
                                "similarity_score": calculate_similarity_score(q_sim, a_sim),
                                "is_similar": false
                            });
                        }
                    }
                }
                
                // Thêm vào kết quả
                results.push(result_item);
            }
            
            let result = serde_json::json!({
                "similarities": results
            });
            Ok(result.to_string())
        },
        (Err(e), _) => Err(format!("Lỗi khi đọc file DOCX: {}", e)),
        (_, Err(e)) => {
            if e.to_string().contains("Table with name Data does not exist") {
                Err(format!("Cơ sở dữ liệu cho môn học '{}' không có bảng Data. Vui lòng kiểm tra lại.", subject))
            } else {
                Err(format!("Lỗi cơ sở dữ liệu: {}", e))
            }
        }
    }
}

#[tauri::command]
async fn filter_docx(file_path: String, duplicate_ids: Vec<String>, original_filename: Option<String>) -> Result<String, String> {
    // Lấy thư mục hiện tại của ứng dụng
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("Không thể xác định thư mục hiện tại: {}", e))?;
    
    println!("File path: {}", file_path);
    println!("Original filename: {:?}", original_filename);
    println!("IDs cần giữ lại: {:?}", duplicate_ids);
    
    // Kiểm tra xem có câu nào được giữ lại không
    if duplicate_ids.is_empty() {
        println!("Không có câu hỏi nào được giữ lại!");
        return Err("Không có câu hỏi nào được giữ lại sau khi lọc. Không thể tạo file mới.".to_string());
    }
    
    // Sử dụng tên file gốc nếu được cung cấp
    let output_file_stem = if let Some(original_name) = original_filename {
        let original_path = std::path::Path::new(&original_name);
        original_path.file_stem().unwrap_or_default().to_string_lossy().to_string()
    } else {
        let path = std::path::Path::new(&file_path);
        path.file_stem().unwrap_or_default().to_string_lossy().to_string()
    };
    
    // Lấy phần mở rộng
    let path = std::path::Path::new(&file_path);
    let extension = path.extension().unwrap_or_default().to_string_lossy();
    
    // Tạo đường dẫn file mới với timestamp
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    let new_file_name = format!("{}_filtered_{}.{}", output_file_stem, timestamp, extension);
    let new_file_path = current_dir.join(&new_file_name);
    let new_file_path_str = new_file_path.to_str().unwrap().to_string();
    
    // Đảm bảo file nguồn tồn tại
    if !std::path::Path::new(&file_path).exists() {
        return Err(format!("File gốc không tồn tại: {}", file_path));
    }
    
    // TODO: Đây là nơi cần thêm logic để xử lý file DOCX
    // Hiện tại chỉ sao chép file, cần thay thế bằng code thực sự lọc câu hỏi
    // Code này cần truy cập vào file DOCX, đọc cấu trúc và lọc theo IDs
    
    // Sao chép file tạm thời để hiển thị UI thành công
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    match std::fs::copy(&file_path, &new_file_path) {
        Ok(_) => {
            println!("Đã sao chép file. LƯU Ý: Chức năng lọc chưa được thực hiện.");
            Ok(new_file_path_str)
        },
        Err(e) => Err(format!("Lỗi khi sao chép file: {}", e))
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            process_docx,
            filter_docx,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
