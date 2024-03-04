fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ureq::builder().no_delay(true).build();
    // for i in 1..=1000 {
        let response = client
            .get("")
            .set("connection", "keep-alive")
            .set("Cookie", "")
            .call()?
            .into_string()?;
        println!("{response}");
        let start = (response.find("window.__initData__").unwrap() as usize) + 22;
        let end = (response.find("window.__checkoutTracker__").unwrap() as usize) - 10;
        let ss = &response[start..end];
        println!("{:?}", ss);
        // // }
    Ok(())
}