<script lang="ts">
    import { ArrowDownNarrowWide, Check, PlusCircle } from "lucide-svelte";
    import { cn } from "$lib/utils";
    import * as Avatar from "$lib/components/ui/avatar";
    import { Button } from "$lib/components/ui/button";
    import * as Command from "$lib/components/ui/command";
    import * as Dialog from "$lib/components/ui/dialog";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import * as Popover from "$lib/components/ui/popover";
    import * as Select from "$lib/components/ui/select";
    import { tick } from "svelte";

    let className: string | undefined | null = undefined;
    export { className as class };

    const groups = [
        {
            label: "Personal Account",
            projects: [
                {
                    label: "Alicia Koch",
                    value: "personal",
                },
            ],
        },
        {
            label: "Projects",
            projects: [
                {
                    label: "Acme Inc.",
                    value: "acme-inc",
                },
                {
                    label: "Monsters Inc.",
                    value: "monsters",
                },
            ],
        },
    ];

    type Project = (typeof groups)[number]["projects"][number];

    let open = false;
    let showProjectDialog = false;

    let selectedProject: Project = groups[0].projects[0];

    function closeAndRefocusTrigger(triggerId: string) {
        open = false;

        tick().then(() => document.getElementById(triggerId)?.focus());
    }
</script>

<Dialog.Root bind:open={showProjectDialog}>
    <Popover.Root bind:open let:ids>
        <Popover.Trigger asChild let:builder>
            <Button
                builders={[builder]}
                variant="outline"
                role="combobox"
                aria-expanded={open}
                aria-label="Select a project"
                class={cn("w-[200px] justify-between", className)}
            >
                <Avatar.Root class="mr-2 h-5 w-5">
                    <Avatar.Image
                        src="https://avatar.vercel.sh/${selectedProject.value}.png"
                        alt={selectedProject.label}
                        class="grayscale"
                    />
                    <Avatar.Fallback>SC</Avatar.Fallback>
                </Avatar.Root>
                {selectedProject.label}
                <ArrowDownNarrowWide class="ml-auto h-4 w-4 shrink-0 opacity-50" />
            </Button>
        </Popover.Trigger>
        <Popover.Content class="w-[200px] p-0">
            <Command.Root>
                <Command.Input placeholder="Search project..." />
                <Command.List>
                    <Command.Empty>No project found.</Command.Empty>
                    {#each groups as group}
                        <Command.Group heading={group.label}>
                            {#each group.projects as project}
                                <Command.Item
                                    onSelect={() => {
                                        selectedProject = project;
                                        closeAndRefocusTrigger(ids.trigger);
                                    }}
                                    value={project.label}
                                    class="text-sm"
                                >
                                    <Avatar.Root class="mr-2 h-5 w-5">
                                        <Avatar.Image
                                            src="https://avatar.vercel.sh/${project.value}.png"
                                            alt={project.label}
                                            class="grayscale"
                                        />
                                        <Avatar.Fallback>SC</Avatar.Fallback>
                                    </Avatar.Root>
                                    {project.label}
                                    <Check
                                        class={cn(
                                            "ml-auto h-4 w-4",
                                            selectedProject.value !== project.value &&
                                                "text-transparent",
                                        )}
                                    />
                                </Command.Item>
                            {/each}
                        </Command.Group>
                    {/each}
                </Command.List>
                <Command.Separator />
                <Command.List>
                    <Command.Group>
                        <Command.Item
                            onSelect={() => {
                                open = false;
                                showProjectDialog = true;
                            }}
                        >
                            <PlusCircle class="mr-2 h-5 w-5" />
                            Create Project
                        </Command.Item>
                    </Command.Group>
                </Command.List>
            </Command.Root>
        </Popover.Content>
    </Popover.Root>
    <Dialog.Content>
        <Dialog.Header>
            <Dialog.Title>Create project</Dialog.Title>
            <Dialog.Description>
                Add a new project to manage products and customers.
            </Dialog.Description>
        </Dialog.Header>
        <div>
            <div class="space-y-4 py-2 pb-4">
                <div class="space-y-2">
                    <Label for="name">Project name</Label>
                    <Input id="name" placeholder="Acme Inc." />
                </div>
                <div class="space-y-2">
                    <Label for="plan">Subscription plan</Label>
                    <Select.Root>
                        <Select.Trigger>
                            <Select.Value placeholder="Select a plan" />
                        </Select.Trigger>
                        <Select.Content>
                            <Select.Item value="free">
                                <span class="font-medium">Free </span>-<span
                                    class="text-muted-foreground"
                                >
                                    Trial for two weeks
                                </span>
                            </Select.Item>
                            <Select.Item value="pro">
                                <span class="font-medium">Pro</span> -
                                <span class="text-muted-foreground">
                                    $9/month per user
                                </span>
                            </Select.Item>
                        </Select.Content>
                    </Select.Root>
                </div>
            </div>
        </div>
        <Dialog.Footer>
            <Button variant="outline" on:click={() => (showProjectDialog = false)}
                >Cancel</Button
            >
            <Button type="submit">Continue</Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
