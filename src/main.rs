

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let weather_data = reqwest::Client::new()
        .get("https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline/Mae%20Chan?unitGroup=metric&include=current&key=ASRQL55QSXYKL63Y2DCEMFUC3&contentType=json")
        .send()
        .await?
        .text()
        .await?;
    println!("{:#?}", weather_data);

    Ok(())
}
