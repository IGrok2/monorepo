<script>
    import { PUBLIC_API } from "$env/static/public";
    import logo from "$lib/assets/top-logo.png";
    import { getCookie } from "$lib/utils/auth.js";

    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import { Button } from "$lib/components/ui/button";

    import { toast } from "svelte-sonner";

    const token = getCookie("jwt");

    let name;
    let email;
    let password;
    let loading = false;

    async function register() {
        console.log(name, email, password);
        //if (!loading) {
        //loading = true;
        const response = await fetch(`${PUBLIC_API}/auth/register`, {
            method: "POST",
            Headers: new Headers({
                "Content-Type": "application/json",
                Authorization: token,
            }),
            body: JSON.stringify({
                name,
                email,
                password,
            }),
        });

        let res = await response.json();

        if (res.success) {
            toast.success(res.message);
            document.cookie = `jwt=Bearer ${res.token}; path=/`;
            document.location.href = "/i/dash";
        } else {
            toast.error(res.message);
            loading = false;
        }
        //} else {
        //toast.loading("Registration is processing");
        //}
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

                <Button type="submit" class="w-full" on:click={() => register()}
                    >{#if !loading}Create account{:else}Creating account ...{/if}</Button
                >
            </form>
        </div>
    </div>
</div>
