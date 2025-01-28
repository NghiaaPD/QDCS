use duckdb::{Connection, Result};
use serde_json;

pub fn get_qa_pairs() -> Result<Vec<(Vec<f32>, Vec<f32>)>> {
    let conn = Connection::open("../../data.duckdb")?;
    
    let mut stmt = conn.prepare("
        SELECT 
        CAST(question_embedding AS JSON) as question_json,
        CAST(answer_embedding AS JSON) as answer_json 
        FROM data
    ")?;

    let mut pairs = Vec::new();
    
    let rows = stmt.query_map([], |row| {
        let question_json: String = row.get(0)?;
        let answer_json: String = row.get(1)?;

        let question: Vec<f32> = serde_json::from_str(&question_json)
            .expect("Không thể parse question JSON");
        let answer: Vec<f32> = serde_json::from_str(&answer_json)
            .expect("Không thể parse answer JSON");
            
        Ok((question, answer))
    })?;

    for row in rows {
        pairs.push(row?);
    }

    Ok(pairs)
}

// fn main() -> Result<()> {
//     match get_qa_pairs() {
//         Ok(pairs) => {
//             println!("Đã tìm thấy {} cặp Q&A", pairs.len());
            
//             // In ra vài cặp đầu tiên để kiểm tra
//             for (i, (question, answer)) in pairs.iter().take(2).enumerate() {
//                 println!("\nCặp thứ {}:", i + 1);
//                 println!("Question embedding: {:?}", &question[..5]); // Chỉ in 5 phần tử đầu
//                 println!("Answer embedding: {:?}", &answer[..5]);     // Chỉ in 5 phần tử đầu
//             }
//         },
//         Err(e) => println!("Lỗi: {}", e),
//     }
    
//     Ok(())
// }
