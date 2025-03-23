// Training Script
// Steps
// 1. Download external CSV file to disk
// 2. Load file from disk into memory
// 3. Prepare the data
// 4. Train an XGBoost model with this data
// 5. Push this model to an AWS S3 bucket (model registry)

use anyhow::Ok;

fn main() -> anyhow::Result<()> {
    println!("Starting training script...");

    // 1. Download external CSV file to disk
    let csv_file_path = download_csv_file()?;
    println!("Downloaded the file and wrote to {}", csv_file_path);

    println!("Finished training script succssefully.");

    Ok(())
}

// Function to downlaod external CSV file
fn download_csv_file() -> anyhow::Result<String> {
    /*
    What we can learn from this function?

    - packages in Rust are called cartes
    - -> () indicates that we will not return anything (None)
    - variables are declared with the keyword let - each variable has a type at compile time!
    - modelues are called crate (does not need to be imported, unlike in Python) - in order to access modules of the library use ::
    - the response type already includes the possibility for an error (an enumeration = enum - can contain multiple values) - reqwest::blocking::get(url) returns a result
    - the ? will propogate an error (like raising an exception in Python) if there is an error, if not, unpack the response
    - if things are Ok, hence successful, return () -> which is None
    - anyhow::Result<()> encompasses both the success and error cases
        - alias for Result<T, anyhow::Error>
        - in case of success we return: ()
        - in case of error we return: anyhow::Error
    - blocking makes the process synchronous, by default the reqwest crate is asynchronous
    */

    // Define URL
    let url = "https://raw.githubusercontent.com/selva86/datasets/master/BostonHousing.csv";

    // Get response from URL
    let response = reqwest::blocking::get(url)?;

    // Get bytes from reponse into memory
    let bytes = response.bytes()?;

    let file_path = "data/boston_housing.csv";
    // Save to disk
    std::fs::write(file_path, bytes)?;

    Ok(file_path.to_string())
}
