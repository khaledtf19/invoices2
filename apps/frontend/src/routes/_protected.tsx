import { Outlet, createFileRoute, redirect } from "@tanstack/react-router";

export const Route = createFileRoute("/_protected")({
  beforeLoad: async () => {
    // TODO: Replace with actual auth check once token/session strategy is finalized.
    // For now, we use a cookie/session check against /auth/me.
    // If the backend uses HttpOnly cookies, a simple fetch with credentials: 'include'
    // will tell us if the session is valid.
    try {
      const apiBase = (import.meta.env.VITE_API_BASE_URL as string) || "";
      const res = await fetch(`${apiBase}/auth/me`, {
        credentials: "include",
        method: "GET",
      });
      if (!res.ok) {
        throw redirect({ to: "/login", search: { redirect: window.location.pathname } });
      }
    } catch (err) {
      if ((err as any).redirect) throw err;
      throw redirect({ to: "/login" });
    }
  },
  component: ProtectedLayout,
});

function ProtectedLayout() {
  return <Outlet />;
}
