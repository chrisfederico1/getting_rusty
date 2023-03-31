use reqwest;

fn game_day_url (level: &str, year: &str, month: &str, day: &str) -> String {

    String::from("http://gd2.mlb.com/components/game/") + level
        + "/year_" + year
        + "/month_" + month
        + "/day_" + day

}

fn game_day_links (url: &str) -> Vec<String> {

    let resp = reqwest::get(url);

    if resp.is_ok() {
        let links = resp.unwrap().text().unwrap_or("".to_string());
        links.split("<li>")
            .filter(|line| line.contains("gid_"))
            .map(|line| url.to_string().clone() + "/"
                 + line.split(|c| c == '>'|| c == '<')
                 .collect::<Vec<&str>>()[2]
                 .trim()
                 )
            .collect::<Vec<String>>()
    }
    else {
        vec![]
    }
}

fn main () {
    let url =game_day_url("mlb", "2018", "06", "10");
    let _games = game_day_links(&url);

   // dbg!(games);
}
