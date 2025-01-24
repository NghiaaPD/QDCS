pub fn cosine_similarity(v1: &Vec<f32>, v2: &Vec<f32>) -> f32 {
    let dot_product: f32 = v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum();
    let norm1: f32 = v1.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm2: f32 = v2.iter().map(|x| x * x).sum::<f32>().sqrt();
    
    dot_product / (norm1 * norm2)
}