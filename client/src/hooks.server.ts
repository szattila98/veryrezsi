import { callWhoAmIApi } from '$routes/api/me/+server'
import serverConfig from '$lib/server/server.config'
import type { Handle, RequestEvent } from '@sveltejs/kit'
import type { User } from '$shared/domain'


// Attach authorization to each server request (role may have changed)
async function attachUserToRequestEvent(sessionId: string, event: RequestEvent) {
	const user: User|null = await callWhoAmIApi(sessionId)

	if (user) {
		event.locals.user = {
			email: user.email,
			username: user.username
		  }
	}
  }

// Invoked for each endpoint called and initially for SSR router
export const handle: Handle = async ({ event, resolve }) => {
  const { cookies } = event
  const sessionId = cookies.get(serverConfig.sessionCookieName)

  // before endpoint or page is called
  if (sessionId) {
    await attachUserToRequestEvent(sessionId, event)
  }

  if (!event.locals.user) cookies.delete(serverConfig.sessionCookieName)

  const response = await resolve(event)

  // after endpoint or page is called

  return response
}