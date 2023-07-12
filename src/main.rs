
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

extern crate serde;
extern crate serde_json;

use std::path::{Path, PathBuf};
//use std::simd::Which;
use rocket::data::{self, FromData};
use rocket::fs::NamedFile;
use rocket::fs::FileServer;
use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_dyn_templates::Template;
use serde::de::DeserializeOwned;

use rocket::outcome::Outcome;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

use rocket::{get, post, routes,response::Redirect};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};
use sqlx::{migrate::MigrateDatabase, Sqlite};

//use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use serde::Serialize;


use std::fs;
use std::io::prelude::*;
use serde_json::{Value, from_str, to_string_pretty};
use tokio::runtime::Runtime;
use std::process::Command;
use rocket::config::Config;
use rand::Rng;
use std::thread;
 
use chrono::{TimeZone, Local, Datelike, Timelike, Utc,};

//use rocket_dyn_templates::Template;

pub mod notificationservice;

static MACOFMARC   :&str = "mac_address_placeholder1";
static MACOFMIKIYA :&str = "mac_address_placeholder2";
static MACOFNICLAS :&str = "mac3_address_placeholder3";

//static ADDRESSOFSELF :&str = "http://haushaltsserver.ddns.net";

use notificationservice::{ADDRESSOFSELF,EMAILOFMARC,EMAILOFMIKIYA,EMAILOFNICLAS,Table};

const DB_PATH: &str = "sqlite://sqlite.db";

#[derive(FromForm)]
#[derive(Clone, PartialEq, Deserialize, Debug, Serialize)]
pub struct Entschuldigung {
    who: String,
    key: u64,
    excuse: String
}

#[derive(Clone, PartialEq, Deserialize, Debug, Serialize)]
pub enum WhichAdmin {
    Marc,
    Mikiya,
    Niclas,
    NoAdmin
}

impl WhichAdmin {
    fn to_str(&self) -> &'static str {
        match self {
            WhichAdmin::Marc =>"marc",
            WhichAdmin::Mikiya => "mikiya",
            WhichAdmin::Niclas => "niclas",
            WhichAdmin::NoAdmin => "No Admin",
        }
    }
    fn email(&self) -> &'static str{
        match self {
            WhichAdmin::Marc =>EMAILOFMARC,
            WhichAdmin::Mikiya => EMAILOFMIKIYA,
            WhichAdmin::Niclas => EMAILOFNICLAS,
            WhichAdmin::NoAdmin => "No Admin",
        }
    }
}

#[derive(FromForm)]
struct Input {
    name: String,
    age: i32,
}

#[derive(FromForm)]
struct MyFormData {
    one: String,
}


/* #[derive(FromForm)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Table {
    marc: (String,String),
    mikiya: (String,String),
    niclas: (String,String),
    week: usize
}  */

#[derive(FromForm)]

#[derive(Serialize, Deserialize, Debug, Clone,)]
struct PendingTable {
    replacing: String,
    what: String,
    week: usize,
    requested_by: String,
    when_exactly: String,
    key: u64
}

#[derive(Clone, PartialEq, Deserialize, Debug, Serialize)]
struct LastDone {
    kitchen: String,
    doorway: String,
    bathroom: String,
    id: usize
}


pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        //response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS, DELETE"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "*"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        //response.set_
    }
}


use rocket::response::{content, status};

#[get("/login")]
fn get_login() -> rocket_dyn_templates::Template {
    rocket_dyn_templates::Template::render("login", serde_json::json!({}))
}

/* #[get("/whichuser")]
async fn which_user( auth: Auth<'_>) -> String {
    let user = auth.get_user().await;
    format!("{:#?}",user)
} */

 #[get("/whichuser")]
async fn which_user( auth: Auth<'_>) -> Option<Json<WhichAdmin>> {
    let user = auth.get_user().await;

       match user {
        Some(User)=>{
            match User.is_admin {
                true=>{
                    match User.email(){
                        EMAILOFMIKIYA=> {Some(Json(WhichAdmin::Mikiya))}
                        EMAILOFMARC=> {Some(Json(WhichAdmin::Marc))}
                        EMAILOFNICLAS=> {Some(Json(WhichAdmin::Niclas))}
                        _ => {println!("we don`t match this admin"); None}
                    }                
                }
                false=>{
                    Some(Json(WhichAdmin::NoAdmin))
                }
            }
        }
        None=>None
    }    
   // Some(Json(WhichAdmin::Mikiya))


}  

 #[get("/")]
async fn index() -> NamedFile {
    NamedFile::open("static/index.html").await.unwrap()
} 

#[get("/status")]
async fn secure() -> NamedFile {
    NamedFile::open("static/index.html").await.unwrap()
}

/* 
#[post("/<path>", data = "<form>")]
    fn handle_post_request(path: String, form: Form<MyFormData>) -> content::RawJson<MyResponseData> {
        // Extract the form data from the request
        let form_data = form.into_inner();
        // Do something with the form data and path parameter
        // ...
        // Return a JSON response
        content::RawJson(MyResponseData { success:true, message:"122".to_string(), data:[0,2].to_vec() })
    }
*/
fn update_json_file(week: String, marc: String, mikiya:String, niclas:String) -> Result<(), std::io::Error> {
    // Read the JSON file
    let mut file = fs::File::open("static/example.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the contents of the file into a JSON object
    let mut data: Value = from_str(&contents)?;

    // Update the value of the specified key
    let entry = data
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|entry| entry["week"] == Value:: String(week.clone().into()));
        //println!("{:#?}", from_str(&contents)?);
    if let Some(entry) = entry {
        // Update the value of the specified user
        entry["marc"] = Value::String(marc);
        entry["niclas"] = Value::String(niclas);
        entry["mikiya"] = Value::String(mikiya);
    } else {
        // The specified week was not found in the JSON file
        // You can handle this case as needed
        println!("nononononoononononon");
        println!("{}", week.clone().to_string());
    }

    // Write the modified data back to the file
    let mut file = fs::File::create("static/example.json")?;
    file.write_all(to_string_pretty(&data).unwrap().as_bytes())?;

    Ok(())
}

#[post("/signup", data="<form>")]
    async fn signup(form: Form<Signup>, auth: Auth<'_>) -> Result<&'static str, Error> {
        match auth.signup(&form).await {
            Ok(okay)=>{ match auth.login_for(&form.into(),std::time::Duration::from_secs(60 * 60)).await {
                Ok(_)=>{
                    println!("logged in successfully");
                    println!("is someone logged in {:?}", auth.get_user().await);
                    println!("is there a session? {:?}", auth.get_session());
                }
                Err(ertor)=>{println!("{:?}",ertor);}
            }
                println!("sign up okay");println!("is someone logged in ????????????????{:?}", auth.get_user().await);Ok("You signed up.")}
            Err(error)=>{println!("{:?}",error);Err(error)}
        }
        //auth.login(&form.into()).await?;
        //println!("is someone logged in ????????????????{:?}", auth.get_user().await);
        
    }
    

//async fn login(form: rocket::serde::json::Json<Login>, auth: Auth<'_>) -> Result<&'static str, Error> {
/* #[post("/login", data="<form>")]    
    async fn login(form: rocket::serde::json::Json<Login>, auth: Auth<'_>) -> Result<&'static str, Error> {
        let result=auth.login(&form).await;
        println!("login attempt: {:?}", result);
        println!("is someone logged in ????????????????{:?}", auth.get_user().await);
        if auth.is_auth() {
            println!("Yes you are.authenticated");
        } else {
            println!("nope. you are not authenticated");
        }
        match result {
            Ok(_)=>{Redirect::to("/");println!("is someone logged in ????????????????{:?}", auth.get_user().await);Ok("Json(token)")}
            Err(error)=>{
                //Err(format!("{:?}",result))
                Err(error)
            }
        }
        

    } */

    #[post("/login", data = "<form>")]
        async fn post_login(auth: Auth<'_>, form: Form<Login>) -> Result<Redirect, Error> {
            let result = auth.login(&form).await;
            println!("login attempt: {:?}", result);
            result?;
            println!("is someone logged in ????????????????{:?}", auth.get_user().await);
            Ok(Redirect::to("/"))
        }

#[get("/logout")]
fn logout(auth: Auth<'_>) {
    auth.logout();
}

#[post("/user", data = "<form>")]
    fn json_update_request(form: Form<Table>) {
        // Extract the form data from the request
        let form_data = form.into_inner();

        println!("{}", form_data.week);
        println!("{}", form_data.mikiya.0);
        println!("{}", form_data.marc.0);
        println!("{}", form_data.niclas.0);

        let week = form_data.week;
        let mikiya = form_data.mikiya;
        let marc = form_data.marc;
        let niclas = form_data.niclas;
        // Do something with the form data and path parameter
        // ...
        // Return a JSON response
        
        match update_json_file(week.to_string(), marc.0.to_string(), mikiya.0.to_string(), niclas.0.to_string()) {
            Ok(_) => println!("Successfully updated JSON file"),
            Err(error) => println!("Error updating JSON file: {}", error),
        }
    }

#[post("/", data = "<form>")]
    fn handle_post_request(form: Form<MyFormData>) {
        // Extract the form data from the request
        let form_data = form.into_inner();
        println!("{}", form_data.one);
        println!("{}", form_data.one);
        println!("{}", form_data.one);
        // Do something with the form data and path parameter
        // ...
        // Return a JSON response
    }



#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[get("/affirm_3L/<key>")]
async fn affirm(key :u64) -> String {

    //read local json into string
    let contents_pending = fs::read_to_string("private/pending.json").expect("Error reading file");
    //read local json into vector of Table
    let mut tables_from_json_pending : Vec<PendingTable> = serde_json::from_str(&contents_pending).expect("Error parsing JSON");
    //get entry(s) of specified key
    let pending_tables_with_key: Vec<PendingTable> = tables_from_json_pending.clone().into_iter().filter(|table| table.key == key).collect();
    //remove Table(s) of specified week
    tables_from_json_pending.retain(|x| x.key != key);
    

    if pending_tables_with_key.len()>0 {
        //read local json into string
        let contents = fs::read_to_string("static/example.json").expect("Error reading file");
        //read local json into vector of Table
        let mut tables_from_json: Vec<Table> = serde_json::from_str(&contents).expect("Error parsing JSON");
        //get entry(s) of specified week
        let tables_with_week_from_week_received: Vec<Table> = tables_from_json.clone().into_iter().filter(|table| table.week == pending_tables_with_key[0].week).collect();
        //remove Table(s) of specified week
        tables_from_json.retain(|x| x.week != pending_tables_with_key[0].week);
        //update entries for that week

        
        let start_time = Local.ymd(2023, 01, 01).and_hms(08, 00, 0);
        let date_today = Local::now();
        let weeks_elapsed;
        let mut table_to_be_manipulated;
        if date_today>start_time{
            weeks_elapsed = (date_today-start_time).num_weeks();
        }
        else {
            weeks_elapsed = 0;
        }
        let mut marc=("0","lol");
        let mut niclas=("0","lol");
        let mut mikiya=("0","lol");
        if tables_with_week_from_week_received.len()>0 {
        table_to_be_manipulated = tables_with_week_from_week_received[0].clone();
        }
        else {
            table_to_be_manipulated = Table{marc:(marc.0.to_string(),marc.1.to_string()),mikiya:(mikiya.0.to_string(),mikiya.1.to_string()),niclas:(niclas.0.to_string(),niclas.1.to_string()),week:weeks_elapsed as usize};
        }

        match pending_tables_with_key[0].requested_by.as_str() {
            "marc"=>{marc=(pending_tables_with_key[0].when_exactly.as_str(),pending_tables_with_key[0].what.as_str())}
            "mikiya"=>{mikiya=(pending_tables_with_key[0].when_exactly.as_str(),pending_tables_with_key[0].what.as_str())}
            "niclas"=>{niclas=(pending_tables_with_key[0].when_exactly.as_str(),pending_tables_with_key[0].what.as_str())}
            &_ =>{println!("oh nein wir haben uns verschrieben")}
        }
        let entry_to_be_merged: Table = Table { marc: (marc.0.to_owned(),marc.1.to_owned()), mikiya: (mikiya.0.to_owned(),mikiya.1.to_owned()), niclas: (niclas.0.to_owned(),niclas.1.to_owned()), week: pending_tables_with_key[0].week };
        //let entry_to_be_merged: Table = Table { marc: (), mikiya: (), niclas: (), week: pending_tables_with_key[0].week };
        if tables_with_week_from_week_received.len() >0{
            if table_to_be_manipulated.niclas.0.eq("0") {
                match entry_to_be_merged.niclas.1.as_str() {
                    "lol"=>{ table_to_be_manipulated.niclas.0 = entry_to_be_merged.niclas.0;}
                    &_ =>{ table_to_be_manipulated.niclas = entry_to_be_merged.niclas}
                }
            }

            if table_to_be_manipulated.mikiya.0.eq("0") {
                match entry_to_be_merged.mikiya.1.as_str() {
                    "lol"=>{ table_to_be_manipulated.mikiya.0 = entry_to_be_merged.mikiya.0;}
                    &_ =>{ table_to_be_manipulated.mikiya = entry_to_be_merged.mikiya}
                }
            }
            if table_to_be_manipulated.marc.0.eq("0") {
                match entry_to_be_merged.marc.1.as_str() {
                    "lol"=>{ table_to_be_manipulated.marc.0 = entry_to_be_merged.marc.0;}
                    &_ =>{ table_to_be_manipulated.marc = entry_to_be_merged.marc}
                }
            }

            tables_from_json.push(table_to_be_manipulated);
        }
        else {
            //let mut table_to_be_manipulated = Table { marc: table_received.marc, mikiya: table_received.mikiya, niclas: table_received.niclas, week: table_received.week };
            tables_from_json.push(entry_to_be_merged);
        }
        let new_contents = serde_json::to_string_pretty(&tables_from_json).expect("Error serializing tables");
        let new_contents_pending = serde_json::to_string_pretty(&tables_from_json_pending).expect("Error serializing tables");
        fs::write("static/example.json", new_contents).expect("Error writing to file");
        fs::write("private/pending.json", new_contents_pending).expect("Error writing to file");
        change_zustande();
    }
    else {
       return "wrong key".to_string();
    }
    return "complete".to_string();
   // rocket_dyn_templates::Template::render("login", serde_json::json!({}))
}

#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[get("/reach")]
async fn reach() -> Option<Json<[i32; 3]>> {
    let tocheck :[&str;3] = [
        MACOFMARC,
        MACOFMIKIYA,
        MACOFNICLAS
    ];

    let mut anwesenheit = [0,0,0];

    let output = Command::new("sudo")
        .arg("arp-scan")
        .arg("-xq")
        .arg("192.168.0.1/24")
        .output()
        .expect("failed to execute process");

    let output_str = String::from_utf8_lossy(&output.stdout);

    let mac_addresses = output_str
        .lines()
        .filter_map(|line| {
            let cols: Vec<&str> = line.split_whitespace().collect();
            if cols.len() > 1 {
                Some(cols[1]) //if there are two collumns second containing the mac address gets returned
            } else {
                None
            }
        })
    .collect::<Vec<_>>()
    .join("\n");

    println!("{}", mac_addresses);

    for adress in tocheck.iter().enumerate(){
        if mac_addresses.contains(adress.1){anwesenheit[adress.0]=1;}
    }

    println!("{:?}", anwesenheit);
    Some(Json(anwesenheit))
}
//use rocket::response::status::Status;

#[options("/tables")]
fn options_handler() -> Status {
    Status::Ok
}

#[options("/reach")]
fn options2_handler() -> Status {
    Status::Ok
}

#[options("/login")]
fn options3_handler() -> Status {
    Status::Ok
}
#[options("/signup")]
fn options4_handler() -> Status {
    Status::Ok
}
#[options("/tables_pending")]
fn options5_handler() -> Status {
    Status::Ok
}

#[post("/tables_pending", format = "application/json", data = "<table>")]
    fn add_pending_table(table: Json<Table>, user:User) -> Status {
        let table_received = table.into_inner();

        let for_whom;
        let to_address;
        let which_week = table_received.week;
        let what;
        let when;
        let mut rng = rand::thread_rng();
        let key = rng.gen_range(184467..18446744073709551615);
        match user.email(){
            y if y==EMAILOFMARC=>{for_whom="marc";}
            y if y==EMAILOFMIKIYA=>{for_whom="mikiya";}
            y if y==EMAILOFNICLAS=>{for_whom="niclas";}
            &_ => {return Status::SeeOther}
        }
        match (table_received.niclas.0.as_str(), table_received.mikiya.0.as_str(), table_received.marc.0.as_str()) {
            ("0","0",&_)=>{to_address=WhichAdmin::Marc; what=table_received.marc.1; when=table_received.marc.0;}
            ("0",&_,"0")=>{to_address=WhichAdmin::Mikiya;what=table_received.mikiya.1; when=table_received.mikiya.0;}
            (&_,"0","0")=>{to_address=WhichAdmin::Niclas;what=table_received.niclas.1; when=table_received.niclas.0;}
            (&_,&_,&_) =>{return Status::BadRequest}
        }
        let pending_request = PendingTable {
            what: what.clone(),
            week: which_week,
            requested_by: for_whom.to_string(),
            when_exactly: when.clone(),
            replacing: to_address.to_str().to_string(),
            key: key
        };
        //read local json into string
        let contents = fs::read_to_string("private/pending.json").expect("Error reading file");
        //read local json into vector of Table
        let mut pending_tables_from_json: Vec<PendingTable> = serde_json::from_str(&contents).expect("Error parsing JSON");
        pending_tables_from_json.push(pending_request);
        let new_contents = serde_json::to_string_pretty(&pending_tables_from_json).expect("Error serializing tables");
        fs::write("private/pending.json", new_contents).expect("Error writing to file");

        notificationservice::lol(to_address.email().to_string(), for_whom.to_string(), when.to_string(), what.to_string(), format!("{}/affirm_3L/{}",ADDRESSOFSELF,key).to_string()); 
        Status::Ok
        }
        

/* async fn notifier() -> String {

} */


#[post("/tables", format = "application/json", data = "<table>")]
    fn add_table(table: Json<Table>, user:User) -> Status {
        let table_received = table.into_inner();

        println!("{:#?}", table_received.clone());
        println!("{:#?}", "tables_with_week_received");

        //read local json into string
        let contents = fs::read_to_string("static/example.json").expect("Error reading file");
        //read local json into vector of Table
        let mut tables_from_json: Vec<Table> = serde_json::from_str(&contents).expect("Error parsing JSON");
        //get entry(s) of specified week
        let tables_with_week_from_week_received: Vec<Table> = tables_from_json.clone().into_iter().filter(|table| table.week == table_received.week).collect();
        //remove Table(s) of specified week
        tables_from_json.retain(|x| x.week != table_received.week);
        //update entries for that week


        if tables_with_week_from_week_received.len()>0 {
            let mut table_to_be_manipulated = tables_with_week_from_week_received[0].clone();

            if table_to_be_manipulated.niclas.0.eq("0") {
                table_to_be_manipulated.niclas = table_received.niclas;
            }

            if table_to_be_manipulated.mikiya.0.eq("0") {
                table_to_be_manipulated.mikiya = table_received.mikiya;
            }
            if table_to_be_manipulated.marc.0.eq("0") {
                table_to_be_manipulated.marc = table_received.marc;
            }

            tables_from_json.push(table_to_be_manipulated);

        }
        else {
            let mut table_to_be_manipulated = Table { marc: table_received.marc, mikiya: table_received.mikiya, niclas: table_received.niclas, week: table_received.week };
            tables_from_json.push(table_to_be_manipulated);
        }

            
       
        let new_contents = serde_json::to_string_pretty(&tables_from_json).expect("Error serializing tables");
        fs::write("static/example.json", new_contents).expect("Error writing to file");

        change_zustande();

        Status::Ok
    }

fn change_zustande(){
    
    let contents_zustande = fs::read_to_string("static/zustande.json").expect("Error reading file");
     let mut states_from_json: Vec<LastDone> = serde_json::from_str(&contents_zustande).expect("Error parsing JSON");
     
     let mut highest_id_in_zustande: usize = 0;
     for zustand_in_json in states_from_json.iter(){
         if zustand_in_json.id>highest_id_in_zustande{
             highest_id_in_zustande=zustand_in_json.id;
         }
     }

    //read local json into string
    let contents = fs::read_to_string("static/example.json").expect("Error reading file");
    //read local json into vector of Table
    let mut tables_from_json: Vec<Table> = serde_json::from_str(&contents).expect("Error parsing JSON");

    let mut to_write_as_vec:Vec<LastDone> = vec![];
    let mut id_left_off = highest_id_in_zustande.clone();

    for Table in tables_from_json.iter()  {
        let mut kitchen_last_done="01-01-2023".to_string();
        let mut doorway_last_done="01-01-2023".to_string();
        let mut bathroom_last_done="01-01-2023".to_string();

        if Table.niclas.0.eq("0") ==false{
            match Table.niclas.1.as_str() {
                "kitchen"=>{
                    kitchen_last_done=Table.niclas.0.clone();
                }
                "doorway"=>{
                    doorway_last_done=Table.niclas.0.clone();
                }
                "bathroom"=>{
                    bathroom_last_done=Table.niclas.0.clone();
                }
                &_ => {

                }
            }
        }

        if Table.marc.0.eq("0") ==false{
            match Table.marc.1.as_str() {
                "kitchen"=>{
                    kitchen_last_done=Table.marc.0.clone();
                }
                "doorway"=>{
                    doorway_last_done=Table.marc.0.clone();
                }
                "bathroom"=>{
                    bathroom_last_done=Table.marc.0.clone();
                }
                &_ => {

                }
            }
        }

        if Table.mikiya.0.eq("0") ==false{
            match Table.mikiya.1.as_str() {
                "kitchen"=>{
                    kitchen_last_done=Table.mikiya.0.clone();
                }
                "doorway"=>{
                    doorway_last_done=Table.mikiya.0.clone();
                }
                "bathroom"=>{
                    bathroom_last_done=Table.mikiya.0.clone();
                }
                &_ => {
                }
            }
        }        
        states_from_json.retain(|x| x.id != Table.week);
        states_from_json.push(LastDone{kitchen:kitchen_last_done, doorway:doorway_last_done, bathroom:bathroom_last_done, id: Table.week})
    }
    let new_contents = serde_json::to_string_pretty(&states_from_json).expect("Error serializing tables");
    fs::write("static/zustande.json", new_contents).expect("Error writing to file");
     //change Zustande Json
     
}

#[get("/Excusemon/<key>")]
fn excuse(option: Option<User>, key:u64) ->  rocket_dyn_templates::Template {
    if let Some(user) = option {
            //read local json into string
        let contents_pending = fs::read_to_string("private/pending.json").expect("Error reading file");
        //read local json into vector of Table
        let mut tables_from_json_pending : Vec<PendingTable> = serde_json::from_str(&contents_pending).expect("Error parsing JSON");
        //get entry(s) of specified key
        let pending_tables_with_key: Vec<PendingTable> = tables_from_json_pending.clone().into_iter().filter(|table| table.key == key).collect();
        if pending_tables_with_key.len()>0{
            rocket_dyn_templates::Template::render("chore_excuse", serde_json::json!({"who":user.email(), "key": key}))
        }
        else {
            rocket_dyn_templates::Template::render("keyexpired", serde_json::json!({}))
        }
    }
    else {
        rocket_dyn_templates::Template::render("loginfirst", serde_json::json!({}))
    }
}

#[post("/post_excuse", data="<form>")]
    fn post_excuse(user: User, form: Form<Entschuldigung>) -> String {

    //read local json into string
    let contents_pending = fs::read_to_string("private/pending.json").expect("Error reading file");
    //read local json into vector of Table
    let mut tables_from_json_pending : Vec<PendingTable> = serde_json::from_str(&contents_pending).expect("Error parsing JSON");
    //get entry(s) of specified key
    let pending_tables_with_key: Vec<PendingTable> = tables_from_json_pending.clone().into_iter().filter(|table| table.key == form.key).collect();
    //remove Table(s) of specified week
    tables_from_json_pending.retain(|x| x.key != form.key);
    

    if pending_tables_with_key.len()>0 {
        //read local json into string
        let contents = fs::read_to_string("static/example.json").expect("Error reading file");
        //read local json into vector of Table
        let mut tables_from_json: Vec<Table> = serde_json::from_str(&contents).expect("Error parsing JSON");
        //get entry(s) of specified week
        let tables_with_week_from_week_received: Vec<Table> = tables_from_json.clone().into_iter().filter(|table| table.week == pending_tables_with_key[0].week).collect();
        //remove Table(s) of specified week
        tables_from_json.retain(|x| x.week != pending_tables_with_key[0].week);
        //update entries for that week
        let start_time = Local.ymd(2023, 01, 01).and_hms(08, 00, 0);
        let date_today = Local::now();
        let weeks_elapsed;
        let mut table_to_be_manipulated;
        if date_today>start_time{
            weeks_elapsed = (date_today-start_time).num_weeks();
        }
        else {
            weeks_elapsed = 0;
        }
        let mut marc=("0","lol");
        let mut niclas=("0","lol");
        let mut mikiya=("0","lol");
        if tables_with_week_from_week_received.len()>0 {
        table_to_be_manipulated = tables_with_week_from_week_received[0].clone();
        }
        else {
            table_to_be_manipulated = Table{marc:(marc.0.to_string(),marc.1.to_string()),mikiya:(mikiya.0.to_string(),mikiya.1.to_string()),niclas:(niclas.0.to_string(),niclas.1.to_string()),week:weeks_elapsed as usize};
        }
        match pending_tables_with_key[0].requested_by.as_str() {
            "marc"=>{marc=(pending_tables_with_key[0].when_exactly.as_str(),form.excuse.as_str())}
            "mikiya"=>{mikiya=(pending_tables_with_key[0].when_exactly.as_str(),form.excuse.as_str())}
            "niclas"=>{niclas=(pending_tables_with_key[0].when_exactly.as_str(),form.excuse.as_str())}
            &_ =>{println!("oh nein wir haben uns verschrieben")}
        }
        let entry_to_be_merged: Table = Table { marc: (marc.0.to_owned(),marc.1.to_owned()), mikiya: (mikiya.0.to_owned(),mikiya.1.to_owned()), niclas: (niclas.0.to_owned(),niclas.1.to_owned()), week: pending_tables_with_key[0].week };
        //let entry_to_be_merged: Table = Table { marc: (), mikiya: (), niclas: (), week: pending_tables_with_key[0].week };
        if tables_with_week_from_week_received.len() >0{
            if table_to_be_manipulated.niclas.0.eq("0") {
                match entry_to_be_merged.niclas.1.as_str() {
                    "lol"=>{ table_to_be_manipulated.niclas.0 = entry_to_be_merged.niclas.0;}
                    &_ =>{ table_to_be_manipulated.niclas = entry_to_be_merged.niclas}
                }
            }

            if table_to_be_manipulated.mikiya.0.eq("0") {
                match entry_to_be_merged.mikiya.1.as_str() {
                    "lol"=>{ table_to_be_manipulated.mikiya.0 = entry_to_be_merged.mikiya.0;}
                    &_ =>{ table_to_be_manipulated.mikiya = entry_to_be_merged.mikiya}
                }
            }
            if table_to_be_manipulated.marc.0.eq("0") {
                match entry_to_be_merged.marc.1.as_str() {
                    "lol"=>{ table_to_be_manipulated.marc.0 = entry_to_be_merged.marc.0;}
                    &_ =>{ table_to_be_manipulated.marc = entry_to_be_merged.marc}
                }
            }

            tables_from_json.push(table_to_be_manipulated);
        }
        else {
            //let mut table_to_be_manipulated = Table { marc: table_received.marc, mikiya: table_received.mikiya, niclas: table_received.niclas, week: table_received.week };
            tables_from_json.push(entry_to_be_merged);
        }
        let new_contents = serde_json::to_string_pretty(&tables_from_json).expect("Error serializing tables");
        let new_contents_pending =serde_json::to_string_pretty(&tables_from_json_pending).expect("Error serializing tables");
        fs::write("static/example.json", new_contents).expect("Error writing to file");
        change_zustande();
        fs::write("private/pending.json", new_contents_pending).expect("Error writing to file");
    }
    else {
       return "wrong key".to_string();
    }
    return "complete".to_string();
   // rocket_dyn_templates::Template::render("login", serde_json::json!({}))
        
        //rocket_dyn_templates::Template::render("chore_excuse", serde_json::json!({"who":user.email(), "key": key}))
    }

#[tokio::main]
async fn main() -> Result<(), Error> {

    let figment = Config::figment()
    .merge(("secret_key", "encryption_placeholder"));
    let config = Config::from(figment);
    assert!(!config.secret_key.is_zero());

    if !Sqlite::database_exists(DB_PATH).await.unwrap_or(false) {
        println!("Creating database {}", DB_PATH);
        match Sqlite::create_database(DB_PATH).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let users = Users::open_sqlite(DB_PATH).await?;
    //notificationservice::lol(EMAILOFNICLAS.to_string(), "for_whom".to_string(), "when".to_string(), "what".to_string(), format!("{}/affirm_3L/{}","ADDRESSOFSELF","key").to_string()); 
    //rocket_auth::Auth::login(&[("email", "max@email.com"),("password", "Password123")]);
    thread::spawn(|| {
        notificationservice::sauber_polizei();
    });  
    //notificationservice::sauber_polizei();
    
     let _ =rocket::build()
        .attach(CORS)
        //.mount("/", routes![signup,get_login, login, logout,options4_handler,options3_handler, options2_handler,options_handler,add_table,hello,handle_post_request,files,json_update_request,reach])
        .mount("/", routes![post_excuse, excuse, affirm,options5_handler,add_pending_table,which_user,secure,index,signup,get_login, post_login, logout,options4_handler,options3_handler, options2_handler,options_handler,add_table,hello,handle_post_request,files,json_update_request,reach])
        .manage(users)
        //.mount("/", FileServer::from("static"))
        .attach(rocket_dyn_templates::Template::fairing())
        .launch()
        .await
        .unwrap();   

    Ok(())
}