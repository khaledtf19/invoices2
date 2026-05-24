import React, { useState } from "react";

// Lightweight AuthForm scaffold (replace with shadcn-ui components in a follow-up)
export type LoginData = {
  email: string;
  password: string;
};

export type RegisterData = {
  name: string;
  email: string;
  password: string;
};

type Props = {
  mode: "login" | "register";
  onSubmit: (data: LoginData | RegisterData) => void;
};

export default function AuthForm({ mode, onSubmit }: Props) {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [name, setName] = useState("");
  const [errors, setErrors] = useState<string[]>([]);

  const isRegister = mode === "register";

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const errs: string[] = [];
    if (!email || !email.includes("@")) errs.push("Please provide a valid email");
    if (!password || password.length < 6) errs.push("Password must be at least 6 characters");
    if (isRegister && (!name || name.trim().length < 2)) errs.push("Name is required");
    setErrors(errs);
    if (errs.length === 0) {
      if (isRegister) {
        onSubmit({ name, email, password } as RegisterData);
      } else {
        onSubmit({ email, password } as LoginData);
      }
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      {isRegister && (
        <div>
          <label>Name</label>
          <input value={name} onChange={(e) => setName(e.target.value)} />
        </div>
      )}
      <div>
        <label>Email</label>
        <input type="email" value={email} onChange={(e) => setEmail(e.target.value)} />
      </div>
      <div>
        <label>Password</label>
        <input type="password" value={password} onChange={(e) => setPassword(e.target.value)} />
      </div>
      {errors.length > 0 && (
        <ul>
          {errors.map((er, idx) => (
            <li key={idx} style={{ color: "red" }}>
              {er}
            </li>
          ))}
        </ul>
      )}
      <button type="submit">{isRegister ? "Register" : "Login"}</button>
    </form>
  );
}
