#[derive(Debug, serde::Deserialize)]
pub struct GoogleOAuthProfile {
    pub email: String,
    pub family_name: String,
    pub given_name: String,
    pub id: String,
    pub locale: String,
    pub name: String,
    pub picture: String,
}

#[derive(Debug)]
pub struct OAuthProfile {
    pub email: String,
    pub id: String,
    pub name: String,
    pub avatar_url: String,
}

impl From<GoogleOAuthProfile> for OAuthProfile {
    fn from(google_profile: GoogleOAuthProfile) -> Self {
        OAuthProfile {
            email: google_profile.email,
            id: google_profile.id,
            name: google_profile.name,
            avatar_url: google_profile.picture,
        }
    }
}
