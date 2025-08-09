import { Buffer } from "buffer";
import { AssembledTransaction, Client as ContractClient, ClientOptions as ContractClientOptions, MethodOptions, Result } from '@stellar/stellar-sdk/contract';
import type { u32, i128, Option } from '@stellar/stellar-sdk/contract';
export * from '@stellar/stellar-sdk';
export * as contract from '@stellar/stellar-sdk/contract';
export * as rpc from '@stellar/stellar-sdk/rpc';
export declare const networks: {
    readonly testnet: {
        readonly networkPassphrase: "Test SDF Network ; September 2015";
        readonly contractId: "CCJNWS5X3WHQNPXVVMRJV2SHCS43Y744E22V5QNI7K6OFXICKXPRNYYM";
    };
};
export type PrizeMode = {
    tag: "PrizePoolSplit";
    values: void;
} | {
    tag: "AssetBased";
    values: void;
};
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
export declare const QuizError: {
    1: {
        message: string;
    };
    2: {
        message: string;
    };
    3: {
        message: string;
    };
    4: {
        message: string;
    };
    5: {
        message: string;
    };
    6: {
        message: string;
    };
    7: {
        message: string;
    };
    8: {
        message: string;
    };
    9: {
        message: string;
    };
    10: {
        message: string;
    };
    11: {
        message: string;
    };
    12: {
        message: string;
    };
    15: {
        message: string;
    };
    16: {
        message: string;
    };
    17: {
        message: string;
    };
    18: {
        message: string;
    };
    19: {
        message: string;
    };
    20: {
        message: string;
    };
    21: {
        message: string;
    };
    22: {
        message: string;
    };
    23: {
        message: string;
    };
    24: {
        message: string;
    };
    25: {
        message: string;
    };
};
export interface Client {
    /**
     * Construct and simulate a init_pool_room transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
     */
    init_pool_room: ({ room_id, host, fee_token, entry_fee, host_fee_bps, prize_pool_bps, first_place_pct, second_place_pct, third_place_pct }: {
        room_id: u32;
        host: string;
        fee_token: string;
        entry_fee: i128;
        host_fee_bps: Option<u32>;
        prize_pool_bps: u32;
        first_place_pct: u32;
        second_place_pct: Option<u32>;
        third_place_pct: Option<u32>;
    }, options?: {
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
    }) => Promise<AssembledTransaction<Result<void>>>;
    /**
     * Construct and simulate a init_asset_room transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
     */
    init_asset_room: ({ room_id, host, fee_token, entry_fee, host_fee_bps, prizes }: {
        room_id: u32;
        host: string;
        fee_token: string;
        entry_fee: i128;
        host_fee_bps: Option<u32>;
        prizes: Array<PrizeAsset>;
    }, options?: {
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
    }) => Promise<AssembledTransaction<Result<void>>>;
    /**
     * Construct and simulate a join_room transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
     */
    join_room: ({ room_id, player, screen_name, extras_amount }: {
        room_id: u32;
        player: string;
        screen_name: string;
        extras_amount: i128;
    }, options?: {
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
    }) => Promise<AssembledTransaction<Result<void>>>;
    /**
     * Construct and simulate a end_room transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
     */
    end_room: ({ room_id, first_place, second_place, third_place }: {
        room_id: u32;
        first_place: Option<string>;
        second_place: Option<string>;
        third_place: Option<string>;
    }, options?: {
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
    }) => Promise<AssembledTransaction<Result<void>>>;
    /**
     * Construct and simulate a end_room_by_screen_names transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
     */
    end_room_by_screen_names: ({ room_id, first_place_name, second_place_name, third_place_name }: {
        room_id: u32;
        first_place_name: Option<string>;
        second_place_name: Option<string>;
        third_place_name: Option<string>;
    }, options?: {
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
    }) => Promise<AssembledTransaction<Result<void>>>;
    /**
     * Construct and simulate a get_room_players transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
     */
    get_room_players: ({ room_id }: {
        room_id: u32;
    }, options?: {
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
    }) => Promise<AssembledTransaction<Array<PlayerEntry>>>;
    /**
     * Construct and simulate a get_player_by_screen_name transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
     */
    get_player_by_screen_name: ({ room_id, screen_name }: {
        room_id: u32;
        screen_name: string;
    }, options?: {
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
    }) => Promise<AssembledTransaction<Option<string>>>;
    /**
     * Construct and simulate a get_room_config transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
     */
    get_room_config: ({ room_id }: {
        room_id: u32;
    }, options?: {
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
    }) => Promise<AssembledTransaction<Option<RoomConfig>>>;
    /**
     * Construct and simulate a get_room_financials transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
     */
    get_room_financials: ({ room_id }: {
        room_id: u32;
    }, options?: {
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
    }) => Promise<AssembledTransaction<Option<readonly [i128, i128, i128, i128, i128]>>>;
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
    }) => Promise<AssembledTransaction<string>>;
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
    }) => Promise<AssembledTransaction<string>>;
}
export declare class Client extends ContractClient {
    readonly options: ContractClientOptions;
    static deploy<T = Client>(
    /** Options for initializing a Client as well as for calling a method, with extras specific to deploying. */
    options: MethodOptions & Omit<ContractClientOptions, "contractId"> & {
        /** The hash of the Wasm blob, which must already be installed on-chain. */
        wasmHash: Buffer | string;
        /** Salt used to generate the contract's ID. Passed through to {@link Operation.createCustomContract}. Default: random. */
        salt?: Buffer | Uint8Array;
        /** The format used to decode `wasmHash`, if it's provided as a string. */
        format?: "hex" | "base64";
    }): Promise<AssembledTransaction<T>>;
    constructor(options: ContractClientOptions);
    readonly fromJSON: {
        init_pool_room: (json: string) => AssembledTransaction<Result<void, import("@stellar/stellar-sdk/contract").ErrorMessage>>;
        init_asset_room: (json: string) => AssembledTransaction<Result<void, import("@stellar/stellar-sdk/contract").ErrorMessage>>;
        join_room: (json: string) => AssembledTransaction<Result<void, import("@stellar/stellar-sdk/contract").ErrorMessage>>;
        end_room: (json: string) => AssembledTransaction<Result<void, import("@stellar/stellar-sdk/contract").ErrorMessage>>;
        end_room_by_screen_names: (json: string) => AssembledTransaction<Result<void, import("@stellar/stellar-sdk/contract").ErrorMessage>>;
        get_room_players: (json: string) => AssembledTransaction<PlayerEntry[]>;
        get_player_by_screen_name: (json: string) => AssembledTransaction<Option<string>>;
        get_room_config: (json: string) => AssembledTransaction<Option<RoomConfig>>;
        get_room_financials: (json: string) => AssembledTransaction<Option<readonly [bigint, bigint, bigint, bigint, bigint]>>;
        get_platform_wallet: (json: string) => AssembledTransaction<string>;
        get_charity_wallet: (json: string) => AssembledTransaction<string>;
    };
}
