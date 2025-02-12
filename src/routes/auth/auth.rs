use reqwest::header::AUTHORIZATION;

pub async fn login_google(token: String) -> Result<User, Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://www.googleapis.com/oauth2/v1/userinfo")
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await?;

    if response.status().is_success() {
        let user_info: GoogleUserInfo = response.json().await?;

        // Проверяем, есть ли пользователь с таким Google ID в базе данных
        let user = db::find_user_by_google_id(&user_info.id).await?;

        if let Some(user) = user {
            Ok(user)
        } else {
            // Создаем нового пользователя в базе данных
            let new_user = User {
                google_id: Some(user_info.id),
                // ... другие поля
            };
            let new_user = db::create_user(new_user).await?;
            Ok(new_user)
        }
    } else {
        Err(Error::from(actix_web::error::ErrorUnauthorized().into()))
    }
}