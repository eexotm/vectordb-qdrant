enum CosSimErr {
    DimensionMismatch { a_len : usize, b_len: usize},
}
fn cosine_similarity(a: &[f32], b: &[f32]) -> Result<f32, CosSimErr> {

    if a.len() != b.len() { return Err(CosSimErr::DimensionMismatch { a_len: a.len(), b_len: b.len() }) }
    let dot: f32 = a.iter().zip(b.iter()).map(|(x,y)| x * y).sum();
    let vec_a: f32 = a.iter().map(|x| x*x).sum::<f32>().sqrt();
    let vec_b: f32 = b.iter().map(|x| x*x).sum::<f32>().sqrt();
    if vec_a*vec_b == 0.0 {return Ok(0.0)};
    let s = dot / (vec_a * vec_b);
    Ok(s)
}

fn brute_force_search(query: &[f32], vectors: &[Vec<f32>], k: usize) -> Result<Vec<(usize,f32)>, CosSimErr> {
    let mut results: Vec<(usize, f32)> = Vec::with_capacity(vectors.len());
    for (i,v) in vectors.iter().enumerate() {
        let score = cosine_similarity(query, v)?;
        results.push((i, score));
    }
    Ok(results);
}
fn main() {
    let a = vec![1.0, 2.0, 3.0];
    let b = vec![2.0, 4.0, 6.0];   
    let c = vec![-1.0, -2.0, -3.0]; 
}