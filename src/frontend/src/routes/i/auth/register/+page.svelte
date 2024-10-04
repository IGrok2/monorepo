<script>
    import APIClient from "$lib/utils/api";

    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import { Button } from "$lib/components/ui/button";

    import { toast } from "svelte-sonner";

    let name;
    let email;
    let password;
    let loading = false;

    async function register() {
        console.log("login");
        loading = true;

        let data = {
            name,
            email,
            password,
        };

        try {
            let res = await APIClient.post("/auth/register", data);
            console.log(res);

            toast.success(res.data.data.message);

            // Set JWT token in a cookie
            document.cookie = `jwt=${res.data.data.token}; path=/;`;
            document.location = "/i/dash";
        } catch (err) {
            console.log(err);
            toast.error(err.response.data.data.message);
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
                Welcome to Packetware 👋
            </h2>
            <a
                href="/i/auth/login"
                class="leading-9 tracking-tight font-semibold text-muted-foreground duration-100 text-right"
                ><p class="pr-4 after:content-['_↗']">
                    Already have an account? Login
                </p></a
            >
        </div>

        <div
            class="pt-10 sm:mx-auto sm:w-full sm:max-w-sm rounded-lg lg:backdrop-blur-lg"
        >
            <form class="space-y-6 backdrop-blur-sm p-4 rounded-lg">
                <div>
                    <Label for="name">Name</Label>
                    <div class="mt-2">
                        <Input
                            bind:value={name}
                            id="name"
                            name="name"
                            type="string"
                            autocomplete="name"
                            required
                        />
                    </div>
                </div>

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
                    </div>
                    <div class="mt-2">
                        <Input
                            id="password"
                            bind:value={password}
                            name="password"
                            type="password"
                            autocomplete="password"
                            required
                        />
                    </div>
                </div>

                <Button type="submit" class="w-full" disabled={loading} on:click={() => register()}
                    >{#if !loading}Create account{:else}Creating account ...{/if}</Button
                >
            </form>
        </div>
    </div>
</div>
