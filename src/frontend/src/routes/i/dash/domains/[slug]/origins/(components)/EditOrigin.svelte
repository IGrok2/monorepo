<script>
    /** @type {import('./$types').ActionData} */
    export let form;

    import { page } from "$app/stores";
    import { fade } from "svelte/transition";
    import { enhance } from "$app/forms";

    import * as Card from "$lib/components/ui/card";
    import { Button } from "$lib/components/ui/button";
    import { Label } from "$lib/components/ui/label";
    import { Input } from "$lib/components/ui/input";
    import { Switch } from "$lib/components/ui/switch";

    import { X } from "lucide-svelte";

    import { toast } from "svelte-sonner";
    import { validateOrigin } from "$lib/utils/origins";

    import DocsLink from "$lib/base/DocsLink.svelte";
    import BasicLink from "$lib/base/BasicLink.svelte";

    export let domain_info;
    export let origin;

    export let show;
</script>

{#if show}
    <Card.Root>
        <Card.Header>
            <Card.Title class="flex justify-between items-center"
                ><h3>Edit Origin Settings for {origin._id}</h3>
                <Button
                    variant="ghost"
                    on:click={() => {
                        show = false;
                    }}><X /></Button
                ></Card.Title
            >
        </Card.Header>
        <Card.Content>
            <p>
                <Label for="origin-setting-host">Host:</Label>
                <a class="text-muted-foreground text-sm tracking-tight"
                    >where this origin setting will apply. This should include
                    the domain.</a
                >
                <Input
                    bind:value={origin._id}
                    type="text"
                    name="setting-name"
                    id="origin-setting-host"
                    placeholder="hey-jude.{domain_info._id}"
                    required
                />
            </p>

            <div class="lg:grid lg:grid-cols-2 lg:gap-4">
                <div class="flex flex-1 py-2">
                    <div>
                        <Label>SSL:</Label>
                        <a class="text-muted-foreground text-sm tracking-tight"
                            >whether we contact your origins over encryption
                            (TLS). It's recommended to have this on if this
                            origin transfers sensitive data. We accept both
                            self-signed certificates and CA-signed certificates.
                            When SSL is enabled, we won't accept plaintext
                            responses from your backend.</a
                        >
                    </div>
                    <Switch bind:checked={origin.ssl} />
                </div>

                <div class="flex flex-1 py-2">
                    <div>
                        <Label>HTTP/2:</Label>
                        <a class="text-muted-foreground text-sm tracking-tight"
                            >whether we should contact your backend over HTTP/2.
                            More efficient for a high number of concurrent
                            requests, but more expensive to create. If a HTTP/2
                            connection cannot be established, we will rely on
                            HTTP/1.1</a
                        >
                    </div>
                    <Switch bind:checked={origin.http2} />
                </div>

                <div class="flex flex-1 py-2">
                    <div>
                        <Label>Forward IP data:</Label>
                        <a class="text-muted-foreground text-sm tracking-tight"
                            >whether we should forward IP data, such as ASN and
                            country, to your backend via HTTP headers. See <BasicLink
                                text="documentation"
                                href="docs.packetware.net/origin_settings"
                            /> for more details</a
                        >
                    </div>
                    <Switch bind:checked={origin.ip_data} />
                </div>

                <p>
                    <Label for="origin-setting-timeout">Timeout:</Label>
                    <a class="text-muted-foreground text-sm tracking-tight"
                        >a 3-60 (three seconds to 60 seconds) value that how
                        long we should wait for a response from your backend.</a
                    >
                    <Input
                        bind:value={origin.timeout}
                        type="number"
                        min="3"
                        max="60"
                        name="origin-setting-timeout"
                        id="rule-order"
                        placeholder="10"
                        required
                    />
                </p>
            </div>

            <div class="mt-4">
                <div class="flex justify-between flex-1 items-center">
                    <div class="justify-start">
                        <Label>Origins:</Label>
                        <a class="text-muted-foreground text-sm tracking-tight"
                            >backends for {origin._id}</a
                        >
                    </div>
                    <div class="justify-end items-center">
                        <button
                            type="button"
                            on:click={() => {
                                origin.origins.push({
                                    url: "",
                                    weight: 1,
                                });
                                origin.origins = origin.origins;
                            }}
                            class="bg-fuchsia-500 text-white px-3 py-1.5 mx-3 rounded text-sm"
                        >
                            +
                        </button>
                    </div>
                </div>
                <div class="mt-4 lg:grid lg:grid-cols-2 lg:gap-4">
                    {#each origin.origins as destination, origin_index}
                        <Card.Root>
                            <Card.Header>
                                <Card.Title class="flex justify-between items-center"
                                    ><div>Origin #{origin_index + 1}</div>
                                    <div
                                        on:click={async () => {
                                            if (origin.origins.length > 1) {
                                                origin.origins.splice(
                                                    origin_index,
                                                    1,
                                                );
                                                origin.origins = [
                                                    ...origin.origins,
                                                ];
                                            }
                                        }}
                                        class="text-xl text-red-500 cursor-grab"
                                    >
                                        𝕏
                                    </div></Card.Title
                                >
                            </Card.Header>
                            <Card.Content>
                                <div>
                                    <Label for="ip">IP address or host:</Label>
                                    <p
                                        class="text-muted-foreground text-sm tracking-tight"
                                    >
                                        where we can contact this backend
                                    </p>
                                    <Input
                                        bind:value={destination.url}
                                        type="text"
                                        name="ip"
                                        id="ip"
                                        placeholder="23.133.104.69"
                                        required
                                    />
                                </div>
                            </Card.Content>
                            <Card.Footer>
                                <div>
                                    <Label for="weight">Weight:</Label>
                                    <p
                                        class="text-muted-foreground text-sm tracking-tight"
                                    >
                                        number from 1-20 that indicates how this
                                        backend should be weighted against other
                                        backends. A higher weight means that
                                        it's more likely to receive traffic. For
                                        more see information, see <DocsLink />
                                    </p>
                                    <Input
                                        bind:value={destination.weight}
                                        type="number"
                                        min="1"
                                        max="20"
                                        name="weight"
                                        id="weight"
                                        placeholder="10"
                                        required
                                    />
                                </div>
                            </Card.Footer>
                        </Card.Root>
                    {/each}
                </div>
            </div>

            {#if validateOrigin(origin)}
                <div class="flex justify-center pt-8">
                    <Button type="submit"
                        >Submit origin setting {origin._id}</Button
                    >
                </div>
            {:else}
                <div class="flex justify-center pt-8">
                    <p
                        class="mt-3 inline-flex w-full items-center justify-center rounded-md text-sm font-semibold text-white shadow-sm hover:bg-red-500 duration-100 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 sm:ml-3 sm:mt-0 sm:w-auto"
                    >
                        All fields and origins must be filled out to submit
                    </p>
                </div>
            {/if}
            <p class="text-sm text-slate-200">
                <br /><br />If you have any questions, our
                <a href="/support" class="text-indigo-500 hover:text-indigo-400"
                    >support team</a
                >
                stands by, ready to lend a hand if you have any questions. It is
                staffed by the same people who built Packetware. You can also check
                the <DocsLink /> on origin settings.
            </p>
        </Card.Content>
    </Card.Root>
{/if}
