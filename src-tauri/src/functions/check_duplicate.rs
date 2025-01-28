use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};
use once_cell::sync::Lazy;
use anyhow::Result;

pub static MODEL: Lazy<TextEmbedding> = Lazy::new(|| {
    TextEmbedding::try_new(
        InitOptions::new(EmbeddingModel::AllMiniLML6V2)
            .with_show_download_progress(true)
    ).expect("Failed to initialize embedding model")
});

fn cosine_similarity(v1: &[f32], v2: &[f32]) -> f32 {
    let dot_product: f32 = v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum();
    let norm1: f32 = v1.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm2: f32 = v2.iter().map(|x| x * x).sum::<f32>().sqrt();
    dot_product / (norm1 * norm2)
}

// pub fn check_duplicate_text(text1: &str, text2: &str) -> Result<()> {
//     let documents = vec![text1, text2];
//     let embeddings = MODEL.embed(documents, None)?;
    
//     let similarity = cosine_similarity(&embeddings[0], &embeddings[1]);
//     println!("\nCosine Similarity: {}", similarity);

//     Ok(())
// }

fn main() -> Result<()> {
    let text1 = "Which of the following jobs was called by the Harvard Business Review the sexiest job of the 21st century?";
    let text2 = "According to the Harvard Business Review, which job has been referred to as the 'sexiest job of the 21st century'?";
    
    check_duplicate_text(text1, text2)?;
    
    Ok(())
}

