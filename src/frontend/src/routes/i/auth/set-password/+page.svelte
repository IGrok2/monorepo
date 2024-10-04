<script>
    import { onMount } from "svelte";
    import { toast } from "svelte-sonner";
    import { getCookie } from "$lib/utils/auth";

    import { Button } from "$lib/components/ui/button";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";

    import { PUBLIC_API } from "$env/static/public";

    let email;
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

    async function forgotPassword() {
        loading = true;

        let data = JSON.stringify({
            email,
        });

        // TODO loading effect
        await fetch(`${PUBLIC_API}/auth/set-password`, {
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

                document.location.href = "/i/auth/login";
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
                Forgot Password
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
                on:submit|preventDefault={forgotPassword}
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

                <Button
                    type="submit"
                    class="w-full"
                    >{#if !loading}Submit{:else}Sending Email ...{/if}</Button
                >
            </form>
        </div>
    </div>
</div>
