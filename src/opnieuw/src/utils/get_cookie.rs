use crate::models::request_context::RequestContext;

impl RequestContext {
    pub fn get_cookie(&self, name: &str) -> Option<String> {
        let cookie = self.req.headers.get("cookie")?;

        let cookie = cookie.to_str().ok()?;

        let cookie = cookie.split(';').find(|c| c.contains(name))?;

        let cookie = cookie.split('=').skip(1).collect::<Vec<&str>>().join("=");

        Some(cookie)
    }

    // get all cookies
    pub fn get_cookies(&self) -> Option<Vec<Vec<&str>>> {
        let cookie = self.req.headers.get("cookie")?;

        let cookie = cookie.to_str().ok()?;

        // get all cookies with names Vec<(String, String)>
        let cookie_names = cookie
            .split(';')
            .map(|c| c.split('=').collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();

        Some(cookie_names)
    }
}
