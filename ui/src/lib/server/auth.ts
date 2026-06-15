// src/lib/server/auth.ts
import { redirect, error } from '@sveltejs/kit'

export function requireAuth(locals: App.Locals) {
    if (!locals.user) redirect(303, '/login')
    return locals.user
}

// "user must have AT LEAST ONE of these roles"
export function requireAnyRole(locals: App.Locals, ...roles: App.Role[]) {
    const user = requireAuth(locals)
    const hasRole = roles.some((r) => user.roles.includes(r))
    if (!hasRole) error(403, 'Forbidden')
    return user
}

// "user must have ALL of these roles"
export function requireAllRoles(locals: App.Locals, ...roles: App.Role[]) {
    const user = requireAuth(locals)
    const hasAll = roles.every((r) => user.roles.includes(r))
    if (!hasAll) error(403, 'Forbidden')
    return user
}

// handy boolean for UI logic
export function hasRole(user: NonNullable<App.Locals['user']>, ...roles: App.Role[]) {
    return roles.some((r) => user.roles.includes(r))
}
