<script>
    import { page } from "$app/stores";

    import APIClient from "$lib/utils/api";
    import { toast } from "svelte-sonner";

    import { Button } from "$lib/components/ui/button";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";

    let loading = false;

    console.log($page);

    /** @param {{ currentTarget: EventTarget & HTMLFormElement}} event */
    async function setPassword(event) {
        loading = true;

        const token = $page.params.token;

        const data = new FormData(event.currentTarget);
        const password = data.get("password");
        const confirmPassword = data.get("confirm-password");

        if (password !== confirmPassword) {
            toast.error("Passwords do not match");
            loading = false;

            return;
        }

        let body = {
            token,
            new_password: password,
        };

        try {
            let res = await APIClient.post(
                "/auth/reset_password_complete",
                body,
            );
            console.log(res);

            toast.success("password successfully reset.");
        } catch (err) {
            console.log(err);
            toast.error(err.response.data.data.error.message);
            loading = false;
        }
    }
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
        <form
            class="space-y-6 p-4 rounded-lg"
            on:submit|preventDefault={setPassword}
        >
            <!--
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
            -->
            <div>
                <Label for="password">New Password</Label>

                <div class="mt-2">
                    <Input
                        id="password"
                        name="password"
                        type="password"
                        required
                    />
                </div>
            </div>
            <div>
                <Label for="confirm-password">Confirm Password</Label>

                <div class="mt-2">
                    <Input
                        id="confirm-password"
                        name="confirm-password"
                        type="password"
                        required
                    />
                </div>
            </div>

            <div>
                {#if !loading}
                    <Button type="submit" class="w-full">Change</Button>
                {:else}
                    <Button class="w-full">Changing Password ...</Button>
                {/if}
            </div>
        </form>
    </div>
</div>
