import { createFileRoute } from "@tanstack/react-router";

import { API_BASE } from "@/api/utils";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";

export const Route = createFileRoute("/login")({
  component: LoginPage,
});

function LoginPage() {
  const handleGoogleSignIn = () => {
    window.location.assign(`${API_BASE}/auth/google-auth`);
  };

  return (
    <div className="bg-background min-h-svh text-left">
      <div className="mx-auto flex min-h-svh w-full max-w-6xl items-center justify-items-center gap-8 px-4 py-8 md:px-8">
        <Card className="w-full max-w-md justify-self-center md:justify-self-end">
          <CardHeader className="gap-2">
            <Badge variant="outline" className="w-fit">
              Welcome back
            </Badge>
            <CardTitle className="text-2xl">Sign in to Invoices</CardTitle>
            <CardDescription>Use your Google account to continue.</CardDescription>
          </CardHeader>
          <CardContent>
            <Button type="button" variant="outline" size="lg" className="w-full" onClick={handleGoogleSignIn}>
              <GoogleLogo data-icon="inline-start" />
              Continue with Google
            </Button>
          </CardContent>
          <CardFooter>
            <p className="text-muted-foreground text-center text-sm">
              Your account is created automatically after Google verifies your email.
            </p>
          </CardFooter>
        </Card>
      </div>
    </div>
  );
}

function GoogleLogo(props: React.SVGProps<SVGSVGElement>) {
  return (
    <svg viewBox="0 0 24 24" aria-hidden="true" {...props}>
      <path
        fill="#4285F4"
        d="M21.8 12.2c0-.7-.1-1.4-.2-2H12v3.8h5.5a4.7 4.7 0 0 1-2 3.1v2.5h3.2c1.9-1.7 3.1-4.3 3.1-7.4z"
      />
      <path
        fill="#34A853"
        d="M12 22c2.7 0 5-.9 6.7-2.4l-3.2-2.5c-.9.6-2 .9-3.5.9a6.1 6.1 0 0 1-5.7-4.2H3v2.6A10 10 0 0 0 12 22z"
      />
      <path fill="#FBBC05" d="M6.3 13.8a6 6 0 0 1 0-3.6V7.6H3a10 10 0 0 0 0 8.8l3.3-2.6z" />
      <path
        fill="#EA4335"
        d="M12 6c1.5 0 2.8.5 3.8 1.5l2.9-2.9A9.7 9.7 0 0 0 12 2a10 10 0 0 0-9 5.6l3.3 2.6A6.1 6.1 0 0 1 12 6z"
      />
    </svg>
  );
}
