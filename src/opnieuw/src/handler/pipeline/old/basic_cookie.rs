fn basic_cookie() {
    match req.cookie("__dcdn_passage") {
        Some(t) => {
            println!("Cookie is here");
            if t.to_string() == "__dcdn_passage=".to_owned() + &const_xxh3(&(ip.to_string() + self.domain.domain.as_str()).into_bytes()).to_string() {
                println!("Passed")
            } else { // if the cookie does not pass the fly check
                let cookie =
                    const_xxh3(&(ip.to_string() + self.domain.domain.as_str()).into_bytes());
                return Ok(resp.body(format!(r##"<script>document.cookie="__pw_loves_you={};path=/; SameSite=Lax;",document.location.reload()</script>"##, cookie)));
            }
        }
        None => {
            let cookie =
                const_xxh3(&(ip.to_string() + self.domain.domain.as_str()).into_bytes());
            return Ok(resp.body(format!(r##"<script>document.cookie="__pw_loves_you={};path=/; SameSite=Lax;",document.location.reload()</script>"##, cookie)));
        }
    }
}
