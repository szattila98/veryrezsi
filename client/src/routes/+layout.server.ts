
import type { LayoutServerLoad } from './$types'

export const load: LayoutServerLoad = ({ locals }) => {
  const { user } = locals // locals.user set by hooks.server.ts/handle()
  return {
    user
  }
}