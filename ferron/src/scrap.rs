#[path = "util"]
pub mod ferron_util {
  pub mod url_sanitizer;
  pub mod url_sanitizer_old;
}

pub use ferron_util::url_sanitizer::sanitize_url;
pub use ferron_util::url_sanitizer_old::sanitize_url as sanitize_url_old;
