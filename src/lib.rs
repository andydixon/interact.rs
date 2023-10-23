pub mod sms {
    use chrono::prelude::{DateTime, Utc};
    use chrono::Duration;
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Phonenumber {
        phone: Vec<String>,
    }

    #[derive(Serialize)]
    struct Payload {
        message_body: String,
        #[serde(rename = "from")]
        from_field: String,
        send_at: String,
        valid_until: String,
        #[serde(rename = "to")]
        to_field: Vec<Phonenumber>,
    }

    #[derive(Debug, Clone)]
    pub struct InteractSMS {
        api_endpoint: String,
        apikey: String,
        body: String,
        from: String,
        to: Vec<String>,
        schedule_time: Option<DateTime<Utc>>,
        valid_until: Option<DateTime<Utc>>,
    }

    pub struct InteractResponse {
        pub status: u16,
        pub response_body: String,
    }

    #[derive(Debug)]
    pub enum InteractError {
        Error { message: String,data: Option <String> },
    }

    impl InteractError {
        pub fn get_data(&self) -> Option<String> {
            match self {
                InteractError::Error { message:_,data } => {return data.clone();},
            }
        }
    }

    impl std::error::Error for InteractError {}

    impl std::fmt::Display for InteractError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                InteractError::Error { message,data:_ } => write!(f, "{}.", message),
            }
        }
    }


    impl InteractSMS {
        pub fn add_recipient(mut self, recipient: String) -> InteractSMS {
            self.to.push(recipient.to_string());
            self
        }

        pub fn message(mut self, message_body: String) -> InteractSMS {
            self.body = message_body;
            self
        }

        pub fn set_originator(mut self, sender_name: String) -> InteractSMS {
            self.from = sender_name;
            self
        }

        pub fn expires(mut self, expiry_timestamp: DateTime<Utc>) -> InteractSMS {
            self.valid_until = Some(expiry_timestamp);
            self
        }

        pub fn send_at(mut self, send_timestamp: DateTime<Utc>) -> InteractSMS {
            self.schedule_time = Some(send_timestamp);
            self
        }

        pub fn send_sms(&self) -> Result<InteractResponse, InteractError> {
            let schedule_time = match self.schedule_time {
                Some(a) => a,
                None => Utc::now() + Duration::minutes(5),
            };

            let validity = match self.valid_until {
                Some(a) => a,
                None => Utc::now() + Duration::days(2),
            };

            let mut recipients: Vec<Phonenumber> = Vec::new();

            recipients.push(Phonenumber {
                phone: self.to.clone(),
            });

            let schedule_time = schedule_time.format("%Y-%m-%dT%H:%M:%SZ");
            let validity = validity.format("%Y-%m-%dT%H:%M:%SZ");

            let data = Payload {
                message_body: self.body.clone(),
                from_field: self.from.clone(),
                to_field: recipients,
                send_at: schedule_time.to_string(),
                valid_until: validity.to_string(),
            };

            let payload = serde_json::to_string(&data).expect("Failed to serialize payload");
            let client = reqwest::blocking::Client::new();

            let res = client
                .post(self.api_endpoint.clone())
                .header("accept", "application/json")
                .header("Content-Type", "application/json")
                .header("X-auth-key", self.apikey.clone())
                 .header("Content-Length", &payload.len().to_string())
                 .header("User-Agent", "Rust/interact.rs".to_string())
                .body(payload);
               let res = res.send();

            match res {
                Ok(resp) => {
                    let mut response = InteractResponse {
                        status: 0,
                        response_body: "".to_string(),
                    };
                    let success=resp.status().is_success();
                    let status_code = resp.status().clone();
                    if success {
                        response.status = status_code.as_u16();
                    }

                    response.response_body = match resp.text() {
                        Ok(t) => t,
                        Err(_) => {
                            if !success {
                                return Err(InteractError::Error {
                                    message: "API call was unsuccessful, and unable to parse response body"
                                        .to_string(),
                                    data: None
                                });
                            } else {
                                return Err(InteractError::Error {
                                    message: "API call successful, but unable to parse response body"
                                        .to_string(),
                                    data: None
                                });
                            }
                        }
                    };
                    if !success {
                        return Err(InteractError::Error {
                            message: format!("API returned error {}", status_code),
                            data: Some(response.response_body.to_string())
                        });
                    }

                    return Ok(response);
                }
                Err(err) => {
                    return Err(InteractError::Error {
                        message: format!("Fatal Error: {}", err),
                        data: None
                    });
                }
            };
        }
    }

    pub fn sms_api(api_key: String) -> InteractSMS {
        InteractSMS {
            api_endpoint: "https://api.webexinteract.com/v1/sms".to_string(),
            apikey: api_key,
            body: "".to_string(),
            from: "".to_string(),
            to: vec![],
            schedule_time: None,
            valid_until: None,
        }
    }
}
