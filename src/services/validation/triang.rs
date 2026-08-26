use crate::models::TrainData;

pub fn validate_train(train: &TrainData) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    // Validation will go here.

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}