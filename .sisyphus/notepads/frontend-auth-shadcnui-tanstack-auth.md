Gap Analysis — Frontend Auth (ShadcnUI + TanStack) Wave 1 to Wave 2

- Summary: This document summarizes gaps, decisions, and a concrete plan to advance Wave 2 (UI wiring, route guards,
  token strategy, error handling) for the frontend authentication integration with a Rust backend.

Gaps categorized by criticality

- Critical
  1. CSRF strategy for cookie-based refresh tokens: choose between an explicit CSRF token header approach versus relying
     solely on SameSite cookie protections. This impacts API client behavior and backend coordination.
  2. Token lifetimes and rotation policy: define exact lifetimes for access and refresh tokens and rotation policy
     (rotate on refresh, revoke old tokens, cross-tab consistency).
  3. Hydration and user state sharing: decide how to hydrate user state on startup and how to share it across routes
     (AuthProvider, me() hydration, QueryClient interaction).
  4. Protected routes guard pattern: confirm loader-based guard on root vs per-route RequireAuth wrapper and expected UX
     (loading/redirect flows).

- Major 5) Error handling model: standardize APIError payload and mapping to UI errors; decide between toasts vs inline
  errors or a combination. 6) Environment/baseURL strategy: confirm env var naming and fallback behavior; ensure
  CORS/credentials config across environments. 7) Tests scaffolding: establish MSW-driven tests for hooks and some UI
  wiring checks.

- Ambiguous 8) OAuth/social login MVP: decide whether to include in Wave 2 planning or defer entirely.

Wave 2 concrete actions (proposed)

- Document/commit to CSRF strategy; implement interface in API client scaffolding to support the chosen approach for
  /auth/refresh flows.
- Lock token lifetimes and rotation policy; implement silent refresh mechanism in the API client (retry on 401, refresh
  token, retry last request).
- Introduce hydration mechanism (AuthContext or dedicated hook) and wire /auth/me startup flow where feasible.
- Implement a simple RequireAuth wrapper and a sample protected route to validate UX and flow.
- Build a centralized error mapper (normalizeApiError) and surface errors through UI (toasts/inline).
- Establish a minimal tests scaffold for hooks and UI wiring; outline MSW mocks for login/register/me/refresh.

Key questions to resolve with plan owner

- What CSRF strategy should we adopt for cookie-based refresh: header token pattern or rely on SameSite cookies with
  server-side checks?
- What are the target lifetimes and rotation rules for access/refresh tokens?
- On app startup, should we automatically hydrate via /auth/me or defer until a protected route is loaded?
- Guard approach preference: loader-based root guard or per-route RequireAuth wrapper?
- Standardized APIError payload shape and mapping rules?
- Should OAuth be included in Wave 2 or deferred to a later wave?

This Notepad entry serves as a living artifact for decisions and planning alignment.
