import { ManagedRuntime } from "effect";
import { BrowserHttpClient } from "@effect/platform-browser";

export const runtime = ManagedRuntime.make(BrowserHttpClient);
