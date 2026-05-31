import { useEffect } from "react";
import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { toast } from "sonner";

import { API_BASE } from "@/api/utils";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Spinner } from "@/components/ui/spinner";

export const Route = createFileRoute("/auth/callback")({
  component: AuthCallbackPage,
});

function AuthCallbackPage() {
  const navigate = useNavigate();

  useEffect(() => {
    if (window.location.search) {
      window.history.replaceState(null, "", window.location.pathname);
    }

    const verifySession = async () => {
      try {
        const response = await fetch(`${API_BASE}/auth/me`, {
          credentials: "include",
          method: "GET",
        });

        if (!response.ok) {
          throw new Error("OAuth session could not be verified");
        }

        toast.success("Signed in successfully");
        await navigate({ to: "/", replace: true });
      } catch {
        toast.error("Google sign-in could not be completed");
        await navigate({ to: "/login", replace: true });
      }
    };

    void verifySession();
  }, [navigate]);

  return (
    <main className="flex min-h-svh items-center justify-center bg-background p-4 text-left">
      <Card className="w-full max-w-sm">
        <CardHeader>
          <CardTitle>Completing sign-in</CardTitle>
          <CardDescription>Checking your secure session before opening the app.</CardDescription>
        </CardHeader>
        <CardContent className="flex items-center gap-3 text-sm text-muted-foreground">
          <Spinner />
          Verifying Google sign-in
        </CardContent>
      </Card>
    </main>
  );
}
