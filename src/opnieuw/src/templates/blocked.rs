use crate::{
    models::request_context::RequestContext,
    utils::resp::resp,
    HttpResponse,
    GA,
};
use hyper::StatusCode;

impl RequestContext {
    pub fn blocked(&self) -> HttpResponse {
        self.domain.analytic.blocked_reqs.inc();
        GA.template.blocked.inc();
        resp(
            r##"
<!DOCTYPE html>
<html lang="en" class="bg-gray-700">
<head>
    <meta charset="UTF-8">
    <title>blocked by page rule</title>
    <script src="https://cdn.tailwindcss.com"></script>
             <link rel="stylesheet"
             href="https://fonts.googleapis.com/css?family=Krona+One">
                 </head>
                 <style>

                body {
                 font-family: 'Krona One', serif;
                }

                 </style>
                 <body class="bg-gradient-to-br from-blue-500 via-gray-900 to-fuchsia-500">
                <div class="bg-no-repeat bg-cover bg-right w-screen bg-origin-content">
                <div class="backdrop-blur-sm bg-black bg-opacity-40">
                <div class="flex flex-col items-center justify-center min-h-screen py-2 text-slate-200 text-center backdrop-blur-sm">
                <div class="flex flex-col items-center justify-center px-32 lg:px-64">
                <h2 class="text-xl font-bold text-slate-400 bod">hello there, intrepid digital explorer. it seems you've stumbled upon the virtual equivalent of a "No Entry" sign ...</h2>
                 <h1 class="text-6xl font-bold text-slate-50 balls">Access Denied</h1>
                 <!-- <h2 class="text-5xl balls text-slate-100">(code 403)</h2> -->
        <h1 class="text-4xl text-slate-200 balls">We can't offer you entry to this Web property right now (code 403). 🚫</h1>
            <h2 class="text-3xl text-slate-300 bod">We're sorry to be so upfront with you, but we're under top secret orders from the content owner to restrict this page.<br>
            <br><a>While you're here, why did the website go bankrupt? Lost all their cache! Okay, we're working on our joke skills.
            Remember, even in this digital "No Trespassing" zone, laughter still debugs the soul.</a></h2>
            </div>
            </div>
            </div>
            <div class="sticky bottom-0 border-amber-500 border-dotted text-center text-slate-300 bod">
            <p class="py-4">friendly content delivery by <a href="https://packetware.net" class="underline decoration-fuchsia-500">packetware</a>, a Whole Lotta Heart, Corp. company</p>
            </div>
            </div>
            </body>
            </html>
        "##,
            Some(StatusCode::FORBIDDEN),
            false,
        )
    }
}
