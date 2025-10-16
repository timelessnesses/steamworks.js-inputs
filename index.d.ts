export function init(appId?: number): Omit<Client, "init" | "runCallbacks">;
export function restartAppIfNecessary(appId: number): boolean;
export function electronEnableSteamOverlay(disableEachFrameInvalidation?: boolean): void;
export type Client = typeof import("./client.d");
export const SteamCallback: typeof import("./client.d").callback.SteamCallback;
export const Ticket: typeof import("./client.d").auth.Ticket;
export const Handle: typeof import("./client.d").callback.Handle;
export interface FileInfo {
    name: string;
    size: bigint;
}
export const Controller: typeof import("./client.d").input.Controller;
export interface AnalogActionVector {
    x: number;
    y: number;
}
export const InputActionOrigins: typeof import("./client.d").input.InputActionOrigins;
export const InputType: typeof import("./client.d").input.InputType;
export interface MotionData { 
    /** Absolute Rotation (drift) X axis */
    rotQuatX: number
    /** Absolute Rotation (drift) Y axis */
    rotQuatY: number
    /** Absolute Rotation (drift) Z axis */
    rotQuatZ: number
    /** Absolute Rotation (drift) W axis */
    rotQuatW: number
    /** Positional Acceleration X axis */
    posAccelX: number
    /** Positional Acceleration Y axis */
    posAccelY: number
    /** Positional Acceleration Z axis */
    posAccelZ: number
    /** Rotational Velocity X axis */
    rotVelX: number
    /** Rotational Velocity Y axis */
    rotVelY: number
    /** Rotational Velocity Z axis */
    rotVelZ: number
}

export interface PlayerSteamId {
    steamId64: bigint
    steamId32: string
    accountId: number
}

export const Lobby: typeof import("./client.d").matchmaking.Lobby;
export const LobbyType: typeof import("./client.d").matchmaking.LobbyType;
export interface P2PPacket {
    data: Buffer
    size: number
    steamId: PlayerSteamId
}
export const SendType: typeof import("./client.d").networking.SendType;
export const Dialog: typeof import("./client.d").overlay.Dialog;
export const StoreFlag: typeof import("./client.d").overlay.StoreFlag;
