<script>
    import {onMount} from "svelte";
    import { toast } from "svelte-sonner";
    import { getCookie } from "$lib/utils/auth";

    import { PUBLIC_API } from "$env/static/public";
    import { Input } from "$lib/components/ui/input";

    let email;

    let code = "";

    let lastVerificationDate;

    $: code, determineEmail();

    async function determineEmail() {
            if (code?.length === 50) {
                    await verifyEmail();
            }
    }

    async function verifyEmail() {
        let body;

        try {
                console.log(`right before verifying: ${document.cookie}`)
                let token = getCookie("jwt");

            await fetch(`${PUBLIC_API}/@/verify/complete`, {
                method: "POST",
                headers: new Headers({
                    'content-type': 'application/json',
                    'Authorization': token
                }),
                body: JSON.stringify({code})
            }).then(async (rawBody) => {
                body = await rawBody.json();

                if (rawBody.status === 200) {
                        toast.success("Email verified successfully");

                        // sleep for 2 seconds
                        await new Promise(r => setTimeout(r, 2000));

                        console.log(`right before redirction: ${document.cookie}`)

                    document.location.href = "/i/dash/domains";
                } else {
                    toast.error(`Failed to verify email: ${body.message}`);
                }
            })
        } catch (err) {
            toast.error("Failed to verify email");
        }
    }

    onMount(async () => {
        let body;

        try {
                console.log(`right before getting: ${document.cookie}`)
            const token = getCookie("jwt")
                console.log(`right after getting: ${document.cookie}`)

            await fetch(`${PUBLIC_API}/@/me`, {
                method: "GET",
                headers: new Headers({'content-type': 'application/json', 'Authorization': token}),
            }).then(async (rawBody) => {
                    console.log(`right after request: ${document.cookie}`)
                body = await rawBody.json();

                if (body.message === "Email not verified") {
                        email = body.email;
                        lastVerificationDate = body.lastEmailVerificationRequest;

                        console.log(`after setting variables: ${document.cookie}`)
                } else if (body.status === 200) {
                        document.location.href = "/i/dash/domains";
                } else {
                        document.location.href = "/i/auth/login";
                }
            })
        } catch (err) {
            document.location.href = "/i/auth/login";
        }
    })

    const resendEmail = async () => {
            let token = getCookie("jwt");

            await fetch(`${PUBLIC_API}/@/verify/request`, {
                    method: "POST",
                    headers: new Headers({
                            'content-type': 'application/json',
                            'Authorization': token
                    })
            }).then(async (res) => {
                    let response = await res.json();
                    if (res.status !== 200) {
                            toast.error(response.message)
                    } else {
                            toast.success("Check your inbox for an email with the title 'Complete your Packetware registration'");

                            lastVerificationDate = Date.now();
                    }
            }).catch(async (err) => {
                    toast.error("Failed to send request");
            })
    }
</script>

<div class="flex min-h-full flex-col justify-center pb-20 lg:pb-44 sm:ax-w-md text-center">
        <div class="backdrop-blur-md p-8 lg:px-16">
        <h2 class="sm:mx-auto sm:w-full text-4xl font-bold text-slate-200">To continue, you'll need to verify your email {email}</h2>
        <p class="text-slate-300 pt-2 tracking-tight">We've sent you an email with a code to verify your email address. Enter it below:</p>
        <div class="mt-2 w-full flex justify-center">
                <Input bind:value={code} id="email" name="code" type="text" required class="block w-full rounded-md border-0 py-1.5 text-gray-200 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 sm:max-w-sm focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 bg-gray-500" />
        </div>
        <p class="text-slate-300 pt-2 tracking-tight">If you haven't received the email, check your spam folder or <button class="text-fuchsia-500" on:click={resendEmail}>click here to resend the email</button>.</p>
        <p class="text-slate-300 tracking-tight">Need help? Give us a shout: <a class="text-fuchsia-500" href="mailto:concierge@packetware.net">concierge@packetware.net</a>.</p>
        <p class="text-slate-200 pt-2 tracking-tight">Last email was sent on {new Date(lastVerificationDate).toLocaleDateString('en-us', { weekday:"long", month:"short", day:"numeric", hour:"numeric", minute:"numeric"})}</p>
        </div>
</div>