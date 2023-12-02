use tokio::task::{JoinError, JoinSet};

pub async fn joinset_join_all<T: 'static>(join_set: &mut JoinSet<T>) -> Result<Vec<T>, JoinError> {
    let mut results = Vec::new();
    
    while let Some(res) = join_set.join_next().await {
        results.push(res?);
    }
    
    Ok(results)
} 