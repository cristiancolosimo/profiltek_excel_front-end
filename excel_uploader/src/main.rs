use reqwest::blocking::Client;
use std::fs::File;
use reqwest::blocking::multipart;

fn request(client: &Client, file: &String) {
    let form = multipart::Form::new().file("fileUPLOAD", file).unwrap();
    let res = client.post("https://itilab.it/santo_cristian_colosimo/elaborazione_excel/src/controller/upload.php")
        .multipart(form)
        .send().unwrap();
    println!("{:?}", res.text().unwrap());

}
fn request_convalida(client: &Client) {

    let res = client.get("https://itilab.it/santo_cristian_colosimo/elaborazione_excel/src/controller/checkfiles.php").send().unwrap();
    println!("{:?}", res.text().unwrap());
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut files : Vec<String> =  Vec::new();
    let paths = std::fs::read_dir("./files").unwrap();

    for path in paths {
        files.push(path.unwrap().path().to_str().unwrap().to_string());
        //println!("Name: {}", path.unwrap().path().display());

    }   

    for path in files {
        println!("{}", path);
        let client = Client::new();

        request(&client, &path);
  
    }
    let client = Client::new();

    request_convalida(&client);
    //let client = Client::new();
    //request(&client, &files[0]);
    /*
    let file = File::open("files/")?;

    let client = Client::new();
*/
    Ok(())
}