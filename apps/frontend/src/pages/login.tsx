import React from "react";
import AuthForm from "../components/AuthForm";
import type { LoginData } from "../components/AuthForm";

const LoginPage: React.FC = () => {
  const onSubmit = (data: LoginData) => {
    console.log("Login data submitted:", data);
  };

  return (
    <div>
      <h1>Login</h1>
      <AuthForm mode="login" onSubmit={onSubmit} />
    </div>
  );
};

export default LoginPage;
