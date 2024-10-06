<script>
    import APIClient from "$lib/utils/api";
    import { page } from "$app/stores";
    
    import { toast } from "svelte-sonner";
    import { Button } from "$lib/components/ui/button";
    import * as Card from "$lib/components/ui/card";
    import * as RadioGroup from "$lib/components/ui/radio-group";
    import { Separator } from "$lib/components/ui/separator";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import { Slider } from "$lib/components/ui/slider";

    /** @type {import('./$types').PageData} */
    export let data;
    const { regions } = data;

    let slug = "";
    let size = [5];
    let selectedRegion = regions[0].Slug;
    let blockSubmit = false;

    async function createVolume() {
        blockSubmit = true;
        toast.loading("Attempting to create volume.");

        const body = {
            slug: slug,
            size: size[0],
            region: selectedRegion
        }

        try {
            let res = await APIClient.post(
                `/project/${$page.params.project}/environment/${$page.params.environment}/volume`,
                body
            );

            toast.success(res.data.data.message)
        } catch (err) {
            console.log("err: ", err);
            toast.error(err.response.data.data.message)
            blockSubmit = false;
        }
    }
</script>

<div class="container my-8 space-y-2">
    <h3 class="text-xl">Create Volume</h3>
    <div class="flex w-full flex-col gap-1.5">
        <Label for="slug">Slug</Label>
        <Input
            type="text"
            id="slug"
            name="slug"
            bind:value={slug}
            placeholder="volume-slug-name"
        />
    </div>
    <div class="flex w-full flex-col gap-1.5">
        <div class="flex w-full justify-between">
            <Label for="size">Size</Label>
            <Label>{size[0]} GB</Label>
        </div>
        <Slider bind:value={size} min={1} max={25} step={1} />
    </div>
    <RadioGroup.Root bind:value={selectedRegion}>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {#each regions as region}
                <Card.Root>
                    <div class="flex h-full items-center">
                        <div class="ml-4 h-full flex items-center space-x-4">
                            <RadioGroup.Item
                                value={region.Slug}
                                id={region.Slug}
                            />
                            <Separator orientation="vertical" />
                        </div>
                        <Card.Header>
                            <Card.Title>{region.Name}</Card.Title>
                        </Card.Header>
                    </div>
                </Card.Root>
            {/each}
        </div>
    </RadioGroup.Root>
    <div class="flex justify-end">
        <Button type="submit" disabled={blockSubmit} on:click={createVolume}
            >Create</Button
        >
    </div>
</div>
