pub mod models;

pub fn login(creds: models::Credentials) {
  // Authenticate...
  crate::database::get_user();
}

pub fn logout() {
  // log user out...
}

// You can create helpers within a module that are 
// not exposed to public
fn private_helper() {
  // Private helper function
}
