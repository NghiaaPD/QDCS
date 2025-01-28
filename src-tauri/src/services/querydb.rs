use duckdb::{Connection, Result};

pub fn get_qa_pairs() -> Result<Vec<(Vec<f32>, Vec<f32>)>> {
    let conn = Connection::open("../../data.duckdb")?;
    
    let mut stmt = conn.prepare(
        "SELECT question_embedding, answer_embedding FROM data"
    )?;

    let mut pairs = Vec::new();
    
    let rows = stmt.query_map([], |row| {
        let question: Vec<f32> = row.get(0)?;
        let answer: Vec<f32> = row.get(1)?;
        Ok((question, answer))
    })?;

    for row in rows {
        pairs.push(row?);
    }

    Ok(pairs)
}

fn main() -> Result<()> {
    match get_qa_pairs() {
        Ok(pairs) => {
            println!("Đã tìm thấy {} cặp Q&A", pairs.len());
            
            // In ra vài cặp đầu tiên để kiểm tra
            for (i, (question, answer)) in pairs.iter().take(2).enumerate() {
                println!("\nCặp thứ {}:", i + 1);
                println!("Question embedding: {:?}", &question[..5]); // Chỉ in 5 phần tử đầu
                println!("Answer embedding: {:?}", &answer[..5]);     // Chỉ in 5 phần tử đầu
            }
        },
        Err(e) => println!("Lỗi: {}", e),
    }
    
    Ok(())
}
