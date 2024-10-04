import { writable } from "svelte/store";

export const variables = writable([])

export const pending_variables = writable([
    {
        name: "",
        value: "",
    },
])