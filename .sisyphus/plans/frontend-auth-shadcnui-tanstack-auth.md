## Plan Title: Frontend Auth with ShadcnUI + TanStack (Rust backend)

## TL;DR

> Summary: Plan to implement SPA authentication using Shadcn UI components, TanStack Query for data fetching, and
> TanStack Router for protected routes, consuming Rust backend auth APIs. The plan includes token storage via HttpOnly
> cookies with refresh tokens and a future path for OAuth integration. Deliverables: UI login/register, protected
> routes, API client, token management, guard logic, tests and QA scenarios. Effort: Large Parallel: YES - multiple
> independent UI components and routing guards can be developed in parallel. Critical Path: Setup API client → Implement
> login → Implement protected routes → Implement refresh flow → Integrate components → QA.

## Context

### Original Request

- Frontend auth using shadcnui, TanStack Query, and TanStack Router connecting to Rust backend auth APIs.

### Interview Summary

- We discussed token storage preference (HttpOnly cookies with refresh tokens), route protection approach (loader-based
  guards on root), and OAuth as a future option.
- We identified key API endpoints to integrate with (login, register, refresh, me).

### Metis Review (gaps addressed)

- Gaps identified and addressed per Metis run: CSRF strategy, token lifetimes/rotation, error handling model, and user
  state hydration across routes.
- Guardrails recommended: adopt HttpOnly refresh cookies with security flags; implement a central error normalization;
  plan for silent token refresh on 401; document publicly the security assumptions.

## Work Objectives

### Core Objective

- Provide a secure, usable authentication experience in the SPA that aligns with the backend auth APIs and adheres to
  the chosen token strategy and routing guards.

### Deliverables

- Frontend login and register pages using Shadcn UI components.
- API client wrappers with TanStack Query hooks for login, register, refresh, and user profile.
- TanStack Router configuration with protected routes guarded by loader-based checks.
- Token storage and CSRF considerations documented and implemented.
- QA scenarios covering happy path, edge cases, and failure modes.

### Definition of Done (DoD)

- All pages/components render correctly with the chosen UI library.
- API client successfully authenticates, stores tokens in HttpOnly cookies, and refreshes tokens on expiry.
- Protected routes redirect unauthenticated users to login.
- All tasks have QA scenarios and acceptance criteria.

### Must Have

- Login page with email/password fields, validation, and error handling.
- Register page with required fields and success flow.
- API client with endpoints: login, register, refresh, me.
- Auth guard on root routes using TanStack Router loaders.
- Token storage in HttpOnly cookies with Secure and SameSite flags.

### Must NOT Have (guardrails)

- Do not expose tokens to front-end JS; avoid localStorage.
- Do not bypass login guard for protected routes.
- No server-side authentication logic in frontend code.

## Verification Strategy

- Agent-executed tests for login and protected routes (happy path and failures).
- UI QA for login/register flows with the chosen UI components.
- Evidence: .sisyphus/evidence/task-frontend-auth-\*.md

## Execution Strategy

### Parallel Execution Waves

> Target: 5 tasks per wave. Waves designed to maximize parallelism while respecting dependencies.

- Wave 1: Foundation
- [x] Setup project structure for auth (src/pages/login.tsx, src/pages/register.tsx, src/components/AuthForm.tsx)
- [ ] Implement API client helpers (authApi.ts) and TanStack Query hooks (useLogin, useRegister, useMe, useRefresh)
- [ ] Configure TanStack Router with ProtectedRoot and route guards
- [ ] Implement Shadcn UI login/register forms with validation
- [ ] Token storage strategy: HttpOnly cookies setup (cookie names, Secure, SameSite)

Wave 2: Integration

- [ ] Hook up login/register UI with API hooks
- [ ] Implement refresh flow and automatic token renewal
- [ ] Implement protected route redirection logic
- [ ] Implement error handling and user feedback (toasts, inline messages)

Wave 3: QA & Polish

- [ ] Add automated tests/scenarios for login + protected routes
- [ ] Accessibility and keyboard navigation checks
- [ ] UI polish and consistency checks with shadcn-ui

## Dependency Matrix

- TanStack Router: route guards and loaders
- TanStack Query: data fetching hooks
- Shadcn UI: components for forms and layout
- Rust backend: auth APIs at /auth/login, /auth/register, /auth/refresh, /auth/me

## Agent Dispatch Summary

- Wave 1: Foundation (authApi, login/register pages, route guards)
- Wave 2: Integration (connect UI to API, refresh, redirects)
- Wave 3: QA

## Final Verification Wave

- F1 Plan Compliance Audit — oracle
- F2 Code Quality Review — unspecified-high
- F3 Real Manual QA — unspecified-high
- F4 Scope Fidelity Check — deep

## References

- API contracts: TBD based on backend docs
- UI components: shadcn-ui docs
- Router: TanStack Router docs

## Plan Generated: Frontend Auth with ShadcnUI + TanStack (Rust backend)

**Key Decisions**: [Decision] Rationale

- [DECISION] Token storage using HttpOnly cookies for refresh tokens; access tokens kept in memory or short-lived
  storage to reduce exposure. Rationale: reduces XSS risk and aligns with cookie-based refresh flow.
- [DECISION] Protected routes implemented via loader-based guards on root routes. Rationale: aligns with TanStack Router
  patterns and provides early redirection.
- [DECISION] OAuth/social login planned for future; no third-party integrations in current scope. Rationale: reduces
  surface area and keeps MVP stable.
- [DECISION] Endpoints to integrate with: /auth/login, /auth/register, /auth/refresh, /auth/me. Rationale: aligns with
  common backend contracts.

**Scope**: IN: Frontend auth UI (login/register), API client, routing guards, token management; OUT: Backend auth
implementation, non-auth UI.

**Guardrails** (Metis): Implement CSRF considerations for cookie-based tokens, define token lifetimes, and ensure
explicit error handling strategies.

**Auto-Resolved**:

- CSRF approach defaults to CSRF token header if cookies used; abstracted behind API client.
- Token lifetimes: access tokens short-lived; refresh lifetime managed by server and rotation policy defaulted to
  rotation-on-use.

**Defaults Applied**:

- Framework: React SPA (Vite) with TanStack Router and TanStack Query; UI: Shadcn UI components.
- API client uses baseURL from env and auto-refresh on 401.

**Decisions Needed**:

- [DECISION NEEDED] CSRF strategy details (token-in-header pattern vs same-site cookie protections) for mutate requests.
- [DECISION NEEDED] Exact token lifetimes (access/refresh) and refresh rotation policy.
- [DECISION NEEDED] Exact error shape and UI error handling policy.
- [DECISION NEEDED] Hydration strategy for user state across page reloads.

Plan saved to: .sisyphus/plans/frontend-auth-shadcnui-tanstack-auth.md
