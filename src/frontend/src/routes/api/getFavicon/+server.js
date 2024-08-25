// src/routes/api/+server.js
import fetch from 'node-fetch';
import cheerio from 'cheerio';
import {json} from "@sveltejs/kit";

export async function POST({ request }) {
    const url = (await request.json()).url;

    try {
        // fetch with a timeout
        const controller = new AbortController()

// 5 second timeout:

        const timeoutId = setTimeout(() => controller.abort(), 2000)

        const response = await fetch(url, { signal: controller.signal });

        const html = await response.text();
        const $ = cheerio.load(html);
        let favicon = $("link[rel='shortcut icon']").attr('href') ||
            $("link[rel='icon']").attr('href');

        // Resolve relative URLs
        if (favicon && !favicon.includes('://')) {
            if (favicon.startsWith('/')) {
                const urlObj = new URL(url);
                favicon = urlObj.origin + favicon;
            } else {
                favicon = new URL(favicon, url).href;
            }
        }

        if (!favicon) {
            favicon = new URL("favicon.ico", url).href;
        }

        return json({
            success: true,
            data: favicon
        })
    } catch (error) {
        return json({
            success: false,
            message: "mogged by the site"
        });
    }
}
