<script>
    import { onMount } from "svelte";
    import { toast } from "svelte-sonner";
    import { getCookie } from "$lib/utils/auth";

    import { Button } from "$lib/components/ui/button";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";

    import { PUBLIC_API } from "$env/static/public";

    let email;
    let password;
    let loading = false;

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
                        //document.cookie =
                            //"jwt=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
                    } else {
                        document.location.href = "/i/dash";
                    }
                })
                .catch(() => {
                    //document.cookie =
                        //"jwt=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
                });
        }
    });

    async function login() {
        loading = true;

        let data = JSON.stringify({
            email,
            password,
        });

        // TODO loading effect
        await fetch(`${PUBLIC_API}/auth/login`, {
            method: "POST",
            headers: new Headers({ "content-type": "application/json" }),
            body: data,
        })
            .then(async (resp) => {
                let response = await resp.json();

                if (!response.success) {
                    toast.error("An unexpected error occurred");
                    loading = false;
                    return;
                }

                toast.success("We're glad to see you!");

                document.cookie = `jwt=Bearer ${response.token}; path=/`;

                document.location.href = "/i/dash";
            })
            .catch(() => {
                loading = false;
                toast.error("An unexpected error occurred");
            });
    }

    import logo from "$lib/assets/top-logo.png";
</script>

<div
    class="backdrop-blur-sm flex min-h-full flex-col justify-center pb-56 lg:px-8"
>
    <div class="backdrop-blur-md">
        <div class="sm:mx-auto sm:w-full sm:max-w-sm">
            <h2
                class="backdrop-blur-sm mt-10 text-center text-4xl font-bold font-ultrawide leading-9 tracking-tight text-primary"
            >
                Welcome back 👋
            </h2>
            <a
                href="/i/auth/register"
                class="leading-9 tracking-tight font-semibold text-muted-foreground duration-100 text-right"
                ><p class="pr-4 after:content-['_↗']">
                    Need an account? Register
                </p></a
            >
        </div>

        <div
            class="pt-10 sm:mx-auto sm:w-full sm:max-w-sm rounded-lg lg:backdrop-blur-lg"
        >
            <form
                class="space-y-6 backdrop-blur-sm p-4 rounded-lg"
                on:submit|preventDefault={login}
            >
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
                    <div class="flex items-center justify-between">
                        <Label for="password">Password</Label>
                        <div class="text-sm">
                            <a
                                href="/i/auth/set-password"
                                class="font-semibold text-muted-foreground hover:underline duration-150"
                                >Forgot password?</a
                            >
                        </div>
                    </div>
                    <div class="mt-2">
                        <Input
                            id="password"
                            bind:value={password}
                            name="password"
                            type="password"
                            autocomplete="current-password"
                            required
                        />
                    </div>
                </div>

                <Button type="submit" class="w-full"
                    >{#if !loading}Sign in{:else}Signing in ...{/if}</Button
                >
            </form>
        </div>
    </div>
</div>
