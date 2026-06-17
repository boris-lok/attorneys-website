// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces

import type { PayLoad } from '$lib/utils'

declare global {
    namespace App {
        // interface Error {}
        interface Locals {
            user: PayLoad
        }
        // interface PageData {}
        // interface PageState {}
        // interface Platform {}

        type Role = 'admin' | 'lawyer'
    }
}

export {}
