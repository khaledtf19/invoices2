import React from "react";

// Minimal routing scaffold using TanStack Router style placeholders
export const ProtectedRedirect: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  // Placeholder: in a full app, check auth state here and redirect if needed
  return <>{children}</>;
};

const RouterStub: React.FC = () => {
  return (
    <div>
      <p>Router placeholder for frontend-auth scaffolding. Wire TanStack Router in a follow-up task.</p>
    </div>
  );
};

export default RouterStub;
