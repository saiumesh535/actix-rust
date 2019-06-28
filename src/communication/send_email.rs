use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::{SendableEmail, Envelope, EmailAddress, Transport, SmtpClient};
use lettre::smtp::extension::ClientId;
use lettre::smtp::ConnectionReuseParameters;


pub fn send_email(email: String) {
    let email_1 = SendableEmail::new(
        Envelope::new(
            Some(EmailAddress::new("d.sai535@gmail.com".to_string()).unwrap()),
            vec![EmailAddress::new(email).unwrap()],
        ).unwrap(),
        "id1".to_string(),
        "<p>hello</p>".to_string().into_bytes(),
    );


    // Connect to a remote server on a custom port
    let mut mailer = SmtpClient::new_simple("smtp.gmail.com").unwrap()
        // Set the name sent during EHLO/HELO, default is `localhost`
        .hello_name(ClientId::Domain("smtp.gmail.com".to_string()))
        // Add credentials for authentication
        .credentials(Credentials::new("your email".to_string(), "your password".to_string()))
        // Enable SMTPUTF8 if the server supports it
        .smtp_utf8(true)
        // Configure expected authentication mechanism
        .authentication_mechanism(Mechanism::Plain)
        // Enable connection reuse
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited).transport();


    match mailer.send(email_1) {
        Ok(_) => {
            println!("Email has sent");
        },
        Err(error) => {
            println!("error while sending email {}", error);
        }
    }
    // Explicitly close the SMTP transaction as we enabled connection reuse
    mailer.close();
}