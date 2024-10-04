<script>
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import { Button } from "$lib/components/ui/button";

    import { Github } from "lucide-svelte";

    import { PUBLIC_API } from "$env/static/public";

    import { getCookie } from "$lib/utils/auth";

    /** @type {import('./$types').PageData} */
    export let data;

    /** @type {import('./$types').ActionData} */
    export let form;

    import { enhance } from "$app/forms";

    import { toast } from "svelte-sonner";

    const profile = data.profile;

    $: console.log(form);

    const token = getCookie(`jwt`);

    const installGithubApp = async () => {
        const response = await fetch(`${PUBLIC_API}/@/container/begin-auth`, {
            method: "GET",
            headers: new Headers({
                "content-type": "application/json",
                Authorization: token,
            }),
        });

        const res = await response.json();
        console.log(res);

        return {
            status: 302,
            redirect: res.installURL,
        };
    };
</script>

<div class="flex justify-right">
    {#if form}
        {form.message}
    {/if}
</div>

<!-- Settings forms -->
<div class="divide-y divide-white/5">
    <div
        class="grid max-w-7xl grid-cols-1 gap-x-8 gap-y-10 px-4 py-16 sm:px-6 md:grid-cols-3 lg:px-8"
    >
        <div>
            <h2 class="text-base font-semibold leading-7 text-primary">
                Personal Information
            </h2>
            <p class="mt-1 text-sm leading-6 text-muted-foreground">
                Use a permanent address where you can receive mail.
            </p>
        </div>

        <form
            method="POST"
            action="?/changeInformation"
            class="md:col-span-2"
            use:enhance={({}) => {
                return async ({ result, update }) => {
                    switch (result.type) {
                        case "success":
                            toast.success("Deatils successfully changed.");
                            break;
                        case "error":
                            toast.error(result.error.message);
                            break;
                        default:
                            break;
                    }
                };
            }}
        >
            <div
                class="grid grid-cols-1 gap-x-6 gap-y-8 sm:max-w-xl sm:grid-cols-6"
            >
                <div class="col-span-full">
                    <Label for="name">Name</Label>
                    <Input
                        type="text"
                        name="name"
                        id="name"
                        value={profile.user.name}
                    />
                    <p class="text-sm text-muted-foreground">
                        Enter your name.
                    </p>
                </div>

                <div class="col-span-full">
                    <Label for="email">Email</Label>
                    <Input
                        type="email"
                        name="email"
                        id="email"
                        value={profile.user.email}
                    />
                    <p class="text-sm text-muted-foreground">
                        Enter your email address.
                    </p>
                </div>

                <div class="col-span-full">
                    <Label for="verify-password">Verify Password</Label>
                    <Input
                        type="password"
                        name="verify-password"
                        id="verify-password"
                        autocomplete="current-password"
                    />
                    <p class="text-sm text-muted-foreground">
                        Enter your password to verify your changes.
                    </p>
                </div>
            </div>

            <div class="mt-8 flex">
                <Button type="submit">Save</Button>
            </div>
        </form>
    </div>

    <div
        class="grid max-w-7xl grid-cols-1 gap-x-8 gap-y-10 px-4 py-16 sm:px-6 md:grid-cols-3 lg:px-8"
    >
        <div>
            <h2 class="text-base font-semibold leading-7 text-primary">
                Change password
            </h2>
            <p class="mt-1 text-sm leading-6 text-muted-foreground">
                Update your password associated with your account.
            </p>
        </div>

        <form
            method="POST"
            action="?/changePassword"
            class="md:col-span-2"
            use:enhance={({}) => {
                return async ({ result, update }) => {
                    switch (result.type) {
                        case "success":
                            toast.success("Deatils successfully changed.");
                            break;
                        case "error":
                            toast.error(result.error.message);
                            break;
                        default:
                            break;
                    }
                };
            }}
        >
            <div
                class="grid grid-cols-1 gap-x-6 gap-y-8 sm:max-w-xl sm:grid-cols-6"
            >
                <div class="col-span-full">
                    <Label for="current-password">Current Password</Label>
                    <Input
                        type="password"
                        name="current-password"
                        id="current-password"
                        autocomplete="current-password"
                    />
                    <p class="text-sm text-muted-foreground">
                        Enter your current password..
                    </p>
                </div>

                <div class="col-span-full">
                    <Label for="new-password">New password</Label>
                    <Input
                        type="password"
                        name="new-password"
                        id="new-password"
                        autocomplete="current-password"
                    />
                    <p class="text-sm text-muted-foreground">
                        Enter your new password.
                    </p>
                </div>

                <div class="col-span-full">
                    <Label for="confirm-password">Confirm Password</Label>
                    <Input
                        type="password"
                        name="confirm-password"
                        id="confirm-password"
                        autocomplete="current-password"
                    />
                    <p class="text-sm text-muted-foreground">
                        Enter your new password again to confirm.
                    </p>
                </div>
            </div>

            <div class="mt-8 flex">
                <Button type="submit">Save</Button>
            </div>
        </form>
    </div>

    <div
        class="grid max-w-7xl grid-cols-1 gap-x-8 gap-y-10 px-4 py-16 sm:px-6 md:grid-cols-3 lg:px-8"
    >
        <div>
            <h2 class="text-base font-semibold leading-7 text-primary">
                Connected Accounts
            </h2>
            <p class="mt-1 text-sm leading-6 text-muted-foreground">
                Connect accounts of development platforms to Packetware.
            </p>
        </div>

        <div class="mt-8 flex">
            <Button on:click={installGithubApp}><Github class="w-4 h-4" /> GitHub</Button>
        </div>
    </div>
</div>
