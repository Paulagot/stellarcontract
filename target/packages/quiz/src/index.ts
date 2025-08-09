import { Buffer } from "buffer";
import { Address } from '@stellar/stellar-sdk';
import {
  AssembledTransaction,
  Client as ContractClient,
  ClientOptions as ContractClientOptions,
  MethodOptions,
  Result,
  Spec as ContractSpec,
} from '@stellar/stellar-sdk/contract';
import type {
  u32,
  i32,
  u64,
  i64,
  u128,
  i128,
  u256,
  i256,
  Option,
  Typepoint,
  Duration,
} from '@stellar/stellar-sdk/contract';
export * from '@stellar/stellar-sdk'
export * as contract from '@stellar/stellar-sdk/contract'
export * as rpc from '@stellar/stellar-sdk/rpc'

if (typeof window !== 'undefined') {
  //@ts-ignore Buffer exists
  window.Buffer = window.Buffer || Buffer;
}


export const networks = {
  testnet: {
    networkPassphrase: "Test SDF Network ; September 2015",
    contractId: "CCJNWS5X3WHQNPXVVMRJV2SHCS43Y744E22V5QNI7K6OFXICKXPRNYYM",
  }
} as const

export type PrizeMode = {tag: "PrizePoolSplit", values: void} | {tag: "AssetBased", values: void};


export interface PrizeAsset {
  amount: i128;
  contract_id: string;
}


export interface PlayerEntry {
  entry_paid: i128;
  extras_paid: i128;
  join_ledger: u32;
  player: string;
  screen_name: string;
  total_paid: i128;
}


export interface RoomConfig {
  charity_bps: u32;
  creation_ledger: u32;
  ended: boolean;
  entry_fee: i128;
  fee_token: string;
  host: string;
  host_fee_bps: u32;
  host_wallet: Option<string>;
  player_addresses: Array<string>;
  players: Array<PlayerEntry>;
  prize_assets: Array<Option<PrizeAsset>>;
  prize_distribution: Array<u32>;
  prize_mode: PrizeMode;
  prize_pool_bps: u32;
  room_id: Buffer;
  total_entry_fees: i128;
  total_extras_fees: i128;
  total_paid_out: i128;
  total_pool: i128;
  winners: Array<string>;
}

export const QuizError = {
  1: {message:"InvalidHostFee"},
  2: {message:"MissingHostWallet"},
  3: {message:"InvalidPrizeSplit"},
  4: {message:"CharityBelowMinimum"},
  5: {message:"InvalidPrizePoolBps"},
  6: {message:"MissingPrizePoolConfig"},
  7: {message:"MissingPrizeAssets"},
  8: {message:"InvalidPrizeAssets"},
  9: {message:"InvalidTotalAllocation"},
  10: {message:"InvalidFeeToken"},
  11: {message:"RoomAlreadyExists"},
  12: {message:"RoomNotFound"},
  15: {message:"RoomAlreadyEnded"},
  16: {message:"PlayerAlreadyJoined"},
  17: {message:"InsufficientPayment"},
  18: {message:"Unauthorized"},
  19: {message:"InvalidWinners"},
  20: {message:"AssetTransferFailed"},
  21: {message:"InsufficientPlayers"},
  22: {message:"InsufficientAssets"},
  23: {message:"DepositFailed"},
  24: {message:"ScreenNameTaken"},
  25: {message:"InvalidScreenName"}
}

export interface Client {
  /**
   * Construct and simulate a init_pool_room transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  init_pool_room: ({room_id, host, fee_token, entry_fee, host_fee_bps, prize_pool_bps, first_place_pct, second_place_pct, third_place_pct}: {room_id: u32, host: string, fee_token: string, entry_fee: i128, host_fee_bps: Option<u32>, prize_pool_bps: u32, first_place_pct: u32, second_place_pct: Option<u32>, third_place_pct: Option<u32>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a init_asset_room transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  init_asset_room: ({room_id, host, fee_token, entry_fee, host_fee_bps, prizes}: {room_id: u32, host: string, fee_token: string, entry_fee: i128, host_fee_bps: Option<u32>, prizes: Array<PrizeAsset>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a join_room transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  join_room: ({room_id, player, screen_name, extras_amount}: {room_id: u32, player: string, screen_name: string, extras_amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a end_room transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  end_room: ({room_id, first_place, second_place, third_place}: {room_id: u32, first_place: Option<string>, second_place: Option<string>, third_place: Option<string>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a end_room_by_screen_names transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  end_room_by_screen_names: ({room_id, first_place_name, second_place_name, third_place_name}: {room_id: u32, first_place_name: Option<string>, second_place_name: Option<string>, third_place_name: Option<string>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a get_room_players transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_room_players: ({room_id}: {room_id: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Array<PlayerEntry>>>

  /**
   * Construct and simulate a get_player_by_screen_name transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_player_by_screen_name: ({room_id, screen_name}: {room_id: u32, screen_name: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Option<string>>>

  /**
   * Construct and simulate a get_room_config transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_room_config: ({room_id}: {room_id: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Option<RoomConfig>>>

  /**
   * Construct and simulate a get_room_financials transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_room_financials: ({room_id}: {room_id: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Option<readonly [i128, i128, i128, i128, i128]>>>

  /**
   * Construct and simulate a get_platform_wallet transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_platform_wallet: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<string>>

  /**
   * Construct and simulate a get_charity_wallet transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_charity_wallet: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<string>>

}
export class Client extends ContractClient {
  static async deploy<T = Client>(
    /** Options for initializing a Client as well as for calling a method, with extras specific to deploying. */
    options: MethodOptions &
      Omit<ContractClientOptions, "contractId"> & {
        /** The hash of the Wasm blob, which must already be installed on-chain. */
        wasmHash: Buffer | string;
        /** Salt used to generate the contract's ID. Passed through to {@link Operation.createCustomContract}. Default: random. */
        salt?: Buffer | Uint8Array;
        /** The format used to decode `wasmHash`, if it's provided as a string. */
        format?: "hex" | "base64";
      }
  ): Promise<AssembledTransaction<T>> {
    return ContractClient.deploy(null, options)
  }
  constructor(public readonly options: ContractClientOptions) {
    super(
      new ContractSpec([ "AAAAAgAAAAAAAAAAAAAACVByaXplTW9kZQAAAAAAAAIAAAAAAAAAAAAAAA5Qcml6ZVBvb2xTcGxpdAAAAAAAAAAAAAAAAAAKQXNzZXRCYXNlZAAA",
        "AAAAAQAAAAAAAAAAAAAAClByaXplQXNzZXQAAAAAAAIAAAAAAAAABmFtb3VudAAAAAAACwAAAAAAAAALY29udHJhY3RfaWQAAAAAEw==",
        "AAAAAQAAAAAAAAAAAAAAC1BsYXllckVudHJ5AAAAAAYAAAAAAAAACmVudHJ5X3BhaWQAAAAAAAsAAAAAAAAAC2V4dHJhc19wYWlkAAAAAAsAAAAAAAAAC2pvaW5fbGVkZ2VyAAAAAAQAAAAAAAAABnBsYXllcgAAAAAAEwAAAAAAAAALc2NyZWVuX25hbWUAAAAAEAAAAAAAAAAKdG90YWxfcGFpZAAAAAAACw==",
        "AAAAAQAAAAAAAAAAAAAAClJvb21Db25maWcAAAAAABQAAAAAAAAAC2NoYXJpdHlfYnBzAAAAAAQAAAAAAAAAD2NyZWF0aW9uX2xlZGdlcgAAAAAEAAAAAAAAAAVlbmRlZAAAAAAAAAEAAAAAAAAACWVudHJ5X2ZlZQAAAAAAAAsAAAAAAAAACWZlZV90b2tlbgAAAAAAABMAAAAAAAAABGhvc3QAAAATAAAAAAAAAAxob3N0X2ZlZV9icHMAAAAEAAAAAAAAAAtob3N0X3dhbGxldAAAAAPoAAAAEwAAAAAAAAAQcGxheWVyX2FkZHJlc3NlcwAAA+oAAAATAAAAAAAAAAdwbGF5ZXJzAAAAA+oAAAfQAAAAC1BsYXllckVudHJ5AAAAAAAAAAAMcHJpemVfYXNzZXRzAAAD6gAAA+gAAAfQAAAAClByaXplQXNzZXQAAAAAAAAAAAAScHJpemVfZGlzdHJpYnV0aW9uAAAAAAPqAAAABAAAAAAAAAAKcHJpemVfbW9kZQAAAAAH0AAAAAlQcml6ZU1vZGUAAAAAAAAAAAAADnByaXplX3Bvb2xfYnBzAAAAAAAEAAAAAAAAAAdyb29tX2lkAAAAA+4AAAAgAAAAAAAAABB0b3RhbF9lbnRyeV9mZWVzAAAACwAAAAAAAAARdG90YWxfZXh0cmFzX2ZlZXMAAAAAAAALAAAAAAAAAA50b3RhbF9wYWlkX291dAAAAAAACwAAAAAAAAAKdG90YWxfcG9vbAAAAAAACwAAAAAAAAAHd2lubmVycwAAAAPqAAAAEw==",
        "AAAABAAAAAAAAAAAAAAACVF1aXpFcnJvcgAAAAAAABcAAAAAAAAADkludmFsaWRIb3N0RmVlAAAAAAABAAAAAAAAABFNaXNzaW5nSG9zdFdhbGxldAAAAAAAAAIAAAAAAAAAEUludmFsaWRQcml6ZVNwbGl0AAAAAAAAAwAAAAAAAAATQ2hhcml0eUJlbG93TWluaW11bQAAAAAEAAAAAAAAABNJbnZhbGlkUHJpemVQb29sQnBzAAAAAAUAAAAAAAAAFk1pc3NpbmdQcml6ZVBvb2xDb25maWcAAAAAAAYAAAAAAAAAEk1pc3NpbmdQcml6ZUFzc2V0cwAAAAAABwAAAAAAAAASSW52YWxpZFByaXplQXNzZXRzAAAAAAAIAAAAAAAAABZJbnZhbGlkVG90YWxBbGxvY2F0aW9uAAAAAAAJAAAAAAAAAA9JbnZhbGlkRmVlVG9rZW4AAAAACgAAAAAAAAARUm9vbUFscmVhZHlFeGlzdHMAAAAAAAALAAAAAAAAAAxSb29tTm90Rm91bmQAAAAMAAAAAAAAABBSb29tQWxyZWFkeUVuZGVkAAAADwAAAAAAAAATUGxheWVyQWxyZWFkeUpvaW5lZAAAAAAQAAAAAAAAABNJbnN1ZmZpY2llbnRQYXltZW50AAAAABEAAAAAAAAADFVuYXV0aG9yaXplZAAAABIAAAAAAAAADkludmFsaWRXaW5uZXJzAAAAAAATAAAAAAAAABNBc3NldFRyYW5zZmVyRmFpbGVkAAAAABQAAAAAAAAAE0luc3VmZmljaWVudFBsYXllcnMAAAAAFQAAAAAAAAASSW5zdWZmaWNpZW50QXNzZXRzAAAAAAAWAAAAAAAAAA1EZXBvc2l0RmFpbGVkAAAAAAAAFwAAAAAAAAAPU2NyZWVuTmFtZVRha2VuAAAAABgAAAAAAAAAEUludmFsaWRTY3JlZW5OYW1lAAAAAAAAGQ==",
        "AAAAAAAAAAAAAAAOaW5pdF9wb29sX3Jvb20AAAAAAAkAAAAAAAAAB3Jvb21faWQAAAAABAAAAAAAAAAEaG9zdAAAABMAAAAAAAAACWZlZV90b2tlbgAAAAAAABMAAAAAAAAACWVudHJ5X2ZlZQAAAAAAAAsAAAAAAAAADGhvc3RfZmVlX2JwcwAAA+gAAAAEAAAAAAAAAA5wcml6ZV9wb29sX2JwcwAAAAAABAAAAAAAAAAPZmlyc3RfcGxhY2VfcGN0AAAAAAQAAAAAAAAAEHNlY29uZF9wbGFjZV9wY3QAAAPoAAAABAAAAAAAAAAPdGhpcmRfcGxhY2VfcGN0AAAAA+gAAAAEAAAAAQAAA+kAAAPtAAAAAAAAB9AAAAAJUXVpekVycm9yAAAA",
        "AAAAAAAAAAAAAAAPaW5pdF9hc3NldF9yb29tAAAAAAYAAAAAAAAAB3Jvb21faWQAAAAABAAAAAAAAAAEaG9zdAAAABMAAAAAAAAACWZlZV90b2tlbgAAAAAAABMAAAAAAAAACWVudHJ5X2ZlZQAAAAAAAAsAAAAAAAAADGhvc3RfZmVlX2JwcwAAA+gAAAAEAAAAAAAAAAZwcml6ZXMAAAAAA+oAAAfQAAAAClByaXplQXNzZXQAAAAAAAEAAAPpAAAD7QAAAAAAAAfQAAAACVF1aXpFcnJvcgAAAA==",
        "AAAAAAAAAAAAAAAJam9pbl9yb29tAAAAAAAABAAAAAAAAAAHcm9vbV9pZAAAAAAEAAAAAAAAAAZwbGF5ZXIAAAAAABMAAAAAAAAAC3NjcmVlbl9uYW1lAAAAABAAAAAAAAAADWV4dHJhc19hbW91bnQAAAAAAAALAAAAAQAAA+kAAAPtAAAAAAAAB9AAAAAJUXVpekVycm9yAAAA",
        "AAAAAAAAAAAAAAAIZW5kX3Jvb20AAAAEAAAAAAAAAAdyb29tX2lkAAAAAAQAAAAAAAAAC2ZpcnN0X3BsYWNlAAAAA+gAAAATAAAAAAAAAAxzZWNvbmRfcGxhY2UAAAPoAAAAEwAAAAAAAAALdGhpcmRfcGxhY2UAAAAD6AAAABMAAAABAAAD6QAAA+0AAAAAAAAH0AAAAAlRdWl6RXJyb3IAAAA=",
        "AAAAAAAAAAAAAAAYZW5kX3Jvb21fYnlfc2NyZWVuX25hbWVzAAAABAAAAAAAAAAHcm9vbV9pZAAAAAAEAAAAAAAAABBmaXJzdF9wbGFjZV9uYW1lAAAD6AAAABAAAAAAAAAAEXNlY29uZF9wbGFjZV9uYW1lAAAAAAAD6AAAABAAAAAAAAAAEHRoaXJkX3BsYWNlX25hbWUAAAPoAAAAEAAAAAEAAAPpAAAD7QAAAAAAAAfQAAAACVF1aXpFcnJvcgAAAA==",
        "AAAAAAAAAAAAAAAQZ2V0X3Jvb21fcGxheWVycwAAAAEAAAAAAAAAB3Jvb21faWQAAAAABAAAAAEAAAPqAAAH0AAAAAtQbGF5ZXJFbnRyeQA=",
        "AAAAAAAAAAAAAAAZZ2V0X3BsYXllcl9ieV9zY3JlZW5fbmFtZQAAAAAAAAIAAAAAAAAAB3Jvb21faWQAAAAABAAAAAAAAAALc2NyZWVuX25hbWUAAAAAEAAAAAEAAAPoAAAAEw==",
        "AAAAAAAAAAAAAAAPZ2V0X3Jvb21fY29uZmlnAAAAAAEAAAAAAAAAB3Jvb21faWQAAAAABAAAAAEAAAPoAAAH0AAAAApSb29tQ29uZmlnAAA=",
        "AAAAAAAAAAAAAAATZ2V0X3Jvb21fZmluYW5jaWFscwAAAAABAAAAAAAAAAdyb29tX2lkAAAAAAQAAAABAAAD6AAAA+0AAAAFAAAACwAAAAsAAAALAAAACwAAAAs=",
        "AAAAAAAAAAAAAAATZ2V0X3BsYXRmb3JtX3dhbGxldAAAAAAAAAAAAQAAABA=",
        "AAAAAAAAAAAAAAASZ2V0X2NoYXJpdHlfd2FsbGV0AAAAAAAAAAAAAQAAABA=" ]),
      options
    )
  }
  public readonly fromJSON = {
    init_pool_room: this.txFromJSON<Result<void>>,
        init_asset_room: this.txFromJSON<Result<void>>,
        join_room: this.txFromJSON<Result<void>>,
        end_room: this.txFromJSON<Result<void>>,
        end_room_by_screen_names: this.txFromJSON<Result<void>>,
        get_room_players: this.txFromJSON<Array<PlayerEntry>>,
        get_player_by_screen_name: this.txFromJSON<Option<string>>,
        get_room_config: this.txFromJSON<Option<RoomConfig>>,
        get_room_financials: this.txFromJSON<Option<readonly [i128, i128, i128, i128, i128]>>,
        get_platform_wallet: this.txFromJSON<string>,
        get_charity_wallet: this.txFromJSON<string>
  }
}