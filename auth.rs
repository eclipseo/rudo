use crate::config;
use crate::user;
use pam_client::conv_cli::Conversation; // CLI implementation of pam
use pam_client::{Context, Flag}; // pam support
use std::error::Error;

pub fn auth_pam(
    conf: &config::Config,
    userdata: user::User,
) -> Result<Context<Conversation>, Box<dyn Error>> {
    // Create the pam context
    let mut context = Context::new(
        "sudo",                           // Service name "sudo" for now
        Some(userdata.username.as_str()), // give the name of the actual user
        Conversation::new(),
    )?;

    if conf.password {
        // Authenticate the user (ask for password, 2nd-factor token, fingerprint, etc.)
        context.authenticate(Flag::DISALLOW_NULL_AUTHTOK)?;
    }

    // Validate the account (is not locked, expired, etc.)
    context.acct_mgmt(Flag::DISALLOW_NULL_AUTHTOK)?;

    // Change the user to root to have privilege access
    context.set_user(Some(conf.user.as_str()))?;

    Ok(context)
}
