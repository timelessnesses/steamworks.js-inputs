export function init(appId?: number, callbackDurationFactory?: ((arg0: Omit<Client, "init" | "runCallbacks">) => number) | undefined): Omit<Client, "init" | "runCallbacks">;
export function shutdown(): void;
export function restartAppIfNecessary(appId: number): boolean;
export function electronEnableSteamOverlay(disableEachFrameInvalidation?: boolean): void;
export type Client = typeof import("./client.d");
export const SteamCallback: typeof import("./client.d").callback.SteamCallback;
