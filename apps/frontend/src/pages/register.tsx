import React from "react";
import AuthForm from "../components/AuthForm";
import { RegisterData } from "../components/AuthForm";

const RegisterPage: React.FC = () => {
  const onSubmit = (data: RegisterData) => {
    console.log("Register data submitted:", data);
  };

  return (
    <div>
      <h1>Register</h1>
      <AuthForm mode="register" onSubmit={onSubmit} />
    </div>
  );
};

export default RegisterPage;
