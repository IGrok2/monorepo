<script>
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import { toast } from "svelte-sonner";
    import { getCookie } from "$lib/utils/auth";

    import { Button } from "$lib/components/ui/button";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";

    import { PUBLIC_API } from "$env/static/public";

    let email = "";
    $page.url.searchParams.get("email");
    let password;
    let confirmPassword;
    let loading = false;

    const code = $page.params.code;
    email = $page.url.searchParams.get("email");
    $: console.log(email);

    $: samePassword = password === confirmPassword;

    onMount(async () => {
        // does the jwt cookie exist?
        if (
            document.cookie
                .split(";")
                .some((item) => item.trim().startsWith("jwt="))
        ) {
            // before redirecting, make sure the cookie is valid by fetching a GET to /@/me
            const token = getCookie("jwt");

            await fetch(`${PUBLIC_API}/@/me`, {
                method: "GET",
                headers: new Headers({
                    "content-type": "application/json",
                    Authorization: token,
                }),
            })
                .then(async (resp) => {
                    let response = await resp.json();

                    if (resp.status !== 200 || !response.success) {
                        // remove the cookie
                        document.cookie =
                            "jwt=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
                    } else {
                        document.location.href = "/i/dash/domains";
                    }
                })
                .catch(() => {
                    document.cookie =
                        "jwt=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
                });
        }
    });

    async function login() {
        loading = true;

        let data = JSON.stringify({
            email,
            code,
            password,
        });

        // TODO loading effect
        await fetch(`${PUBLIC_API}/auth/set-password/reset`, {
            method: "POST",
            headers: new Headers({ "content-type": "application/json" }),
            body: data,
        })
            .then(async (resp) => {
                let response = await resp.json();
                console.log(response);

                if (!response.success) {
                    toast.error("An unexpected error occurred");
                    loading = false;
                    return;
                }

                toast.success("Password set. You may login now.");

                document.location.href = "/i/auth/login";
            })
            .catch(() => {
                loading = false;
                toast.error("An unexpected error occurred");
            });
    }

    import logo from "$lib/assets/top-logo.png";
</script>

<div class="flex min-h-full flex-col justify-center pb-56 lg:px-8">
    <div class="sm:mx-auto sm:w-full sm:max-w-sm">
        <h2
            class="mt-10 text-center text-4xl font-bold font-ultrawide leading-9 tracking-tight text-primary"
        >
            Change Password
        </h2>
    </div>

    <div class="pt-10 sm:mx-auto sm:w-full sm:max-w-sm rounded-lg">
        <form class="space-y-6 p-4 rounded-lg" on:submit|preventDefault={login}>
            <div>
                <Label for="email">Email address</Label>
                <div class="mt-2">
                    <Input
                        bind:value={email}
                        id="email"
                        name="email"
                        type="email"
                        autocomplete="email"
                        required
                    />
                </div>
            </div>

            <div>
                <Label for="password">New Password</Label>

                <div class="mt-2">
                    <Input
                        id="password"
                        bind:value={password}
                        name="password"
                        type="password"
                        required
                    />
                </div>
            </div>
            <div>
                <Label
                    for="confirm-password"
                    >Confirm Password</Label
                >

                <div class="mt-2">
                    <Input
                        id="confirm-password"
                        bind:value={confirmPassword}
                        name="confirm-password"
                        type="password"
                        required
                    />
                </div>
            </div>

            <div>
                {#if !loading && samePassword}
                    <Button type="submit" class="w-full">Change</Button>
                {:else if !loading && !samePassword}
                    <Button class="w-full">Passwords Dont Match</Button>
                {:else}
                    <Button class="w-full">Changing Password ...</Button>
                {/if}
            </div>
        </form>
    </div>
</div>
