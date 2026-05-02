import { fetch as tauriFetch } from '@tauri-apps/plugin-http';

/**
 * Custom `fetch` function implementation for SDK clients.
 * Uses Tauri's HTTP plugin in the desktop app to bypass CORS restrictions.
 * When `undefined`, SDKs fall back to the global `fetch`.
 */
export const customFetch = window.__TAURI_INTERNALS__
	? async (input: RequestInfo | URL, init?: RequestInit): Promise<Response> => {
			// tauri-plugin-http fetch has known issues serializing File/Blob objects inside FormData.
			// When uploading audio, we must fall back to the browser's native fetch.
			// Since official APIs (OpenAI, Groq) support CORS, this works correctly.
			if (init?.body instanceof FormData) {
				let hasFile = false;
				for (const value of init.body.values()) {
					if (value instanceof File || value instanceof Blob) {
						hasFile = true;
						break;
					}
				}
				if (hasFile) {
					return window.fetch(input, init);
				}
			}
			return tauriFetch(input, init);
		}
	: undefined;
