use futures::try_join;
use serde::Serialize;
use std::error::Error;

type Handle = Result<f32, Box<dyn Error>>;

#[derive(Debug, Serialize)]
pub struct Rouge {
    recall: String,
    precision: String,
    f1: String,
}

impl Rouge {
    pub fn new() -> Rouge {
        Rouge {
            recall: String::new(),
            precision: String::new(),
            f1: String::new(),
        }
    }

    pub async fn from(abs: &str, ref1: &str, ref2: &str, ref3: &str) -> Rouge {
        self::Rouge {
            recall: recall(abs, ref1, ref2, ref3).await,
            precision: precision(abs, ref1, ref2, ref3).await,
            f1: f1(abs, ref1, ref2, ref3).await,
        }
    }
}

async fn recall(abs: &str, r1: &str, r2: &str, r3: &str) -> String {
    let recall = match try_join!(
            calculate_recall(abs, r1),
            calculate_recall(abs, r2),
            calculate_recall(abs, r3),
        ) {
        Ok((a, b, c)) => {
            (a + b + c) / 3.0
        },
        Err(_) => -1.0,
    };
    recall.to_string()
}

async fn calculate_recall(abs: &str, r: &str) -> Handle {
    Ok(0.0)
}

async fn precision(abs: &str, r1: &str, r2: &str, r3: &str) -> String {
    let precision = match try_join!(
            calculate_precision(abs, r1),
            calculate_precision(abs, r2),
            calculate_precision(abs, r3),
        ) {
        Ok((a, b, c)) => (a + b + c) / 3.0,
        Err(_) => -1.0,
    };
    precision.to_string()
}

async fn calculate_precision(abs: &str, r: &str) -> Handle {
    Ok(0.0)
}

async fn f1(abs: &str, r1: &str, r2: &str, r3: &str) -> String {
    let f1 = match try_join!(
            calculate_f1(abs, r1),
            calculate_f1(abs, r2),
            calculate_f1(abs, r3),
        ) {
        Ok((a, b, c)) => (a + b + c) / 3.0,
        Err(_) => -1.0,
    };
    f1.to_string()
}

async fn calculate_f1(abs: &str, r: &str) -> Handle {
    Ok(0.0)
}
