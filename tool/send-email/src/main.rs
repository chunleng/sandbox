use clap::Parser;
use lettre::{message::header::ContentType, Message, SmtpTransport, Transport};
use std::error::Error;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(num_args = 1)]
    conn_string: String,

    #[arg(short, long)]
    from: String,
    #[arg(short, long, default_value = "limchunleng+test@gmail.com")]
    to: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::try_parse();
    match args {
        Ok(args) => {
            println!(
                "Attempting to send email to {}, using {}",
                args.to, args.from
            );
            send_test_mail(args.from, args.to, args.conn_string)?;
            println!("Email sent successfully");
        }
        Err(msg) => {
            msg.print()?;
        }
    }
    Ok(())
}

fn send_test_mail(
    send_from: String,
    send_to: String,
    conn_string: String,
) -> Result<(), Box<dyn Error>> {
    let email = Message::builder()
        .from(send_from.parse().unwrap())
        .to(send_to.parse().unwrap())
        .subject(format!("Message sent from {}", send_from))
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(
            "A test email sent from 'send_test_mail'. Built by Tosh Lim",
        ))
        .unwrap();

    let mailer = SmtpTransport::from_url(&conn_string).unwrap().build();
    println!("{}", conn_string);

    mailer.send(&email)?;
    Ok(())
}
