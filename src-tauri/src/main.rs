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
use docx_rust::DocxFile;
use docx_rust::document::BodyContent;
use crate::middleware::fill_format::extract_cell_text;
use crate::middleware::fill_format::EMBEDDING_MODEL;
use std::collections::HashMap;
use crate::services::load_accurancy::load_similarity_threshold;
use crate::middleware::check_duplicate_answers::check_duplicate_answers;

#[tauri::command]
async fn process_docx(file_path: String) -> Result<String, String> {
    let similarity_threshold = load_similarity_threshold()?;
    
    let path = PathBuf::from(file_path);
    let docx_result = read_docx_content(path.to_str().unwrap_or(""));
    let db_result = query_db();
    
    match (docx_result, db_result) {
        (Ok(questions), Ok(embeddings)) => {
            let mut results = Vec::new();
            let mut similarity_cache: HashMap<(String, String), (f32, f32)> = HashMap::new();
            
            for (i, docx_item1) in questions.iter().enumerate() {
                // Check trùng lặp trong cùng câu hỏi trước
                if let Some((ans1, ans2, similarity)) = check_duplicate_answers(&docx_item1.answers) {
                    results.push(serde_json::json!({
                        "id": docx_item1.id,
                        "docx_question": docx_item1.text,
                        "docx_answer": docx_item1.correct_answers,
                        "answers": docx_item1.answers,
                        "true_answers": docx_item1.correct_answers,
                        "correct_answer_keys": docx_item1.correct_answer_keys,
                        "duplicate_answers": {
                            "answer1": ans1,
                            "answer2": ans2,
                            "similarity": similarity
                        },
                        "duplicate_type": "internal",
                        "is_similar": true
                    }));
                    continue;  // Bỏ qua các check khác nếu phát hiện trùng nội bộ
                }
                
                // Tiếp tục với logic check trùng hiện tại nếu không có trùng nội bộ
                let mut found_similar = false;
                
                for (j, docx_item2) in questions.iter().enumerate() {
                    if i != j {
                        let cache_key = if docx_item1.id < docx_item2.id {
                            (docx_item1.id.clone(), docx_item2.id.clone())
                        } else {
                            (docx_item2.id.clone(), docx_item1.id.clone())
                        };

                        let (question_similarity, answer_similarity) = if let Some(&cached_result) = similarity_cache.get(&cache_key) {
                            cached_result
                        } else {
                            let q_sim = calculate_cosine_similarity(
                                &docx_item1.question_embedding,
                                &docx_item2.question_embedding
                            );
                            
                            let a_sim = if docx_item1.correct_answers.len() != docx_item2.correct_answers.len() {
                                0.0
                            } else {
                                let mut answer_similarities = Vec::new();
                                
                                for (i, ans1) in docx_item1.correct_answers.iter().enumerate() {
                                    if let Some(ans2) = docx_item2.correct_answers.get(i) {
                                        let ans1_embedding = EMBEDDING_MODEL.embed(vec![ans1], None)
                                            .map_err(|e| e.to_string())?
                                            .remove(0);
                                        let ans2_embedding = EMBEDDING_MODEL.embed(vec![ans2], None)
                                            .map_err(|e| e.to_string())?
                                            .remove(0);
                                        let similarity = calculate_cosine_similarity(&ans1_embedding, &ans2_embedding);
                                        answer_similarities.push(similarity);
                                    }
                                }
                                
                                if answer_similarities.is_empty() {
                                    0.0
                                } else {
                                    answer_similarities.iter().sum::<f32>() / answer_similarities.len() as f32
                                }
                            };

                            similarity_cache.insert(cache_key, (q_sim, a_sim));
                            (q_sim, a_sim)
                        };

                        if question_similarity > similarity_threshold && answer_similarity > similarity_threshold {
                            results.push(serde_json::json!({
                                "id": docx_item1.id,
                                "docx_question": docx_item1.text,
                                "docx_answer": docx_item1.correct_answers,
                                "answers": docx_item1.answers,
                                "true_answers": docx_item1.correct_answers,
                                "correct_answer_keys": docx_item1.correct_answer_keys,
                                "similar_docx_question": docx_item2.text,
                                "similar_docx_answer": docx_item2.correct_answers,
                                "similar_answers": docx_item2.answers,
                                "similar_true_answers": docx_item2.correct_answers,
                                "question_similarity": question_similarity,
                                "answer_similarity": answer_similarity,
                                "duplicate_type": "docx",
                                "is_similar": true
                            }));
                            found_similar = true;
                        }
                    }
                }
                
                if !found_similar {
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
                            results.push(serde_json::json!({
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
                            }));
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
                            results.push(serde_json::json!({
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
                            }));
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

#[tauri::command]
async fn filter_docx(file_path: String, duplicate_ids: Vec<String>) -> Result<(), String> {
    let doc_file = DocxFile::from_file(&file_path)
        .map_err(|e| e.to_string())?;
    let mut docx = doc_file.parse()
        .map_err(|e| e.to_string())?;

    let duplicate_numbers: Vec<i32> = duplicate_ids
        .iter()
        .filter_map(|id| id.parse::<i32>().ok())
        .collect();

    docx.document.body.content.retain(|content| {
        if let BodyContent::Table(table) = content {
            if let Some(first_row) = table.rows.first() {
                if let Some(first_cell) = first_row.cells.first() {
                    let cell_text = extract_cell_text(first_cell).trim().to_string();
                    
                    if let Some(id_str) = cell_text.strip_prefix("QN=") {
                        if let Ok(id_num) = id_str.trim().parse::<i32>() {
                            return duplicate_numbers.contains(&id_num);
                        }
                    }
                }
            }
        }
        true
    });

    docx.write_file("./filter.docx")
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![process_docx, filter_docx])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
