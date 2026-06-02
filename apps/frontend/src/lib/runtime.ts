import { ManagedRuntime } from "effect";
import * as BrowserHttpClient from "@effect/platform-browser/BrowserHttpClient";

export const runtime = ManagedRuntime.make(BrowserHttpClient.layerXMLHttpRequest);
