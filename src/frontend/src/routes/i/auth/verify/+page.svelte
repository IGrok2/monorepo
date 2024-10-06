<script>
        import APIClient from "$lib/utils/api";
        import { toast } from "svelte-sonner";

        import { Button } from "$lib/components/ui/button";
        import { Input } from "$lib/components/ui/input";
        import { Label } from "$lib/components/ui/label";

        let loading = false;

        /** @param {{ currentTarget: EventTarget & HTMLFormElement}} event */
        async function verifyEmail(event) {
                loading = true;

                const data = new FormData(event.currentTarget);
                const code = data.get("code");

                let body = {
                        code,
                };

                try {
                        let res = await APIClient.post(
                                "/auth/verify_email",
                                body,
                        );
                        console.log(res);

                        toast.success("Enail verified.");
                } catch (err) {
                        console.log(err);
                        toast.error(err.response.data.data.error.message);
                        loading = false;
                }
        }
</script>

<div
        class="flex min-h-full flex-col justify-center pb-20 lg:pb-44 sm:ax-w-md text-center"
>
        <div class="backdrop-blur-md p-8 lg:px-16">
                <form
                        class="space-y-6 backdrop-blur-sm p-4 rounded-lg"
                        on:submit|preventDefault={verifyEmail}
                >
                        <h2
                                class="sm:mx-auto sm:w-full text-4xl font-bold text-slate-200"
                        >
                                To continue, you'll need to verify your email
                        </h2>
                        <p class="text-slate-300 pt-2 tracking-tight">
                                We've sent you an email with a code to verify
                                your email address. Enter it below:
                        </p>
                        <div class="mt-2 w-full flex justify-center">
                                <Input
                                        id="code"
                                        name="code"
                                        type="text"
                                        required
                                />
                        </div>
                        <Button type="submit" class="w-full" disabled={loading}
                                >{#if !loading}Verify{:else}Verifying ...{/if}</Button
                        >
                </form>
                <p class="text-slate-300 pt-2 tracking-tight">
                        If you haven't received the email, check your spam
                        folder or <button
                                class="text-fuchsia-500"
                                on:click={resendEmail}
                                >click here to resend the email</button
                        >.
                </p>
        </div>
</div>
