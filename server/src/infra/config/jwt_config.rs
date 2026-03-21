use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AccessTokenData {
    pub secret: String,
    pub expiration_time: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RefreshTokenData {
    pub secret: String,
    pub expiration_time: u64,
    pub salt: String,
}

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub access_token: AccessTokenData,
    pub refresh_token: RefreshTokenData,
}

impl<'de> Deserialize<'de> for JwtConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Clone, Deserialize)]
        struct AccessTokenDataHelper {
            secret: Option<String>,
            expiration_time: Option<u64>,
        }

        #[derive(Clone, Deserialize)]
        struct RefreshTokenDataHelper {
            secret: Option<String>,
            expiration_time: Option<u64>,
            salt: Option<String>,
        }

        #[derive(Clone, Deserialize)]
        struct JwtConfigHelper {
            access_token: Option<AccessTokenDataHelper>,
            refresh_token: Option<RefreshTokenDataHelper>,
        }

        let helper = JwtConfigHelper::deserialize(deserializer)?;
        let default_access_token = AccessTokenData {
            secret: "default_access_token_secret".to_string(),
            expiration_time: 3_600, // 1 hour
        };
        let default_refresh_token = RefreshTokenData {
            secret: "default_refresh_token_secret".to_string(),
            expiration_time: 2_592_000, // 1 month
            salt: "default_refresh_token_salt".to_string(),
        };

        Ok(JwtConfig {
            access_token: AccessTokenData {
                secret: helper
                    .access_token
                    .clone()
                    .unwrap()
                    .secret
                    .unwrap_or(default_access_token.secret),
                expiration_time: helper
                    .access_token
                    .unwrap()
                    .expiration_time
                    .unwrap_or(default_access_token.expiration_time),
            },
            refresh_token: RefreshTokenData {
                secret: helper
                    .refresh_token
                    .clone()
                    .unwrap()
                    .secret
                    .unwrap_or(default_refresh_token.secret),
                expiration_time: helper
                    .refresh_token
                    .clone()
                    .unwrap()
                    .expiration_time
                    .unwrap_or(default_refresh_token.expiration_time),
                salt: helper
                    .refresh_token
                    .unwrap()
                    .salt
                    .unwrap_or(default_refresh_token.salt),
            },
        })
    }
}

impl Default for JwtConfig {
    fn default() -> Self {
        JwtConfig {
            access_token: AccessTokenData {
                secret: "default_access_token_secret".to_string(),
                expiration_time: 3_600, // 1 hour
            },
            refresh_token: RefreshTokenData {
                secret: "default_refresh_token_secret".to_string(),
                expiration_time: 2_592_000, // 1 month
                salt: "default_refresh_token_salt".to_string(),
            },
        }
    }
}
