

use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport,Tokio1Executor, Transport,message::*,AsyncTransport, AsyncSmtpTransport, 
};
use tera::Tera;
use tera::Context;
use serde_json::json;
use std::time::Duration;
use std::thread;
use std::fs;
use std::io::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use chrono::{TimeZone, Local, Datelike, Timelike, Utc,};
use rand::Rng;

const SECRETPASSWORD :&str = "sp1_censored";
pub const EMAILOFMIKIYA : &str = "email1_censor_placeholder";
pub const EMAILOFMARC : &str = "email2_censor_placeholder";
pub const EMAILOFNICLAS :&str = "email3_censor_placeholder";
const SMTPEMAIL :&str = "smtpmail_censor_placeholder";
pub static ADDRESSOFSELF :&str = "http://haushaltsserver.ddns.net";
//pub static ADDRESSOFSELF :&str = "http://192.168.0.81:7980";
#[derive(FromForm)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Table {
    pub marc: (String,String),
    pub mikiya: (String,String),
    pub niclas: (String,String),
    pub week: usize
}

#[derive(Clone, PartialEq, Deserialize, Debug, Serialize)]
enum WhichAdmin {
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

#[derive(Serialize, Deserialize, Debug, Clone,)]
struct PendingTable {
    replacing: String,
    what: String,
    week: usize,
    requested_by: String,
    when_exactly: String,
    key: u64
}

pub fn sauber_polizei(){
    loop {  
        thread::sleep(Duration::from_secs(60*60*15));   
        //thread::sleep(Duration::from_secs(60*60*15));
        //read local json into string
        let contents = fs::read_to_string("static/example.json").expect("Error reading file");
        //read local json into vector of Table
        let mut tables_from_json: Vec<Table> = serde_json::from_str(&contents).expect("Error parsing JSON");

        let start_time = Local.ymd(2023, 01, 01).and_hms(08, 00, 0);
        let date_today = Local::now();
        let weeks_elapsed;

        if date_today>start_time{
            weeks_elapsed = (date_today-start_time).num_weeks();
        }
        else {
            weeks_elapsed = 0;
        }

        let mut week_date_start = start_time + chrono::Duration::weeks(weeks_elapsed);
        let mut week_date_end = start_time + chrono::Duration::weeks(weeks_elapsed) + chrono::Duration::days(6);       
        let days_remaining_in_this_week = (week_date_end-date_today).num_days();


/*         for blah in 0..14 {
            let temp_week_date=format!("{:02}-{:02}-{} bis {:02}-{:02}-{}", week_date_start.day(), week_date_start.month(), week_date_start.year(), week_date_end.day(), week_date_end.month(), week_date_end.year());
            fourteen_weeks.push(temp_week_date);
            week_date_start = week_date_start + chrono::Duration::days(7);   
            week_date_end = week_date_end + chrono::Duration::days(7);   
        } */
        let mut marc_which_active:Vec<usize>=vec![];
        let mut mikiya_which_active:Vec<usize>=vec![];
        let mut niclas_which_active:Vec<usize>=vec![];

        let mut marc_which_missing:Vec<usize>=vec![];
        let mut mikiya_which_missing:Vec<usize>=vec![];
        let mut niclas_which_missing:Vec<usize>=vec![];

       let mut marc_missing = 0;
       let mut mikiya_missing= 0;
       let mut niclas_missing= 0;

       let mut marc_left_at :usize = 0;
       let mut mikiya_left_at:usize =0;
       let mut niclas_left_at:usize=0;

       println!("commencing sauber_polizei");

       let min_week = tables_from_json.iter().map(|t| t.week).min().unwrap_or(0);
       let max_week = weeks_elapsed;

       let mut missing_weeks = Vec::new();
        for week in min_week..=max_week as usize{
            if !tables_from_json.iter().any(|t| t.week == week) {
                missing_weeks.push(week);
            }
        }
        println!("missing weeks : {:?}", missing_weeks);

        // Insert new Table elements with the missing weeks at the appropriate position
        for week in missing_weeks {
            let new_table = Table { marc:("0".to_owned(),"schau bitte online".to_owned()),mikiya:("0".to_owned(),"schau bitte online".to_owned()),niclas:("0".to_owned(),"schau bitte online".to_owned()), week:week };
            let pos = tables_from_json.iter().position(|t| t.week > week).unwrap_or(tables_from_json.len());
            tables_from_json.insert(pos, new_table);
    }

       for i in 0..tables_from_json.len() {
            match tables_from_json[i].marc.0.as_str() {
                "0"=>{
                    if i >= marc_left_at{
                        let mut missing_in_span=0;
                        while let Some(is_this_zero) = tables_from_json.get(i+missing_in_span+1){
                            match is_this_zero.marc.0.as_str(){
                                "0"=>{marc_missing=marc_missing+1;marc_which_missing.push(tables_from_json[i+missing_in_span+1].week);missing_in_span=missing_in_span+1;}
                                &_=>{marc_which_active.push(tables_from_json[i+missing_in_span+1].week);break;}
                            }
                            marc_left_at=i+missing_in_span+1;
                        }
                    }
                }
                &_ => {}
            }
            match tables_from_json[i].mikiya.0.as_str() {
                "0"=>{
                    if i >= mikiya_left_at{
                        let mut missing_in_span=0;
                        while let Some(is_this_zero) = tables_from_json.get(i+missing_in_span+1){
                            match is_this_zero.mikiya.0.as_str(){
                                "0"=>{mikiya_missing=mikiya_missing+1;mikiya_which_missing.push(tables_from_json[i+missing_in_span+1].week);missing_in_span=missing_in_span+1;}
                                &_=>{mikiya_which_active.push(tables_from_json[i+missing_in_span+1].week);break;}
                            }
                            mikiya_left_at=i+missing_in_span+1;
                        }
                    }
                }
                &_ => {}
            }
            match tables_from_json[i].niclas.0.as_str() {
                "0"=>{
                    if i >= niclas_left_at{
                        let mut missing_in_span=0;
                        while let Some(is_this_zero) = tables_from_json.get(i+missing_in_span+1){
                            match is_this_zero.niclas.0.as_str(){
                                "0"=>{niclas_missing=niclas_missing+1;niclas_which_missing.push(tables_from_json[i+missing_in_span+1].week);missing_in_span=missing_in_span+1;}
                                &_=>{niclas_which_active.push(tables_from_json[i+missing_in_span+1].week);break;}
                            }
                            niclas_left_at=i+missing_in_span+1;
                        }
                    }
                }
                &_ => {}
            }
        }

        let weeks_total: Vec<usize> = (1..=(weeks_elapsed as usize)).collect();

        let marcs_positive_weeks: Vec<usize> = weeks_total.iter().filter(|x| !marc_which_missing.contains(x)).cloned().collect();
        let marcs_last_week_covered = marcs_positive_weeks.iter().map(|t| t).max().unwrap_or(&(0 as usize));

        let mikiyas_positive_weeks: Vec<usize> = weeks_total.iter().filter(|x| !mikiya_which_missing.contains(x)).cloned().collect();
        let mikiyas_last_week_covered = mikiyas_positive_weeks.iter().map(|t| t).max().unwrap_or(&(0 as usize));

        let niclas_positive_weeks: Vec<usize> = weeks_total.iter().filter(|x| !niclas_which_missing.contains(x)).cloned().collect();
        let niclas_last_week_covered = niclas_positive_weeks.iter().map(|t| t).max().unwrap_or(&(0 as usize));

        println!("{:?}", marcs_positive_weeks);
        println!("marcs last week{}",marcs_last_week_covered);
        println!("marc missing{}",marc_missing);
        println!("marc missing{:#?}",marc_which_missing);
        println!("marc active{:#?}",marc_which_active);

        let contents = fs::read_to_string("private/pending.json").expect("Error reading file");
        let mut pending_tables_from_json: Vec<PendingTable> = serde_json::from_str(&contents).expect("Error parsing JSON");
        let mut rng = rand::thread_rng();
        let key1:u64 = rng.gen_range(0..184467);
        let key2:u64 = rng.gen_range(0..184467);
        let key3:u64 = rng.gen_range(0..184467);
        //get entry(s) of specified key
        //let pending_tables_with_key: Vec<PendingTable> = tables_from_json_pending.clone().into_iter().filter(|table| table.key == key).collect();
        //pending_tables_from_json.push(pending_request);
        //let temp_date=format!("{}-{:02}-{:02}", date_today.year(), date_today.month(), date_today.day());
        for mut item in vec![key1,key2,key3]{
            while pending_tables_from_json.iter().any(|s| s.key == item) {
                item = rng.gen_range(0..184467);
            }
        }

        match marcs_last_week_covered{
            y if y == &(weeks_elapsed as usize)=>{}
            &_=>{
                if days_remaining_in_this_week<=2{
                    if let Some(last) = tables_from_json.last(){
                        let pending_request = PendingTable {
                            what: "Excuse".to_string(),
                            week: weeks_elapsed as usize,
                            requested_by: "marc".to_string(),
                            when_exactly: format!("{}-{:02}-{:02}", date_today.year(), date_today.month(), date_today.day()),
                            replacing: "marc".to_string(),
                            key: key1
                        };
                        pending_tables_from_json.push(pending_request);
                        dringlichkeitsbenachrichtigung(WhichAdmin::Marc, days_remaining_in_this_week, format!("{:#?}",marcs_positive_weeks), format!("{:#?}",marc_which_missing), format!("{:#?}",marc_which_active), last.marc.1.clone(),key1);
                    }
                }
            }
        }

        match mikiyas_last_week_covered{
            y if y == &(weeks_elapsed as usize)=>{}
            &_=>{
                if days_remaining_in_this_week<=2{
                    if let Some(last) = tables_from_json.last(){
                        let pending_request = PendingTable {
                            what: "Excuse".to_string(),
                            week: weeks_elapsed as usize,
                            requested_by: "mikiya".to_string(),
                            when_exactly: format!("{}-{:02}-{:02}", date_today.year(), date_today.month(), date_today.day()),
                            replacing: "mikiya".to_string(),
                            key: key2
                        };
                        pending_tables_from_json.push(pending_request);
                        dringlichkeitsbenachrichtigung(WhichAdmin::Mikiya, days_remaining_in_this_week, format!("{:#?}",mikiyas_positive_weeks), format!("{:#?}",mikiya_which_missing), format!("{:#?}",mikiya_which_active), last.mikiya.1.clone(),key2);
                    }
                }
            }
        }

        match niclas_last_week_covered{
           y if y == &(weeks_elapsed as usize)=>{}
            &_=>{
                if days_remaining_in_this_week<=2{
                    if let Some(last) = tables_from_json.last(){
                        let pending_request = PendingTable {
                            what: "Excuse".to_string(),
                            week: weeks_elapsed as usize,
                            requested_by: "niclas".to_string(),
                            when_exactly: format!("{}-{:02}-{:02}", date_today.year(), date_today.month(), date_today.day()),
                            replacing: "niclas".to_string(),
                            key: key3
                        };
                        pending_tables_from_json.push(pending_request);
                        dringlichkeitsbenachrichtigung(WhichAdmin::Niclas, days_remaining_in_this_week, format!("{:#?}",niclas_positive_weeks), format!("{:#?}",niclas_missing), format!("{:#?}",niclas_which_active), last.niclas.1.clone(),key3);
                    }
                }
            }
        }
        let new_contents = serde_json::to_string_pretty(&pending_tables_from_json).expect("Error serializing tables");
        fs::write("private/pending.json", new_contents).expect("Error writing to file");
        
    }
}

fn dringlichkeitsbenachrichtigung(who: WhichAdmin, days_remaining:i64, covered:String, missing:String, active:String, task:String, excusekey:u64){
    //let to_address = who.email();
    let to_address = who.email();
    let subject = format!("Bitte mach innerhalb von {} Tagen  {}", days_remaining, task);
    let mailboxes: Mailboxes = to_address.parse().unwrap();
    let to_header: header::To = mailboxes.into();
    //let content = format!("Bestätige, dass {} das {} sauber gemacht hat", whodidit, what);

    let tera = match Tera::new("templates/email/*.html.tera") {
        Ok(t) => { println!("success with tera");t},
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let excusekeylink = format!("{}/Excusemon/{}",ADDRESSOFSELF,excusekey);
    let mut context = Context::new();
    context.insert("Daysremaining", &days_remaining);
    context.insert("covered",&covered);
    context.insert("missing", &missing);
    context.insert("active", &active);
    context.insert("task", &task);
    context.insert("who", who.to_str());
    context.insert("Excusekeylink", &excusekeylink);
    let mut beautiful_content="<p><b>Hello</b>, <i>world</i>! <img src=\"cid:123\"></p>".to_string();
    match tera.render("sauberpolizei.html.tera", &context) {
        Ok(result)=>{beautiful_content=result;}
        Err(error)=>{println!("{:#?}",error);}
    }
    let _email = MessageBuilder::new()
        .mailbox(to_header)
        .from("Haushaltsserver <smtpmail_censor_placeholder>".parse().unwrap())
        .subject(subject)
    //   .singlepart(SinglePart::html(mail_body))
        .multipart(MultiPart::alternative_plain_html(
        String::from("Hello, world! :)"),
        String::from(beautiful_content),))
        .unwrap();

        let creds = Credentials::new(SMTPEMAIL.to_owned(), SECRETPASSWORD.to_owned());

        // Open a remote connection to gmail
        let mailer= SmtpTransport::relay("mail.gmx.net")
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&_email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {e:?}"),
        }

}

pub fn lol(addressee: String, whodidit: String, when: String, what: String, magiclink: String) {
        //tracing_subscriber::fmt::init();
        //let to_address = "email2_censor_placeholder, mbasjp@gmail.com, email3_censor_placeholder";
        let to_address = addressee;
        let subject = format!("{} {} {}", what, when, whodidit);
        let mailboxes: Mailboxes = to_address.parse().unwrap();
        let to_header: header::To = mailboxes.into();
        let content = format!("Bestätige, dass {} das {} sauber gemacht hat", whodidit, what);

        //now we create a beautiful message
        let tera = match Tera::new("templates/email/*.html.tera") {
            Ok(t) => { println!("success with tera");t},
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        let mut context = Context::new();
        context.insert("Headline", &content);
        context.insert("magiclink",&magiclink);
        let mut beautiful_content="<p><b>Hello</b>, <i>world</i>! <img src=\"cid:123\"></p>".to_string();
        match tera.render("pendingrequest.html.tera", &context) {
            Ok(result)=>{beautiful_content=result;}
            Err(error)=>{println!("{:#?}",error);}
        }
        let _email = MessageBuilder::new()
        .mailbox(to_header)
        .from("Haushaltsserver <smtpmail_censor_placeholder>".parse().unwrap())
        .subject(subject)
    //   .singlepart(SinglePart::html(mail_body))
        .multipart(MultiPart::alternative_plain_html(
        String::from("Hello, world! :)"),
        String::from(beautiful_content),))
        .unwrap();

        let creds = Credentials::new(SMTPEMAIL.to_owned(), SECRETPASSWORD.to_owned());

        // Open a remote connection to gmail
        let mailer= SmtpTransport::relay("mail.gmx.net")
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&_email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {e:?}"),
        }
//    });
}

/* #[tokio::main]
pub async fn lol(addressee: &str, whodidit: &str, when: &str, what: &str, magiclink: &str) {
    
    tracing_subscriber::fmt::init();
    //let to_address = "email2_censor_placeholder, mbasjp@gmail.com, email3_censor_placeholder";
    let to_address = addressee;
    let subject = format!("{} {} {}", what, when, whodidit);
    let mailboxes: Mailboxes = to_address.parse().unwrap();
    let to_header: header::To = mailboxes.into();
    let content = format!("Bestätige, dass {} das {} sauber gemacht hat", whodidit, what);

    //now we create a beautiful message
    let tera = match Tera::new("templates/**/*.html.tera") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let mut context = Context::new();
    context.insert("Headline", &content);
    context.insert("magiclink",magiclink);
    let beautiful_content;
    match tera.render("pendingrequest.html.tera", &context) {
        Ok(result)=>{beautiful_content=result;}
        Err(error)=>{println!("{:#?}",error);}
    }
    let _email = MessageBuilder::new()
    .mailbox(to_header)
    .from("Haushaltsserver <smtpmail_censor_placeholder>".parse().unwrap())
    .subject(subject)
 //   .singlepart(SinglePart::html(mail_body))
    .multipart(MultiPart::alternative_plain_html(
    String::from("Hello, world! :)"),
    String::from("<p><b>Hello</b>, <i>world</i>! <img src=\"cid:123\"></p>"),))
    .unwrap();

    let creds = Credentials::new(SMTPEMAIL.to_owned(), SECRETPASSWORD.to_owned());

    // Open a remote connection to gmail
    let mailer:AsyncSmtpTransport<Tokio1Executor> = AsyncSmtpTransport::<Tokio1Executor>::relay("mail.gmx.net")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(_email).await {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
} */


/* pub fn lol() {
    tracing_subscriber::fmt::init();
    let to_address = "email2_censor_placeholder, mbasjp@gmail.com, email3_censor_placeholder";
    //"[email3_censor_placeholder](mailto:email3_censor_placeholder); [mbasjp@gmail.com](mailto:mbasjp@gmail.com); [email2_censor_placeholder](mailto:email2_censor_placeholder)";
    let mailboxes: Mailboxes = to_address.parse().unwrap();
    let to_header: header::To = mailboxes.into();
    let mail_body = MaybeString::String("Be happy!".into());

    let _email = MessageBuilder::new()
    .mailbox(to_header)
    .from("Haushaltsserver <smtpmail_censor_placeholder>".parse().unwrap())
    .subject("Happy new year")
 //   .singlepart(SinglePart::html(mail_body))
    .body(String::from("Be happy!"))
    .unwrap();

    let creds = Credentials::new(SMTPEMAIL.to_owned(), SECRETPASSWORD.to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("mail.gmx.net")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&_email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
} */



/* 
use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,message::header::ContentType,
};

extern crate imap;


// Creating basic data structure for the email
pub async fn lol() -> Result<(), Box<dyn std::error::Error>> {
    let smtp_credentials =
        Credentials::new("smtpmail_censor_placeholder".to_owned(), "".to_owned());

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("mail.gmx.net")?
        .credentials(smtp_credentials)
//        .port(587)
        .build();

    let from = "Sender <smtpmail_censor_placeholder>";
    let to = "receiver <email3_censor_placeholder>";
    let subject = "Sending email with Rust";
    let body = "<h1>This is my first email</h1>".to_string();

    if let Err(e) = send_email_smtp(&mailer, from, to, subject, body).await {
        println!("Failed to send email: {}", e);
        return Err(e.into());
    }

    Ok(())
}

// Email sending function
async fn send_email_smtp(
    mailer: &AsyncSmtpTransport<Tokio1Executor>,
    from: &str,
    to: &str,
    subject: &str,
    body: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
    .from(from.parse().unwrap())
       // .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("Be happy!"))
        .unwrap();
/*         .from(from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body.to_string())?; */
    

        if let Err(e) = mailer.send(email).await {
            println!("Failed to send email : {}", e);
            return Err(e.into());
        }
    
        println!("Email sent successfully");
        Ok(())


}
 */