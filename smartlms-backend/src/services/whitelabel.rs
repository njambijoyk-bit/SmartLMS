// White-label service - CSS injection, custom domains, branding
use crate::tenant::InstitutionConfig;
use serde::{Deserialize, Serialize};

/// White-label settings for an institution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhiteLabelConfig {
    pub logo_url: Option<String>,
    pub favicon_url: Option<String>,
    pub primary_color: String,
    pub secondary_color: String,
    pub accent_color: String,
    pub background_color: String,
    pub text_color: String,
    pub font_family: Option<String>,
    pub custom_css: Option<String>,
    pub custom_js: Option<String>,
}

impl Default for WhiteLabelConfig {
    fn default() -> Self {
        Self {
            logo_url: None,
            favicon_url: None,
            primary_color: "#3b82f6".to_string(),
            secondary_color: "#1e40af".to_string(),
            accent_color: "#10b981".to_string(),
            background_color: "#ffffff".to_string(),
            text_color: "#1f2937".to_string(),
            font_family: None,
            custom_css: None,
            custom_js: None,
        }
    }
}

impl From<&InstitutionConfig> for WhiteLabelConfig {
    fn from(config: &InstitutionConfig) -> Self {
        Self {
            logo_url: config.logo_url.clone(),
            favicon_url: None,
            primary_color: config.primary_color.clone(),
            secondary_color: config.secondary_color.clone(),
            accent_color: "#10b981".to_string(),
            background_color: "#ffffff".to_string(),
            text_color: "#1f2937".to_string(),
            font_family: None,
            custom_css: None,
            custom_js: None,
        }
    }
}

/// Generate CSS variables from white-label config
pub fn generate_css_variables(config: &WhiteLabelConfig) -> String {
    let mut css = String::from(":root {\n");

    css.push_str(&format!("  --primary: {};\n", config.primary_color));
    css.push_str(&format!("  --secondary: {};\n", config.secondary_color));
    css.push_str(&format!("  --accent: {};\n", config.accent_color));
    css.push_str(&format!("  --bg: {};\n", config.background_color));
    css.push_str(&format!("  --text: {};\n", config.text_color));

    if let Some(font) = &config.font_family {
        css.push_str(&format!("  --font-family: {};\n", font));
    }

    css.push_str("}\n");
    css
}

/// Generate complete CSS including custom CSS
pub fn generate_full_css(config: &WhiteLabelConfig) -> String {
    let mut css = generate_css_variables(config);

    // Base styles
    css.push_str("\n/* Base styles */\n");
    css.push_str("body { font-family: var(--font-family, system-ui); }\n");
    css.push_str(".btn-primary { background: var(--primary); }\n");
    css.push_str(".btn-secondary { background: var(--secondary); }\n");

    // Custom CSS
    if let Some(custom) = &config.custom_css {
        css.push_str("\n/* Custom CSS */\n");
        css.push_str(custom);
    }

    css
}

/// Validate custom domain
pub fn validate_domain(domain: &str) -> Result<(), String> {
    // Basic domain validation
    if domain.len() < 3 || domain.len() > 253 {
        return Err("Invalid domain length".to_string());
    }

    // Check for valid characters
    if !domain
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '.')
    {
        return Err("Invalid domain characters".to_string());
    }

    // Must have at least one dot
    if !domain.contains('.') {
        return Err("Domain must have TLD".to_string());
    }

    Ok(())
}

/// Email template with white-label branding
pub mod email {
    use super::WhiteLabelConfig;

    /// Generate branded email header
    pub fn generate_header(config: &WhiteLabelConfig) -> String {
        let mut html = String::new();

        if let Some(logo) = &config.logo_url {
            html.push_str(&format!(
                "<img src=\"{}\" alt=\"Logo\" style=\"max-height: 60px;\">",
                logo
            ));
        } else {
            html.push_str(&format!(
                "<h1 style=\"color: {};\">SmartLMS</h1>",
                config.primary_color
            ));
        }

        html
    }

    /// Generate branded email footer
    pub fn generate_footer(config: &WhiteLabelConfig, institution_name: &str) -> String {
        format!(
            "<p style=\"color: #6b7280; font-size: 12px;\">\n\
             &copy; {} {}. All rights reserved.<br>\n\
             This email was sent from {} LMS.\n\
             </p>",
            chrono::Utc::now().format("%Y"),
            institution_name,
            institution_name
        )
    }
}

/// Custom domain management
pub mod domain {
    use serde::{Deserialize, Serialize};

    /// Custom domain status
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum DomainStatus {
        Pending,   // DNS not yet configured
        Verifying, // Checking DNS records
        Active,    // Domain working
        Failed,    // Verification failed
    }

    /// Custom domain record
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CustomDomain {
        pub domain: String,
        pub status: DomainStatus,
        pub verified_at: Option<chrono::DateTime<chrono::Utc>>,
        pub ssl_enabled: bool,
        pub ssl_issuer: Option<String>,
    }

    /// DNS verification record
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DnsVerification {
        pub record_type: String,
        pub name: String,
        pub value: String,
        pub expected: String,
    }

    /// Generate DNS TXT record for domain verification
    pub fn generate_verification_txt(subdomain: &str, institution_id: &str) -> DnsVerification {
        DnsVerification {
            record_type: "TXT".to_string(),
            name: subdomain.to_string(),
            value: format!("smartlms-verification={}", institution_id),
            expected: format!("smartlms-verification={}", institution_id),
        }
    }

    /// Generate CNAME record for custom domain
    pub fn generate_cname(subdomain: &str) -> DnsVerification {
        DnsVerification {
            record_type: "CNAME".to_string(),
            name: subdomain.to_string(),
            value: format!("{}.smartlms.io", subdomain),
            expected: format!("{}.smartlms.io", subdomain),
        }
    }
}
