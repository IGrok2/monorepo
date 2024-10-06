<script>
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";

  import { CheckCircle, ArrowRight } from "lucide-svelte";
</script>

<div class="min-h-screen flex items-center justify-center p-4">
  <Card.Root class="w-full max-w-md">
    <Card.Header class="text-center">
      <div
        class="mx-auto mb-4 w-16 h-16 bg-primary/10 rounded-full flex items-center justify-center"
      >
        <CheckCircle class="w-10 h-10 text-primary" aria-hidden="true" />
      </div>
      <Card.Title class="text-2xl font-bold"
        >Installation Successful!</Card.Title
      >
      <Card.Description
        >Your GitHub app has been successfully installed.</Card.Description
      >
    </Card.Header>
    <Card.Content class="space-y-4">
      <p class="text-center text-muted-foreground">
        Thank you for installing our GitHub app. You can now enjoy seamless
        integration with your repositories.
      </p>
      <ul class="space-y-2">
        <li class="flex items-center">
          <ArrowRight class="w-4 h-4 mr-2 text-primary" aria-hidden="true" />
          <span>Automated workflow management</span>
        </li>
        <li class="flex items-center">
          <ArrowRight class="w-4 h-4 mr-2 text-primary" aria-hidden="true" />
          <span>Enhanced collaboration tools</span>
        </li>
        <li class="flex items-center">
          <ArrowRight class="w-4 h-4 mr-2 text-primary" aria-hidden="true" />
          <span>Advanced security features</span>
        </li>
      </ul>
    </Card.Content>
    <Card.Footer>
      <Button href="/i/dash" class="w-full">Go To Dashboard</Button>
    </Card.Footer>
  </Card.Root>
</div>
