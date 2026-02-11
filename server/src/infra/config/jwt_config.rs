use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TokenData {
    pub secret: String,
    pub expiration_time: u64,
}

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub access_token: TokenData,
    pub refresh_token: TokenData,
}

impl<'de> Deserialize<'de> for JwtConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Clone, Deserialize)]
        struct TokenDataHelper {
            secret: Option<String>,
            expiration_time: Option<u64>,
        }

        #[derive(Clone, Deserialize)]
        struct JwtConfigHelper {
            access_token: Option<TokenDataHelper>,
            refresh_token: Option<TokenDataHelper>,
        }

        let helper = JwtConfigHelper::deserialize(deserializer)?;
        let default_access_token = TokenData {
            secret: "default_access_token_secret".to_string(),
            expiration_time: 3_600, // 1 hour
        };
        let default_refresh_token = TokenData {
            secret: "default_refresh_token_secret".to_string(),
            expiration_time: 2_592_000, // 1 month
        };

        Ok(JwtConfig {
            access_token: TokenData {
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
            refresh_token: TokenData {
                secret: helper
                    .refresh_token
                    .clone()
                    .unwrap()
                    .secret
                    .unwrap_or(default_refresh_token.secret),
                expiration_time: helper
                    .refresh_token
                    .unwrap()
                    .expiration_time
                    .unwrap_or(default_refresh_token.expiration_time),
            },
        })
    }
}

impl Default for JwtConfig {
    fn default() -> Self {
        JwtConfig {
            access_token: TokenData {
                secret: "default_access_token_secret".to_string(),
                expiration_time: 3_600, // 1 hour
            },
            refresh_token: TokenData {
                secret: "default_refresh_token_secret".to_string(),
                expiration_time: 2_592_000, // 1 month
            },
        }
    }
}
