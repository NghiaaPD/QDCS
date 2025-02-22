#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod functions;
mod services;
mod middleware;

use middleware::fill_format::read_docx_content;
use services::querydb::query_db;
use std::path::PathBuf;
use serde_json;
use functions::cosine_calculate::calculate_cosine_similarity;
use functions::plot_similarity::calculate_similarity_score;

#[tauri::command]
async fn process_docx(file_path: String) -> Result<String, String> {
    let path = PathBuf::from(file_path);

    let docx_result = read_docx_content(path.to_str().unwrap_or(""));
    let db_result = query_db();
    
    match (docx_result, db_result) {
        (Ok(questions), Ok(embeddings)) => {
            let mut results = Vec::new();
            let mut processed_questions = std::collections::HashSet::new();
            
            // Check similarity between docx questions
            for (i, docx_item1) in questions.iter().enumerate() {
                // Nếu câu hỏi đã được xử lý thì bỏ qua
                if processed_questions.contains(&docx_item1.text) {
                    continue;
                }
                
                let mut found_similar = false;
                
                // Check với các câu hỏi khác trong docx
                for (j, docx_item2) in questions.iter().enumerate() {
                    if i != j {
                        let question_similarity = calculate_cosine_similarity(
                            &docx_item1.question_embedding,
                            &docx_item2.question_embedding
                        );
                        
                        let answer_similarity = calculate_cosine_similarity(
                            &docx_item1.answer_embedding,
                            &docx_item2.answer_embedding
                        );
                        
                        if question_similarity > 0.5 && answer_similarity > 0.5 {
                            results.push(serde_json::json!({
                                "docx_question": docx_item1.text,
                                "docx_answer": docx_item1.correct_answer,
                                "answers": docx_item1.answers,
                                "true_answer": docx_item1.correct_answer,
                                "similar_docx_question": docx_item2.text,
                                "similar_docx_answer": docx_item2.correct_answer,
                                "similarity_score": calculate_similarity_score(question_similarity, answer_similarity),
                                "is_similar": true
                            }));
                            found_similar = true;
                            processed_questions.insert(docx_item1.text.clone());
                            processed_questions.insert(docx_item2.text.clone());
                            break;
                        }
                    }
                }
                
                // Nếu chưa tìm thấy trùng lặp trong docx, kiểm tra với database
                if !found_similar && !processed_questions.contains(&docx_item1.text) {
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
                        
                        if question_similarity > 0.5 && answer_similarity > 0.5 {
                            results.push(serde_json::json!({
                                "docx_question": docx_item1.text,
                                "docx_answer": docx_item1.correct_answer,
                                "answers": docx_item1.answers,
                                "true_answer": docx_item1.correct_answer,
                                "db_question": "Question from DB",
                                "db_answer": "Answer from DB",
                                "similarity_score": calculate_similarity_score(question_similarity, answer_similarity),
                                "is_similar": true
                            }));
                            processed_questions.insert(docx_item1.text.clone());
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
                    
                    // Nếu không tìm thấy trùng lặp nào
                    if !found_similar {
                        if let Some((_db_item, (q_sim, a_sim), _)) = max_similarity {
                            results.push(serde_json::json!({
                                "docx_question": docx_item1.text,
                                "docx_answer": docx_item1.correct_answer,
                                "answers": docx_item1.answers,
                                "true_answer": docx_item1.correct_answer,
                                "db_question": "Question from DB",
                                "db_answer": "Answer from DB",
                                "similarity_score": calculate_similarity_score(q_sim, a_sim),
                                "is_similar": false
                            }));
                            processed_questions.insert(docx_item1.text.clone());
                        }
                    }
                }
            }
            
            let result = serde_json::json!({
                "similarities": results
            });
            Ok(result.to_string())
        },
        (Err(e), _) => Err(e.to_string()),
        (_, Err(e)) => Err(e.to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![process_docx])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
