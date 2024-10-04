<script>
    import APIClient from "$lib/utils/api";
    import { toast } from "svelte-sonner";

    import { Button } from "$lib/components/ui/button";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";

    let loading = false;

    /** @param {{ currentTarget: EventTarget & HTMLFormElement}} event */
    async function forgotPassword(event) {
        loading = true;

        const data = new FormData(event.currentTarget);
        const email = data.get("email");

        let body = {
            email,
        };

        try {
            let res = await APIClient.post("/auth/reset_password_send", body);
            console.log(res);

            toast.success("Reset password email has been sent.");
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
