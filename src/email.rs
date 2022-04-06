use lettre::{SmtpTransport, Message, Transport, message::Mailbox};
use lettre::transport::smtp::authentication::Credentials;
use crate::models::Applicant;
use std::env;

pub enum ApplicationStatus {
    Accepted,
    Denied
}

impl std::string::ToString for ApplicationStatus {
    fn to_string(&self) -> String {
        match self {
            ApplicationStatus::Accepted => "Accepted".to_string(),
            ApplicationStatus::Denied => "Denied".to_string()
        }
    }
}

pub fn send_email_to_applicant(applicant: Applicant, application_status: ApplicationStatus)
    -> anyhow::Result<()> {
    let smtp_username = env::var("SMTP_USER")?;
    let smtp_password = env::var("SMTP_PASS")?;

    let admissions_mailbox: Mailbox = format!("Admissions Department <{}", smtp_username).parse()?;
    let applicant_mailbox: Mailbox = format!("{} <{}>", applicant.name, applicant.email).parse()?;

    let email = Message::builder()
        .from(admissions_mailbox)
        .to(applicant_mailbox)
        .subject("Change in Application Status")
        .body(String::from(format!("Your application status has been changed to: {}", application_status.to_string())))?;

    let credentials = Credentials::new(smtp_username, smtp_password);

    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(credentials).build();

    mailer.send(&email)?;

    Ok(())

}