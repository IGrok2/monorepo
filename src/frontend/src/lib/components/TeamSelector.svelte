<script>
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import { Button } from "$lib/components/ui/button";
    import * as Dialog from "$lib/components/ui/dialog";

    import { Layers, ChevronDown, Check, PlusCircle } from "lucide-svelte";

    let createTeamDialog = false;
</script>

<DropdownMenu.Root>
    <DropdownMenu.Trigger asChild let:builder>
        <Button
            builders={[builder]}
            variant="outline"
            class="flex w-full justify-between"
        >
            <Layers class="h-4 w-4 mr-2" />
            <span>Personal</span>
            <ChevronDown class="ml-1 h-4 w-4" />
        </Button>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content align="end" class="w-[200px]">
        <DropdownMenu.Label>Select Team</DropdownMenu.Label>
        <DropdownMenu.Separator />
        <DropdownMenu.Item on:click={() => createTeamDialog = true}>
            <div class="flex items-center justify-between">
                <span>Create Team</span>
                <PlusCircle class="ml-2 h-4 w-4" />
            </div>
        </DropdownMenu.Item>
    </DropdownMenu.Content>
</DropdownMenu.Root>

<Dialog.Root bind:open={createTeamDialog}>
    <Dialog.Content>
        <Dialog.Header>
            <Dialog.Title>Create A Team</Dialog.Title>
            <Dialog.Description>
                This is a Alpha feature reserved for large organizations, if you are working with a small number of individuals please use project members. 
                If you need centralized billing for your organization please contact Aidan@Packetware.net / Edward@Packetware.net.
                We would love to assist with your onboarding and correct usage of the platform.
            </Dialog.Description>
        </Dialog.Header>
    </Dialog.Content>
</Dialog.Root>
