export interface FriendInfo {
  name: string
  nickName?: string
  smallAvatar?: Buffer
  mediumAvatar?: Buffer
  largeAvatar?: Buffer
}

export declare function init(appId?: number | undefined | null): void

export interface PlayerSteamId {
  steamId64: bigint
  steamId32: string
  accountId: number
}

export declare function restartAppIfNecessary(appId: number): boolean

export declare function runCallbacks(): void

/**
 * Shutdown the Steam client instance
 * PLEASE DON'T USE THIS. Use [`shutdown`] instead.
 */
export declare function shutdownClient(): void

export declare namespace achievement {
  export function activate(achievement: string): boolean
  export function clear(achievement: string): boolean
  export function isActivated(achievement: string): boolean
  export function names(): Array<string>
}

export declare namespace apps {
  export function appBuildId(): number
  export function appInstallDir(appId: number): string
  export function appOwner(): PlayerSteamId
  export function availableGameLanguages(): Array<string>
  export function currentBetaName(): string | null
  export function currentGameLanguage(): string
  export function isAppInstalled(appId: number): boolean
  export function isCybercafe(): boolean
  export function isDlcInstalled(appId: number): boolean
  export function isLowViolence(): boolean
  export function isSubscribed(): boolean
  export function isSubscribedApp(appId: number): boolean
  export function isSubscribedFromFreeWeekend(): boolean
  export function isVacBanned(): boolean
}

export declare namespace auth {
  export class Ticket {
    cancel(): void
    getBytes(): Buffer
  }
  export function getAuthTicketForWebApi(identity: string, timeoutSeconds?: number | undefined | null): Promise<Ticket>
  /**
   * @param ip - The string of IPv4 or IPv6 address. Use as NetworkIdentity of the remote system that will authenticate the ticket.
   * @param timeoutSeconds - The number of seconds to wait for the ticket to be validated. Default value is 10 seconds.
   */
  export function getSessionTicketWithIp(ip: string, timeoutSeconds?: number | undefined | null): Promise<Ticket>
  /**
   * @param steamId64 - The user steam id or game server steam id. Use as NetworkIdentity of the remote system that will authenticate the ticket. If it is peer-to-peer then the user steam ID. If it is a game server, then the game server steam ID may be used if it was obtained from a trusted 3rd party
   * @param timeoutSeconds - The number of seconds to wait for the ticket to be validated. Default value is 10 seconds.
   */
  export function getSessionTicketWithSteamId(steamId64: bigint, timeoutSeconds?: number | undefined | null): Promise<Ticket>
}

export declare namespace callback {
  export class Handle {
    disconnect(): void
  }
  export function register<C extends keyof import('./callbacks').CallbackReturns>(steamCallback: C, handler: (value: import('./callbacks').CallbackReturns[C]) => void): Handle
  export const enum SteamCallback {
    PersonaStateChange = 0,
    SteamServersConnected = 1,
    SteamServersDisconnected = 2,
    SteamServerConnectFailure = 3,
    LobbyDataUpdate = 4,
    LobbyChatUpdate = 5,
    P2PSessionRequest = 6,
    P2PSessionConnectFail = 7,
    GameLobbyJoinRequested = 8,
    MicroTxnAuthorizationResponse = 9
  }
}

export declare namespace cloud {
  export function deleteFile(name: string): boolean
  export function fileExists(name: string): boolean
  export interface FileInfo {
    name: string
    size: bigint
  }
  export function isEnabledForAccount(): boolean
  export function isEnabledForApp(): boolean
  export function listFiles(): Array<FileInfo>
  export function readFile(name: string): string
  export function setEnabledForApp(enabled: boolean): void
  export function writeFile(name: string, content: string): boolean
}

export declare namespace friends {
  export function requestUserInformation(steamId: bigint, requireNameOnly: boolean, timeoutSeconds?: number | undefined | null): Promise<FriendInfo>
}

export declare namespace input {
  export class Controller {
    activateActionSet(actionSetHandle: bigint): boolean
    isDigitalActionPressed(actionHandle: bigint): boolean
    getAnalogActionVector(actionHandle: bigint): AnalogActionVector
    getType(): InputType
    getHandle(): bigint
    /** Gets controller's motion sensors */
    getMotionData(): MotionData
    /**
     * Triggers a vibration event
     * It has intensity from 0 (off) to 65535 (max)
     * use something like `setTimeout` to make a timed vibration
     */
    triggerVibration(leftSpeedMicroSecond: number, rightSpeedMicroSecond: number): void
    getAnalogActionOrigins(actionSetHandle: bigint, analogActionHandle: bigint): Array<InputActionOrigins>
    getDigitalActionOrigins(actionSetHandle: bigint, digitalActionHandle: bigint): Array<InputActionOrigins>
    getCurrentActiveActionSet(): bigint
  }
  export interface AnalogActionVector {
    x: number
    y: number
  }
  export function getActionHandle(actionName: string): bigint
  export function getActionSet(actionSetName: string): bigint
  export function getAnalogAction(actionName: string): bigint
  export function getControllers(): Array<Controller>
  export function getDigitalAction(actionName: string): bigint
  export function getFilePathForAction(actionHandle: InputActionOrigins): string
  export function init(): void
  /** The conversion will return 32767 if the value is not found */
  export const enum InputActionOrigins {
    None = 0,
    SteamControllerA = 1,
    SteamControllerB = 2,
    SteamControllerX = 3,
    SteamControllerY = 4,
    SteamControllerLeftBumper = 5,
    SteamControllerRightBumper = 6,
    SteamControllerLeftGrip = 7,
    SteamControllerRightGrip = 8,
    SteamControllerStart = 9,
    SteamControllerBack = 10,
    SteamControllerLeftPadTouch = 11,
    SteamControllerLeftPadSwipe = 12,
    SteamControllerLeftPadClick = 13,
    SteamControllerLeftPadDPadNorth = 14,
    SteamControllerLeftPadDPadSouth = 15,
    SteamControllerLeftPadDPadWest = 16,
    SteamControllerLeftPadDPadEast = 17,
    SteamControllerRightPadTouch = 18,
    SteamControllerRightPadSwipe = 19,
    SteamControllerRightPadClick = 20,
    SteamControllerRightPadDPadNorth = 21,
    SteamControllerRightPadDPadSouth = 22,
    SteamControllerRightPadDPadWest = 23,
    SteamControllerRightPadDPadEast = 24,
    SteamControllerLeftTriggerPull = 25,
    SteamControllerLeftTriggerClick = 26,
    SteamControllerRightTriggerPull = 27,
    SteamControllerRightTriggerClick = 28,
    SteamControllerLeftStickMove = 29,
    SteamControllerLeftStickClick = 30,
    SteamControllerLeftStickDPadNorth = 31,
    SteamControllerLeftStickDPadSouth = 32,
    SteamControllerLeftStickDPadWest = 33,
    SteamControllerLeftStickDPadEast = 34,
    SteamControllerGyroMove = 35,
    SteamControllerGyroPitch = 36,
    SteamControllerGyroYaw = 37,
    SteamControllerGyroRoll = 38,
    SteamControllerReserved0 = 39,
    SteamControllerReserved1 = 40,
    SteamControllerReserved2 = 41,
    SteamControllerReserved3 = 42,
    SteamControllerReserved4 = 43,
    SteamControllerReserved5 = 44,
    SteamControllerReserved6 = 45,
    SteamControllerReserved7 = 46,
    SteamControllerReserved8 = 47,
    SteamControllerReserved9 = 48,
    SteamControllerReserved10 = 49,
    PS4X = 50,
    PS4Circle = 51,
    PS4Triangle = 52,
    PS4Square = 53,
    PS4LeftBumper = 54,
    PS4RightBumper = 55,
    PS4Options = 56,
    PS4Share = 57,
    PS4LeftPadTouch = 58,
    PS4LeftPadSwipe = 59,
    PS4LeftPadClick = 60,
    PS4LeftPadDPadNorth = 61,
    PS4LeftPadDPadSouth = 62,
    PS4LeftPadDPadWest = 63,
    PS4LeftPadDPadEast = 64,
    PS4RightPadTouch = 65,
    PS4RightPadSwipe = 66,
    PS4RightPadClick = 67,
    PS4RightPadDPadNorth = 68,
    PS4RightPadDPadSouth = 69,
    PS4RightPadDPadWest = 70,
    PS4RightPadDPadEast = 71,
    PS4CenterPadTouch = 72,
    PS4CenterPadSwipe = 73,
    PS4CenterPadClick = 74,
    PS4CenterPadDPadNorth = 75,
    PS4CenterPadDPadSouth = 76,
    PS4CenterPadDPadWest = 77,
    PS4CenterPadDPadEast = 78,
    PS4LeftTriggerPull = 79,
    PS4LeftTriggerClick = 80,
    PS4RightTriggerPull = 81,
    PS4RightTriggerClick = 82,
    PS4LeftStickMove = 83,
    PS4LeftStickClick = 84,
    PS4LeftStickDPadNorth = 85,
    PS4LeftStickDPadSouth = 86,
    PS4LeftStickDPadWest = 87,
    PS4LeftStickDPadEast = 88,
    PS4RightStickMove = 89,
    PS4RightStickClick = 90,
    PS4RightStickDPadNorth = 91,
    PS4RightStickDPadSouth = 92,
    PS4RightStickDPadWest = 93,
    PS4RightStickDPadEast = 94,
    PS4DPadNorth = 95,
    PS4DPadSouth = 96,
    PS4DPadWest = 97,
    PS4DPadEast = 98,
    PS4GyroMove = 99,
    PS4GyroPitch = 100,
    PS4GyroYaw = 101,
    PS4GyroRoll = 102,
    PS4DPadMove = 103,
    PS4Reserved1 = 104,
    PS4Reserved2 = 105,
    PS4Reserved3 = 106,
    PS4Reserved4 = 107,
    PS4Reserved5 = 108,
    PS4Reserved6 = 109,
    PS4Reserved7 = 110,
    PS4Reserved8 = 111,
    PS4Reserved9 = 112,
    PS4Reserved10 = 113,
    XBoxOneA = 114,
    XBoxOneB = 115,
    XBoxOneX = 116,
    XBoxOneY = 117,
    XBoxOneLeftBumper = 118,
    XBoxOneRightBumper = 119,
    XBoxOneMenu = 120,
    XBoxOneView = 121,
    XBoxOneLeftTriggerPull = 122,
    XBoxOneLeftTriggerClick = 123,
    XBoxOneRightTriggerPull = 124,
    XBoxOneRightTriggerClick = 125,
    XBoxOneLeftStickMove = 126,
    XBoxOneLeftStickClick = 127,
    XBoxOneLeftStickDPadNorth = 128,
    XBoxOneLeftStickDPadSouth = 129,
    XBoxOneLeftStickDPadWest = 130,
    XBoxOneLeftStickDPadEast = 131,
    XBoxOneRightStickMove = 132,
    XBoxOneRightStickClick = 133,
    XBoxOneRightStickDPadNorth = 134,
    XBoxOneRightStickDPadSouth = 135,
    XBoxOneRightStickDPadWest = 136,
    XBoxOneRightStickDPadEast = 137,
    XBoxOneDPadNorth = 138,
    XBoxOneDPadSouth = 139,
    XBoxOneDPadWest = 140,
    XBoxOneDPadEast = 141,
    XBoxOneDPadMove = 142,
    XBoxOneLeftGripLower = 143,
    XBoxOneLeftGripUpper = 144,
    XBoxOneRightGripLower = 145,
    XBoxOneRightGripUpper = 146,
    XBoxOneShare = 147,
    XBoxOneReserved6 = 148,
    XBoxOneReserved7 = 149,
    XBoxOneReserved8 = 150,
    XBoxOneReserved9 = 151,
    XBoxOneReserved10 = 152,
    XBox360A = 153,
    XBox360B = 154,
    XBox360X = 155,
    XBox360Y = 156,
    XBox360LeftBumper = 157,
    XBox360RightBumper = 158,
    XBox360Start = 159,
    XBox360Back = 160,
    XBox360LeftTriggerPull = 161,
    XBox360LeftTriggerClick = 162,
    XBox360RightTriggerPull = 163,
    XBox360RightTriggerClick = 164,
    XBox360LeftStickMove = 165,
    XBox360LeftStickClick = 166,
    XBox360LeftStickDPadNorth = 167,
    XBox360LeftStickDPadSouth = 168,
    XBox360LeftStickDPadWest = 169,
    XBox360LeftStickDPadEast = 170,
    XBox360RightStickMove = 171,
    XBox360RightStickClick = 172,
    XBox360RightStickDPadNorth = 173,
    XBox360RightStickDPadSouth = 174,
    XBox360RightStickDPadWest = 175,
    XBox360RightStickDPadEast = 176,
    XBox360DPadNorth = 177,
    XBox360DPadSouth = 178,
    XBox360DPadWest = 179,
    XBox360DPadEast = 180,
    XBox360DPadMove = 181,
    XBox360Reserved1 = 182,
    XBox360Reserved2 = 183,
    XBox360Reserved3 = 184,
    XBox360Reserved4 = 185,
    XBox360Reserved5 = 186,
    XBox360Reserved6 = 187,
    XBox360Reserved7 = 188,
    XBox360Reserved8 = 189,
    XBox360Reserved9 = 190,
    XBox360Reserved10 = 191,
    SwitchA = 192,
    SwitchB = 193,
    SwitchX = 194,
    SwitchY = 195,
    SwitchLeftBumper = 196,
    SwitchRightBumper = 197,
    SwitchPlus = 198,
    SwitchMinus = 199,
    SwitchCapture = 200,
    SwitchLeftTriggerPull = 201,
    SwitchLeftTriggerClick = 202,
    SwitchRightTriggerPull = 203,
    SwitchRightTriggerClick = 204,
    SwitchLeftStickMove = 205,
    SwitchLeftStickClick = 206,
    SwitchLeftStickDPadNorth = 207,
    SwitchLeftStickDPadSouth = 208,
    SwitchLeftStickDPadWest = 209,
    SwitchLeftStickDPadEast = 210,
    SwitchRightStickMove = 211,
    SwitchRightStickClick = 212,
    SwitchRightStickDPadNorth = 213,
    SwitchRightStickDPadSouth = 214,
    SwitchRightStickDPadWest = 215,
    SwitchRightStickDPadEast = 216,
    SwitchDPadNorth = 217,
    SwitchDPadSouth = 218,
    SwitchDPadWest = 219,
    SwitchDPadEast = 220,
    SwitchProGyroMove = 221,
    SwitchProGyroPitch = 222,
    SwitchProGyroYaw = 223,
    SwitchProGyroRoll = 224,
    SwitchDPadMove = 225,
    SwitchReserved1 = 226,
    SwitchReserved2 = 227,
    SwitchReserved3 = 228,
    SwitchReserved4 = 229,
    SwitchReserved5 = 230,
    SwitchReserved6 = 231,
    SwitchReserved7 = 232,
    SwitchReserved8 = 233,
    SwitchReserved9 = 234,
    SwitchReserved10 = 235,
    SwitchRightGyroMove = 236,
    SwitchRightGyroPitch = 237,
    SwitchRightGyroYaw = 238,
    SwitchRightGyroRoll = 239,
    SwitchLeftGyroMove = 240,
    SwitchLeftGyroPitch = 241,
    SwitchLeftGyroYaw = 242,
    SwitchLeftGyroRoll = 243,
    SwitchLeftGripLower = 244,
    SwitchLeftGripUpper = 245,
    SwitchRightGripLower = 246,
    SwitchRightGripUpper = 247,
    SwitchJoyConButtonN = 248,
    SwitchJoyConButtonE = 249,
    SwitchJoyConButtonS = 250,
    SwitchJoyConButtonW = 251,
    SwitchReserved15 = 252,
    SwitchReserved16 = 253,
    SwitchReserved17 = 254,
    SwitchReserved18 = 255,
    SwitchReserved19 = 256,
    SwitchReserved20 = 257,
    PS5X = 258,
    PS5Circle = 259,
    PS5Triangle = 260,
    PS5Square = 261,
    PS5LeftBumper = 262,
    PS5RightBumper = 263,
    PS5Option = 264,
    PS5Create = 265,
    PS5Mute = 266,
    PS5LeftPadTouch = 267,
    PS5LeftPadSwipe = 268,
    PS5LeftPadClick = 269,
    PS5LeftPadDPadNorth = 270,
    PS5LeftPadDPadSouth = 271,
    PS5LeftPadDPadWest = 272,
    PS5LeftPadDPadEast = 273,
    PS5RightPadTouch = 274,
    PS5RightPadSwipe = 275,
    PS5RightPadClick = 276,
    PS5RightPadDPadNorth = 277,
    PS5RightPadDPadSouth = 278,
    PS5RightPadDPadWest = 279,
    PS5RightPadDPadEast = 280,
    PS5CenterPadTouch = 281,
    PS5CenterPadSwipe = 282,
    PS5CenterPadClick = 283,
    PS5CenterPadDPadNorth = 284,
    PS5CenterPadDPadSouth = 285,
    PS5CenterPadDPadWest = 286,
    PS5CenterPadDPadEast = 287,
    PS5LeftTriggerPull = 288,
    PS5LeftTriggerClick = 289,
    PS5RightTriggerPull = 290,
    PS5RightTriggerClick = 291,
    PS5LeftStickMove = 292,
    PS5LeftStickClick = 293,
    PS5LeftStickDPadNorth = 294,
    PS5LeftStickDPadSouth = 295,
    PS5LeftStickDPadWest = 296,
    PS5LeftStickDPadEast = 297,
    PS5RightStickMove = 298,
    PS5RightStickClick = 299,
    PS5RightStickDPadNorth = 300,
    PS5RightStickDPadSouth = 301,
    PS5RightStickDPadWest = 302,
    PS5RightStickDPadEast = 303,
    PS5DPadNorth = 304,
    PS5DPadSouth = 305,
    PS5DPadWest = 306,
    PS5DPadEast = 307,
    PS5GyroMove = 308,
    PS5GyroPitch = 309,
    PS5GyroYaw = 310,
    PS5GyroRoll = 311,
    PS5DPadMove = 312,
    PS5LeftGrip = 313,
    PS5RightGrip = 314,
    PS5LeftFn = 315,
    PS5RightFn = 316,
    PS5Reserved5 = 317,
    PS5Reserved6 = 318,
    PS5Reserved7 = 319,
    PS5Reserved8 = 320,
    PS5Reserved9 = 321,
    PS5Reserved10 = 322,
    PS5Reserved11 = 323,
    PS5Reserved12 = 324,
    PS5Reserved13 = 325,
    PS5Reserved14 = 326,
    PS5Reserved15 = 327,
    PS5Reserved16 = 328,
    PS5Reserved17 = 329,
    PS5Reserved18 = 330,
    PS5Reserved19 = 331,
    PS5Reserved20 = 332,
    SteamDeckA = 333,
    SteamDeckB = 334,
    SteamDeckX = 335,
    SteamDeckY = 336,
    SteamDeckL1 = 337,
    SteamDeckR1 = 338,
    SteamDeckMenu = 339,
    SteamDeckView = 340,
    SteamDeckLeftPadTouch = 341,
    SteamDeckLeftPadSwipe = 342,
    SteamDeckLeftPadClick = 343,
    SteamDeckLeftPadDPadNorth = 344,
    SteamDeckLeftPadDPadSouth = 345,
    SteamDeckLeftPadDPadWest = 346,
    SteamDeckLeftPadDPadEast = 347,
    SteamDeckRightPadTouch = 348,
    SteamDeckRightPadSwipe = 349,
    SteamDeckRightPadClick = 350,
    SteamDeckRightPadDPadNorth = 351,
    SteamDeckRightPadDPadSouth = 352,
    SteamDeckRightPadDPadWest = 353,
    SteamDeckRightPadDPadEast = 354,
    SteamDeckL2SoftPull = 355,
    SteamDeckL2 = 356,
    SteamDeckR2SoftPull = 357,
    SteamDeckR2 = 358,
    SteamDeckLeftStickMove = 359,
    SteamDeckL3 = 360,
    SteamDeckLeftStickDPadNorth = 361,
    SteamDeckLeftStickDPadSouth = 362,
    SteamDeckLeftStickDPadWest = 363,
    SteamDeckLeftStickDPadEast = 364,
    SteamDeckLeftStickTouch = 365,
    SteamDeckRightStickMove = 366,
    SteamDeckR3 = 367,
    SteamDeckRightStickDPadNorth = 368,
    SteamDeckRightStickDPadSouth = 369,
    SteamDeckRightStickDPadWest = 370,
    SteamDeckRightStickDPadEast = 371,
    SteamDeckRightStickTouch = 372,
    SteamDeckL4 = 373,
    SteamDeckR4 = 374,
    SteamDeckL5 = 375,
    SteamDeckR5 = 376,
    SteamDeckDPadMove = 377,
    SteamDeckDPadNorth = 378,
    SteamDeckDPadSouth = 379,
    SteamDeckDPadWest = 380,
    SteamDeckDPadEast = 381,
    SteamDeckGyroMove = 382,
    SteamDeckGyroPitch = 383,
    SteamDeckGyroYaw = 384,
    SteamDeckGyroRoll = 385,
    SteamDeckReserved1 = 386,
    SteamDeckReserved2 = 387,
    SteamDeckReserved3 = 388,
    SteamDeckReserved4 = 389,
    SteamDeckReserved5 = 390,
    SteamDeckReserved6 = 391,
    SteamDeckReserved7 = 392,
    SteamDeckReserved8 = 393,
    SteamDeckReserved9 = 394,
    SteamDeckReserved10 = 395,
    SteamDeckReserved11 = 396,
    SteamDeckReserved12 = 397,
    SteamDeckReserved13 = 398,
    SteamDeckReserved14 = 399,
    SteamDeckReserved15 = 400,
    SteamDeckReserved16 = 401,
    SteamDeckReserved17 = 402,
    SteamDeckReserved18 = 403,
    SteamDeckReserved19 = 404,
    SteamDeckReserved20 = 405,
    HoripadM1 = 406,
    HoripadM2 = 407,
    HoripadL4 = 408,
    HoripadR4 = 409,
    Count = 410,
    MaximumPossibleValue = 32767
  }
  export const enum InputType {
    Unknown = 'Unknown',
    SteamController = 'SteamController',
    XBox360Controller = 'XBox360Controller',
    XBoxOneController = 'XBoxOneController',
    GenericGamepad = 'GenericGamepad',
    PS4Controller = 'PS4Controller',
    AppleMFiController = 'AppleMFiController',
    AndroidController = 'AndroidController',
    SwitchJoyConPair = 'SwitchJoyConPair',
    SwitchJoyConSingle = 'SwitchJoyConSingle',
    SwitchProController = 'SwitchProController',
    MobileTouch = 'MobileTouch',
    PS3Controller = 'PS3Controller',
    PS5Controller = 'PS5Controller',
    SteamDeckController = 'SteamDeckController'
  }
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
  export function runFrame(): void
  export function setInputActionManifestFilePath(path: string): boolean
  export function shutdown(): void
  export const enum VibrateSide {
    Left = 0,
    Right = 1
  }
}

export declare namespace localplayer {
  /** @returns the 2 digit ISO 3166-1-alpha-2 format country code which client is running in, e.g. "US" or "UK". */
  export function getIpCountry(): string
  export function getLevel(): number
  export function getName(): string
  export function getSteamId(): PlayerSteamId
  export function setRichPresence(key: string, value?: string | undefined | null): void
}

export declare namespace matchmaking {
  export class Lobby {
    join(): Promise<Lobby>
    leave(): void
    openInviteDialog(): void
    getMemberCount(): bigint
    getMemberLimit(): bigint | null
    getMembers(): Array<PlayerSteamId>
    getOwner(): PlayerSteamId
    setJoinable(joinable: boolean): boolean
    getData(key: string): string | null
    setData(key: string, value: string): boolean
    deleteData(key: string): boolean
    /** Get an object containing all the lobby data */
    getFullData(): Record<string, string>
    /**
     * Merge current lobby data with provided data in a single batch
     * @returns true if all data was set successfully
     */
    mergeFullData(data: Record<string, string>): boolean
    get id(): bigint
    get idAsU64(): bigint
  }
  export function createLobby(lobbyType: LobbyType, maxMembers: number): Promise<Lobby>
  export function getLobbies(): Promise<Array<Lobby>>
  export function joinLobby(lobbyId: bigint): Promise<Lobby>
  export const enum LobbyType {
    Private = 0,
    FriendsOnly = 1,
    Public = 2,
    Invisible = 3
  }
}

export declare namespace networking {
  export function acceptP2PSession(steamId64: bigint): void
  export function isP2PPacketAvailable(): number
  export interface P2PPacket {
    data: Buffer
    size: number
    steamId: PlayerSteamId
  }
  export function readP2PPacket(size: number): P2PPacket
  export function sendP2PPacket(steamId64: bigint, sendType: SendType, data: Buffer): boolean
  /** The method used to send a packet */
  export const enum SendType {
    /**
     * Send the packet directly over udp.
     *
     * Can't be larger than 1200 bytes
     */
    Unreliable = 0,
    /**
     * Like `Unreliable` but doesn't buffer packets
     * sent before the connection has started.
     */
    UnreliableNoDelay = 1,
    /**
     * Reliable packet sending.
     *
     * Can't be larger than 1 megabyte.
     */
    Reliable = 2,
    /**
     * Like `Reliable` but applies the nagle
     * algorithm to packets being sent
     */
    ReliableWithBuffering = 3
  }
}

export declare namespace overlay {
  export function activateDialog(dialog: Dialog): void
  export function activateDialogToUser(dialog: Dialog, steamId64: bigint): void
  export function activateInviteDialog(lobbyId: bigint): void
  export function activateToStore(appId: number, flag: StoreFlag): void
  export function activateToWebPage(url: string): void
  export const enum Dialog {
    Friends = 0,
    Community = 1,
    Players = 2,
    Settings = 3,
    OfficialGameGroup = 4,
    Stats = 5,
    Achievements = 6
  }
  export const enum StoreFlag {
    None = 0,
    AddToCart = 1,
    AddToCartAndShow = 2
  }
}

export declare namespace stats {
  export function getInt(name: string): number | null
  export function resetAll(achievementsToo: boolean): boolean
  export function setInt(name: string, value: number): boolean
  export function store(): boolean
}

export declare namespace utils {
  export const enum FloatingGamepadTextInputMode {
    SingleLine = 0,
    MultipleLines = 1,
    Email = 2,
    Numeric = 3
  }
  export const enum GamepadTextInputLineMode {
    SingleLine = 0,
    MultipleLines = 1
  }
  export const enum GamepadTextInputMode {
    Normal = 0,
    Password = 1
  }
  export function getAppId(): number
  export function getServerRealTime(): number
  export function isSteamRunningOnSteamDeck(): boolean
  /** @returns true if the floating keyboard was shown, otherwise, false */
  export function showFloatingGamepadTextInput(keyboardMode: FloatingGamepadTextInputMode, x: number, y: number, width: number, height: number): Promise<boolean>
  /** @returns the entered text, or null if cancelled or could not show the input */
  export function showGamepadTextInput(inputMode: GamepadTextInputMode, inputLineMode: GamepadTextInputLineMode, description: string, maxCharacters: number, existingText?: string | undefined | null): Promise<string | null>
}

export declare namespace workshop {
  export interface AppIDs {
    creator?: number
    consumer?: number
  }
  export function createItem(appId?: number | undefined | null): Promise<UgcResult>
  export function deleteItem(itemId: bigint): Promise<void>
  /**
   * Download or update a workshop item.
   *
   * @param highPriority - If high priority is true, start the download in high priority mode, pausing any existing in-progress Steam downloads and immediately begin downloading this workshop item.
   * @returns true or false
   *
   * {@link https://partner.steamgames.com/doc/api/ISteamUGC#DownloadItem}
   */
  export function download(itemId: bigint, highPriority: boolean): boolean
  /**
   * Get info about a pending download of a workshop item.
   *
   * @returns an object with the properties {current, total}
   *
   * {@link https://partner.steamgames.com/doc/api/ISteamUGC#GetItemDownloadInfo}
   */
  export function downloadInfo(itemId: bigint): DownloadInfo | null
  export interface DownloadInfo {
    current: bigint
    total: bigint
  }
  export function getAllItems(page: number, queryType: UGCQueryType, itemType: UGCType, creatorAppId: number, consumerAppId: number, queryConfig?: WorkshopItemQueryConfig | undefined | null): Promise<WorkshopPaginatedResult>
  export function getItem(item: bigint, queryConfig?: WorkshopItemQueryConfig | undefined | null): Promise<WorkshopItem | null>
  export function getItems(items: Array<bigint>, queryConfig?: WorkshopItemQueryConfig | undefined | null): Promise<WorkshopItemsResult>
  /**
   * Get all subscribed workshop items.
   * @returns an array of subscribed workshop item ids
   */
  export function getSubscribedItems(): Array<bigint>
  export function getUserItems(page: number, accountId: number, listType: UserListType, itemType: UGCType, sortOrder: UserListOrder, appIds: AppIDs, queryConfig?: WorkshopItemQueryConfig | undefined | null): Promise<WorkshopPaginatedResult>
  /**
   * Gets info about currently installed content on the disc for workshop item.
   *
   * @returns an object with the the properties {folder, size_on_disk, timestamp}
   *
   * {@link https://partner.steamgames.com/doc/api/ISteamUGC#GetItemInstallInfo}
   */
  export function installInfo(itemId: bigint): InstallInfo | null
  export interface InstallInfo {
    folder: string
    sizeOnDisk: bigint
    timestamp: number
  }
  /**
   * Gets the current state of a workshop item on this client. States can be combined.
   *
   * @returns a number with the current item state, e.g. 9
   * 9 = 1 (The current user is subscribed to this item) + 8 (The item needs an update)
   *
   * {@link https://partner.steamgames.com/doc/api/ISteamUGC#GetItemState}
   * {@link https://partner.steamgames.com/doc/api/ISteamUGC#EItemState}
   */
  export function state(itemId: bigint): number
  /**
   * Subscribe to a workshop item. It will be downloaded and installed as soon as possible.
   *
   * {@link https://partner.steamgames.com/doc/api/ISteamUGC#SubscribeItem}
   */
  export function subscribe(itemId: bigint): Promise<void>
  export const enum UgcItemVisibility {
    Public = 0,
    FriendsOnly = 1,
    Private = 2,
    Unlisted = 3
  }
  export const enum UGCQueryType {
    RankedByVote = 0,
    RankedByPublicationDate = 1,
    AcceptedForGameRankedByAcceptanceDate = 2,
    RankedByTrend = 3,
    FavoritedByFriendsRankedByPublicationDate = 4,
    CreatedByFriendsRankedByPublicationDate = 5,
    RankedByNumTimesReported = 6,
    CreatedByFollowedUsersRankedByPublicationDate = 7,
    NotYetRated = 8,
    RankedByTotalVotesAsc = 9,
    RankedByVotesUp = 10,
    RankedByTextSearch = 11,
    RankedByTotalUniqueSubscriptions = 12,
    RankedByPlaytimeTrend = 13,
    RankedByTotalPlaytime = 14,
    RankedByAveragePlaytimeTrend = 15,
    RankedByLifetimeAveragePlaytime = 16,
    RankedByPlaytimeSessionsTrend = 17,
    RankedByLifetimePlaytimeSessions = 18,
    RankedByLastUpdatedDate = 19
  }
  export interface UgcResult {
    itemId: bigint
    needsToAcceptAgreement: boolean
  }
  export const enum UGCType {
    Items = 0,
    ItemsMtx = 1,
    ItemsReadyToUse = 2,
    Collections = 3,
    Artwork = 4,
    Videos = 5,
    Screenshots = 6,
    AllGuides = 7,
    WebGuides = 8,
    IntegratedGuides = 9,
    UsableInGame = 10,
    ControllerBindings = 11,
    GameManagedItems = 12,
    All = 13
  }
  export interface UgcUpdate {
    title?: string
    description?: string
    changeNote?: string
    previewPath?: string
    contentPath?: string
    tags?: Array<string>
    visibility?: UgcItemVisibility
  }
  /**
   * Unsubscribe from a workshop item. This will result in the item being removed after the game quits.
   *
   * {@link https://partner.steamgames.com/doc/api/ISteamUGC#UnsubscribeItem}
   */
  export function unsubscribe(itemId: bigint): Promise<void>
  export function updateItem(itemId: bigint, updateDetails: UgcUpdate, appId?: number | undefined | null): Promise<UgcResult>
  export function updateItemWithCallback(itemId: bigint, updateDetails: UgcUpdate, appId: number | undefined | null, successCallback: (data: UgcResult) => void, errorCallback: (err: any) => void, progressCallback?: (data: UpdateProgress) => void, progressCallbackIntervalMs?: number | undefined | null): void
  export interface UpdateProgress {
    status: UpdateStatus
    progress: bigint
    total: bigint
  }
  export const enum UpdateStatus {
    Invalid = 0,
    PreparingConfig = 1,
    PreparingContent = 2,
    UploadingContent = 3,
    UploadingPreviewFile = 4,
    CommittingChanges = 5
  }
  export const enum UserListOrder {
    CreationOrderAsc = 0,
    CreationOrderDesc = 1,
    TitleAsc = 2,
    LastUpdatedDesc = 3,
    SubscriptionDateDesc = 4,
    VoteScoreDesc = 5,
    ForModeration = 6
  }
  export const enum UserListType {
    Published = 0,
    VotedOn = 1,
    VotedUp = 2,
    VotedDown = 3,
    Favorited = 4,
    Subscribed = 5,
    UsedOrPlayed = 6,
    Followed = 7
  }
  export interface WorkshopItem {
    publishedFileId: bigint
    creatorAppId?: number
    consumerAppId?: number
    title: string
    description: string
    owner: PlayerSteamId
    /** Time created in unix epoch seconds format */
    timeCreated: number
    /** Time updated in unix epoch seconds format */
    timeUpdated: number
    /** Time when the user added the published item to their list (not always applicable), provided in Unix epoch format (time since Jan 1st, 1970). */
    timeAddedToUserList: number
    visibility: UgcItemVisibility
    banned: boolean
    acceptedForUse: boolean
    tags: Array<string>
    tagsTruncated: boolean
    url: string
    numUpvotes: number
    numDownvotes: number
    numChildren: number
    previewUrl?: string
    statistics: WorkshopItemStatistic
  }
  export interface WorkshopItemQueryConfig {
    cachedResponseMaxAge?: number
    includeMetadata?: boolean
    includeLongDescription?: boolean
    includeAdditionalPreviews?: boolean
    onlyIds?: boolean
    onlyTotal?: boolean
    language?: string
    matchAnyTag?: boolean
    requiredTags?: Array<string>
    excludedTags?: Array<string>
    searchText?: string
    rankedByTrendDays?: number
  }
  export interface WorkshopItemsResult {
    items: Array<WorkshopItem | undefined | null>
    wasCached: boolean
  }
  export interface WorkshopItemStatistic {
    numSubscriptions?: bigint
    numFavorites?: bigint
    numFollowers?: bigint
    numUniqueSubscriptions?: bigint
    numUniqueFavorites?: bigint
    numUniqueFollowers?: bigint
    numUniqueWebsiteViews?: bigint
    reportScore?: bigint
    numSecondsPlayed?: bigint
    numPlaytimeSessions?: bigint
    numComments?: bigint
    numSecondsPlayedDuringTimePeriod?: bigint
    numPlaytimeSessionsDuringTimePeriod?: bigint
  }
  export interface WorkshopPaginatedResult {
    items: Array<WorkshopItem | undefined | null>
    returnedResults: number
    totalResults: number
    wasCached: boolean
  }
}
