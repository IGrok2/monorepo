<script>
    let discordId = "988237230701158442";
</script>

<div class="isolate px-6 py-12 sm:py-16 lg:px-8">
    <div class="mx-auto max-w-2xl text-center">
        <h2 class="text-3xl font-bold tracking-tight text-white sm:text-4xl">
            Contact
        </h2>
        <p class="mt-2 text-lg leading-8 text-gray-300">
            Have any questions or would like to get in touch with us?
        </p>
    </div>

    <form method="POST" class="mx-auto mt-16 max-w-xl sm:mt-20">
        <div class="grid grid-cols-1 gap-x-8 gap-y-6 sm:grid-cols-2">
            <div>
                <label
                    for="first-name"
                    class="block text-sm font-semibold leading-6 text-white"
                    >First name</label
                >
                <div class="mt-2.5">
                    <input
                        type="text"
                        name="first-name"
                        id="first-name"
                        autocomplete="given-name"
                        class="block w-full rounded-md border-0 px-3.5 py-2 text-white bg-gray-800 shadow-sm ring-1 ring-inset ring-gray-500 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-fuchsia-600 sm:text-sm sm:leading-6"
                    />
                </div>
            </div>
            <div>
                <label
                    for="last-name"
                    class="block text-sm font-semibold leading-6 text-white"
                    >Last name</label
                >
                <div class="mt-2.5">
                    <input
                        type="text"
                        name="last-name"
                        id="last-name"
                        autocomplete="family-name"
                        class="block w-full rounded-md border-0 px-3.5 py-2 text-white bg-gray-800 shadow-sm ring-1 ring-inset ring-gray-500 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-fuchsia-600 sm:text-sm sm:leading-6"
                    />
                </div>
            </div>
            <div class="sm:col-span-2">
                <label
                    for="company"
                    class="block text-sm font-semibold leading-6 text-white"
                    >Company</label
                >
                <div class="mt-2.5">
                    <input
                        type="text"
                        name="company"
                        id="company"
                        autocomplete="organization"
                        class="block w-full rounded-md border-0 px-3.5 py-2 text-white bg-gray-800 shadow-sm ring-1 ring-inset ring-gray-500 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-fuchsia-600 sm:text-sm sm:leading-6"
                    />
                </div>
            </div>
            <div class="sm:col-span-2">
                <label
                    for="email"
                    class="block text-sm font-semibold leading-6 text-white"
                    >Email</label
                >
                <div class="mt-2.5">
                    <input
                        type="email"
                        name="email"
                        id="email"
                        autocomplete="email"
                        class="block w-full rounded-md border-0 px-3.5 py-2 text-white bg-gray-800 shadow-sm ring-1 ring-inset ring-gray-500 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-fuchsia-600 sm:text-sm sm:leading-6"
                    />
                </div>
            </div>
            <div class="sm:col-span-2">
                <label
                    for="phone-number"
                    class="block text-sm font-semibold leading-6 text-white"
                    >Phone number</label
                >
                <input
                    type="tel"
                    name="phone-number"
                    id="phone-number"
                    autocomplete="tel"
                    class="block w-full rounded-md border-0 px-3.5 py-2 pl-20 text-white bg-gray-800 shadow-sm ring-1 ring-inset ring-gray-500 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-fuchsia-600 sm:text-sm sm:leading-6"
                />
            </div>
            <div class="sm:col-span-2">
                <label
                    for="message"
                    class="block text-sm font-semibold leading-6 text-white"
                    >Message</label
                >
                <div class="mt-2.5">
                    <textarea
                        name="message"
                        id="message"
                        rows="4"
                        class="block w-full rounded-md border-0 px-3.5 py-2 text-white bg-gray-800 shadow-sm ring-1 ring-inset ring-gray-500 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-fuchsia-600 sm:text-sm sm:leading-6"
                    ></textarea>
                </div>
            </div>
            <div class="flex gap-x-4 sm:col-span-2">
                <div class="flex h-6 items-center">
                    <!-- Enabled: "bg-fuchsia-600", Not Enabled: "bg-gray-200" -->
                    <button
                        type="button"
                        class="bg-gray-700 flex w-8 flex-none cursor-pointer rounded-full p-px ring-1 ring-inset ring-gray-100/5 transition-colors duration-200 ease-in-out focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-fuchsia-600"
                        role="switch"
                        aria-checked="true"
                        aria-labelledby="switch-1-label"
                    >
                        <span class="sr-only">Agree to policies</span>
                        <!-- Enabled: "translate-x-3.5", Not Enabled: "translate-x-0" -->
                        <span
                            aria-hidden="true"
                            class="translate-x-0 h-4 w-4 transform rounded-full bg-background shadow-sm ring-1 ring-gray-100/5 transition duration-200 ease-in-out"
                        ></span>
                    </button>
                </div>
                <label
                    class="text-sm leading-6 text-gray-300"
                    id="switch-1-label"
                >
                    By selecting this, you agree to our
                    <a
                        href="/legal/privacy"
                        class="font-semibold text-fuchsia-400"
                        >privacy&nbsp;policy</a
                    >.
                </label>
            </div>
        </div>
        <div class="mt-10">
            <button
                type="submit"
                class="block w-full rounded-md bg-fuchsia-400 px-3.5 py-2.5 text-center text-sm font-semibold text-white hover:text-black shadow-sm hover:bg-fuchsia-300 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-fuchsia-600"
                >Let's talk</button
            >
        </div>
    </form>
</div>
<div class="flex justify-center">
    <!-- Discord Box -->
    <div class="flex bg-gray-800 bg-opacity-50 rounded-lg p-6 mt-6">
        <p class="text-white mr-4">
            Join our Discord community to get the latest updates and support!
        </p>
        <iframe
            src="https://discord.com/widget?id={discordId}&theme=dark"
            width="350"
            height="200"
            allowtransparency="true"
            frameborder="0"
            sandbox="allow-popups allow-popups-to-escape-sandbox allow-same-origin allow-scripts"
            class="ml-auto"
        ></iframe>
    </div>
</div>
