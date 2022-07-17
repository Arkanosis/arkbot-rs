use reqwest::{
    Error,
    blocking::{
        Client,
        Response,
    },
};

use serde::ser::Serialize;

use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Token {
    logintoken: Option<String>,
    csrftoken: Option<String>,
    patroltoken: Option<String>,
}

#[derive(Deserialize)]
struct TokenQuery {
    tokens: Option<Token>,
}

#[derive(Deserialize)]
struct TokenResponse {
    query: Option<TokenQuery>,
}

#[derive(Deserialize)]
struct Login {
    result: Option<String>,
}

#[derive(Deserialize)]
struct LoginResponse {
    login: Option<Login>,
}

#[derive(Deserialize)]
struct Edit {
    result: Option<String>,
}

#[derive(Deserialize)]
struct EditResponse {
    login: Option<Edit>,
}

pub struct Bot {
    client: Client,
    api_url: String,
}

impl Bot {
    pub fn new(server_url: &str, script_path: &str) -> Self {
        Bot {
            client: Client::builder()
                .user_agent(format!("arkbot/{}", crate::version()))
                .cookie_store(true)
                .build()
                .unwrap(),
            api_url: format!("{}{}/api.php", server_url, script_path),
        }
    }

    fn post<T: Serialize + ?Sized>(&self, form: &T) -> Result<Response, Error> {
        self.client.post(&self.api_url)
            .form(form)
            .send()
    }

    fn get_token(&self, token_type: &str) -> Result<String, ()> {
        let response = self.post(&[
            ("action", "query"),
            ("type", &token_type),
            ("meta", "tokens"),
            ("format", "json"),
        ]);

        if let Ok(response) = response {
            let response = response.json::<TokenResponse>();
            if let Ok(response) = response {
                if let Some(query) = response.query {
                    if let Some(tokens) = query.tokens {
                        if let Some(login_token) = tokens.logintoken {
                            return Ok(login_token)
                        } else if let Some(csrf_token) = tokens.csrftoken {
                            return Ok(csrf_token)
                        } else if let Some(patrol_token) = tokens.patroltoken {
                            return Ok(patrol_token)
                        }
                    }
                }
            }
        }
        return Err(())
    }

    pub fn login(&mut self, login: &str, password: &str) -> bool {
        if let Ok(token) = self.get_token("login") {
            let response = self.post(&[
                ("action", "login"),
                ("lgname", &login),
                ("lgpassword", &password),
                ("lgtoken", &token),
                ("format", "json"),
            ]);

            if let Ok(response) = response {
                let response = response.json::<LoginResponse>();
                if let Ok(response) = response {
                    if let Some(login) = response.login {
                        if let Some(result) = login.result {
                            return result == "Success";
                        }
                    }
                }
            }
        }
        return false;

    }

    pub fn edit_page(&self, title: &str, summary: &str, text: &str) -> bool {
        if let Ok(token) = self.get_token("csrf") {
            println!("editing '{}' with content '{}' and summary '{}'", title, text, summary);
            let response = self.post(&[
                ("action", "edit"),
                ("title", &title),
                ("summary", &summary),
                ("text", &text),
                ("token", &token),
                ("bot", "true"),
                ("assert", "user"),
                ("format", "json"),
            ]);

            if let Ok(response) = response {
                let response = response.json::<EditResponse>();
                if let Ok(response) = response {
                    if let Some(login) = response.login {
                        if let Some(result) = login.result {
                            return result == "Success";
                        }
                    }
                }
            }
        }
        return true;
    }
}
