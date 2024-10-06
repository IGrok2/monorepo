<script>
    import * as Sheet from "$lib/components/ui/sheet";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import { Button } from "$lib/components/ui/button";
    import * as Avatar from "$lib/components/ui/avatar";

    import { Menu } from "lucide-svelte";

    import TeamSelector from "$lib/components/TeamSelector.svelte";
    import Notifications from "$lib/components/Notifications.svelte";

    import logo from "$lib/assets/Asset 19.png";
</script>

<header class="bg-background border-b">
    <nav class="px-4 md:px-6 flex items-center h-16">
        <img class="h-6 w-13 mr-4" src={logo} alt="Packetware Logo" />
        <div class="hidden md:flex items-center gap-4 md:gap-6 flex-1">
            <Button href="/i/dash/domains" variant="ghost">Domains</Button>
            <Button href="/i/dash/" variant="ghost">Projects</Button>
            <!-- <Button href="#" variant="ghost">Activity</Button> -->
        </div>
        <Sheet.Root>
            <Sheet.Trigger asChild let:builder>
                <div class="flex w-full justify-end">
                    <Button
                        builders={[builder]}
                        variant="outline"
                        size="icon"
                        class="md:hidden"
                    >
                        <Menu class="h-6 w-6" />
                        <span class="sr-only">Toggle navigation</span>
                    </Button>
                </div>
            </Sheet.Trigger>
            <Sheet.Content side="left" class="md:hidden">
                <div class="mt-8">
                    <TeamSelector />
                    <div class="grid gap-4 p-4">
                        <Button href="/i/dash/domains" variant="ghost"
                            >Domains</Button
                        >
                        <Button href="/i/dash" variant="ghost">Projects</Button>
                    </div>
                </div>
            </Sheet.Content>
        </Sheet.Root>
        <div class="hidden md:flex items-center gap-4">
            <!--<Notifications />-->
            <TeamSelector />
            <DropdownMenu.Root>
                <DropdownMenu.Trigger
                    ><Avatar.Root>
                        <Avatar.Image
                            src="https://github.com/shadcn.png"
                            alt="@shadcn"
                        />
                        <Avatar.Fallback>CN</Avatar.Fallback>
                    </Avatar.Root></DropdownMenu.Trigger
                >
                <DropdownMenu.Content>
                    <DropdownMenu.Group>
                        <DropdownMenu.Label>My Account</DropdownMenu.Label>
                        <DropdownMenu.Separator />
                        <DropdownMenu.Item>Profile</DropdownMenu.Item>
                        <DropdownMenu.Item>Billing</DropdownMenu.Item>
                    </DropdownMenu.Group>
                </DropdownMenu.Content>
            </DropdownMenu.Root>
        </div>
    </nav>
</header>
