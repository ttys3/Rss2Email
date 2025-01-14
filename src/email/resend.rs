//! [`EmailProvider`] implementation using [`Resend`](https://resend.com/).

use resend_rs::{mail::Mail, resend_client::ResendClient};

use crate::info;

use super::{email_provider::EmailProvider, error::EmailError, EnvLoader};

#[derive(Default, Debug)]
pub struct Resend {
  api_key: Option<String>,
}

impl Resend {
  pub(crate) fn new(env_loader: &EnvLoader) -> Self {
    Self {
      api_key: env_loader.api_key.clone(),
    }
  }
}

impl EmailProvider for Resend {
  fn send_email(&self, address: &str, contents: &str) -> Result<(), EmailError> {
    let api_key = self
      .api_key
      .as_ref()
      .ok_or_else(|| EmailError::Config("Cannot use Resend without API_KEY".to_owned()))
      .cloned()?;

    let mail = Mail::new("rss2email@resend.dev", address, "rss2email", contents);
    let client = ResendClient::new(api_key);

    match client.send(mail) {
      Ok(()) => {
        info!("Email request sent");
        Ok(())
      }
      Err(e) => Err(EmailError::from(e)),
    }
  }
}
