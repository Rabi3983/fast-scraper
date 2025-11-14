rustuse std::{thread, time};

pub fn retry_request<F, T>(mut operation: F, retries: u32, delay: u64) -> Result<T, String>
where
    F: FnMut() -> Result<T, String>,
{
    let mut attempts = 0;
    while attempts < retries {
        match operation() {
            Ok(result) => return Ok(result),
            Err(e) => {
                attempts += 1;
                if attempts >= retries {
                    return Err(format!("Operation failed after {} retries: {}", retries, e));
                }
                thread::sleep(time::Duration::from_secs(delay));
            }
        }
    }
    Err("Failed after maximum retries".into())
}