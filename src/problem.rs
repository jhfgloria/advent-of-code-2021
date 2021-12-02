pub fn get_problem(day: i8) -> Result<String, reqwest::Error> {
  let client = reqwest::blocking::Client::new();

  let session_cookie = "session=53616c7465645f5f17aa8782e3d7c3ab5204548403114cdec23232aa5f7818f4c9e227e9638b4c2ea37cce87426d4f4b";
  let url = format!("https://adventofcode.com/2021/day/{}/input", day);
  let response = client.get(url).header("Cookie", session_cookie).send();
  
  return match response {
    Ok(value) => value.text(),
    Err(error) => Err(error),
  }
}
