<script>
    import APIClient from "$lib/utils/api";
    import { toast } from "svelte-sonner";

    import { Button } from "$lib/components/ui/button";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";

    let loading = false;

    /** @param {{ currentTarget: EventTarget & HTMLFormElement}} event */
    async function login(event) {
        loading = true;

        const data = new FormData(event.currentTarget);
        const email = data.get("email");
        const password = data.get("password");

        let body = {
            email,
            password,
        };

        try {
            let res = await APIClient.post("/auth/login", body);
            console.log(res);

            toast.success("User successfully logged in.");

            // Set JWT token in a cookie
            document.cookie = `jwt=${res.data.data.token}; path=/;`;
            document.location = "/i/dash";
        } catch (err) {
            console.log(err);
            toast.error(err.response.data.data.error.message);
            loading = false;
        }
    }
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
                                href="/i/auth/reset-password"
                                class="font-semibold text-muted-foreground hover:underline duration-150"
                                >Forgot password?</a
                            >
                        </div>
                    </div>
                    <div class="mt-2">
                        <Input
                            id="password"
                            name="password"
                            type="password"
                            autocomplete="current-password"
                            required
                        />
                    </div>
                </div>

                <Button type="submit" class="w-full" disabled={loading}
                    >{#if !loading}Sign in{:else}Signing in ...{/if}</Button
                >
            </form>
        </div>
    </div>
</div>
