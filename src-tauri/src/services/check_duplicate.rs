use crate::functions::cosine_calculate;
use crate::functions::embedding;

pub fn check_duplicate_questions(text1: &str, text2: &str) -> Result<f32, Box<dyn std::error::Error>> {
    let embeddings = embedding::check_duplicate_questions(text1, text2)?;
    let similarity = cosine_calculate::cosine_similarity(&embeddings[0], &embeddings[1]);
    Ok(similarity)
}




