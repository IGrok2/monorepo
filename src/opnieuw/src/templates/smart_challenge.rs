use crate::{
    HttpResponse,
    BACKGROUND_CHALLENGE,
    CHALLENGE_KEYS,
    GA,
    SMART_CHALLENGE,
};
use hyper::StatusCode;
use std::ops::Deref;

use crate::models::request_context::RequestContext;

use crate::utils::resp::resp;

impl RequestContext {
    pub fn smart_challenge(&self) -> HttpResponse {
        self.domain.analytic.smart_challenged_reqs.inc();
        GA.template.smart_challenge.inc();

        use std::ops::DerefMut;
        if let Some(ref mut t) = self.by_example.borrow_mut().deref_mut() {
            t.human_engine.smart_challenge_served = true;
        }

        /*
               let keys: Vec<u8> = CHALLENGE_KEYS.read().unwrap().deref().to_vec();

               println!("serving challenge with key: {:?}", keys);

        */
        let smart = SMART_CHALLENGE.read().unwrap();

        resp(
            &(r##"
<!DOCTYPE html>
<html lang="en">
<style>
    body {
        text-align: center;
        padding-top: 10px;
        padding-left: 10px;
        padding-right: 10px;
    }

    select-all-text {
        user-select: all;
        -moz-user-select: all;
        -webkit-user-select: all;
    }
</style>
<head>
    <meta charset="UTF-8">
<body class="text-align: center;">
<font face = "Verdana" size = "2">
    <p id="info">You must have JavaScript enabled to use access this website.</p>
</font>
</body>
<script>
"##
            .to_string()
                + smart.deref().as_str()
                + r##"
</script>
</head>
</html>
        "##),
            Some(StatusCode::UNAUTHORIZED),
            false,
        )
    }
}
