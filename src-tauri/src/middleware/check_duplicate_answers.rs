use crate::functions::cosine_calculate::calculate_cosine_similarity;
use crate::middleware::fill_format::EMBEDDING_MODEL;
use crate::services::load_accurancy::load_similarity_threshold;

pub fn check_duplicate_answers(answers: &Vec<String>) -> Option<(String, String, f32)> {
    // Load threshold từ config
    let threshold = match load_similarity_threshold() {
        Ok(t) => t,
        Err(_) => return None,
    };
    
    // Bỏ qua nếu câu hỏi có ít hơn 2 đáp án
    if answers.len() < 2 {
        return None;
    }
    
    // Tạo embeddings cho tất cả các đáp án
    let mut embeddings = Vec::new();
    for ans in answers {
        // Bỏ qua các đáp án quá ngắn (ví dụ: "Kai", "Cat", "Hat")
        if ans.len() <= 3 {
            continue;
        }
        
        match EMBEDDING_MODEL.embed(vec![ans], None) {
            Ok(mut emb) => embeddings.push((ans, emb.remove(0))),
            Err(_) => continue,
        }
    }
    
    // So sánh từng cặp đáp án có nội dung đủ dài
    for i in 0..embeddings.len() {
        for j in (i + 1)..embeddings.len() {
            let similarity = calculate_cosine_similarity(&embeddings[i].1, &embeddings[j].1);
            
            if similarity > threshold {
                return Some((
                    embeddings[i].0.clone(),
                    embeddings[j].0.clone(),
                    similarity
                ));
            }
        }
    }
    
    None
}