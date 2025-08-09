import { Buffer } from "buffer";
import { AssembledTransaction, Client as ContractClient, ClientOptions as ContractClientOptions, MethodOptions } from '@stellar/stellar-sdk/contract';
import type { u32, i128 } from '@stellar/stellar-sdk/contract';
export * from '@stellar/stellar-sdk';
export * as contract from '@stellar/stellar-sdk/contract';
export * as rpc from '@stellar/stellar-sdk/rpc';
export declare const networks: {
    readonly testnet: {
        readonly networkPassphrase: "Test SDF Network ; September 2015";
        readonly contractId: "CDDS4YPZ5DWCVJKHVJTSZAB5YXDT44MNDZGZKDPMFZDEDCQ7SEOBJN47";
    };
};
export declare const ExampleContractError: {
    1: {
        message: string;
    };
};
export declare const CryptoError: {
    /**
     * The merkle proof length is out of bounds.
     */
    1400: {
        message: string;
    };
    /**
     * The index of the leaf is out of bounds.
     */
    1401: {
        message: string;
    };
    /**
     * No data in hasher state.
     */
    1402: {
        message: string;
    };
};
/**
 * Storage keys for the data associated with `MerkleDistributor`
 */
export type MerkleDistributorStorageKey = {
    tag: "Root";
    values: void;
} | {
    tag: "Claimed";
    values: readonly [u32];
};
export declare const MerkleDistributorError: {
    /**
     * The merkle root is not set.
     */
    1300: {
        message: string;
    };
    /**
     * The provided index was already claimed.
     */
    1301: {
        message: string;
    };
    /**
     * The proof is invalid.
     */
    1302: {
        message: string;
    };
};
/**
 * Storage key for the pausable state
 */
export type PausableStorageKey = {
    tag: "Paused";
    values: void;
};
export declare const PausableError: {
    /**
     * The operation failed because the contract is paused.
     */
    1000: {
        message: string;
    };
    /**
     * The operation failed because the contract is not paused.
     */
    1001: {
        message: string;
    };
};
export declare const UpgradeableError: {
    /**
     * When migration is attempted but not allowed due to upgrade state.
     */
    1100: {
        message: string;
    };
};
/**
 * Storage keys for the data associated with the allowlist extension
 */
export type AllowListStorageKey = {
    tag: "Allowed";
    values: readonly [string];
};
/**
 * Storage keys for the data associated with the blocklist extension
 */
export type BlockListStorageKey = {
    tag: "Blocked";
    values: readonly [string];
};
/**
 * Storage key that maps to [`AllowanceData`]
 */
export interface AllowanceKey {
    owner: string;
    spender: string;
}
/**
 * Storage container for the amount of tokens for which an allowance is granted
 * and the ledger number at which this allowance expires.
 */
export interface AllowanceData {
    amount: i128;
    live_until_ledger: u32;
}
/**
 * Storage keys for the data associated with `FungibleToken`
 */
export type StorageKey = {
    tag: "TotalSupply";
    values: void;
} | {
    tag: "Balance";
    values: readonly [string];
} | {
    tag: "Allowance";
    values: readonly [AllowanceKey];
};
/**
 * Storage container for token metadata
 */
export interface Metadata {
    decimals: u32;
    name: string;
    symbol: string;
}
/**
 * Storage key for accessing the SAC address
 */
export type SACAdminGenericDataKey = {
    tag: "Sac";
    values: void;
};
/**
 * Storage key for accessing the SAC address
 */
export type SACAdminWrapperDataKey = {
    tag: "Sac";
    values: void;
};
export declare const FungibleTokenError: {
    /**
     * Indicates an error related to the current balance of account from which
     * tokens are expected to be transferred.
     */
    100: {
        message: string;
    };
    /**
     * Indicates a failure with the allowance mechanism when a given spender
     * doesn't have enough allowance.
     */
    101: {
        message: string;
    };
    /**
     * Indicates an invalid value for `live_until_ledger` when setting an
     * allowance.
     */
    102: {
        message: string;
    };
    /**
     * Indicates an error when an input that must be >= 0
     */
    103: {
        message: string;
    };
    /**
     * Indicates overflow when adding two values
     */
    104: {
        message: string;
    };
    /**
     * Indicates access to uninitialized metadata
     */
    105: {
        message: string;
    };
    /**
     * Indicates that the operation would have caused `total_supply` to exceed
     * the `cap`.
     */
    106: {
        message: string;
    };
    /**
     * Indicates the supplied `cap` is not a valid cap value.
     */
    107: {
        message: string;
    };
    /**
     * Indicates the Cap was not set.
     */
    108: {
        message: string;
    };
    /**
     * Indicates the SAC address was not set.
     */
    109: {
        message: string;
    };
    /**
     * Indicates a SAC address different than expected.
     */
    110: {
        message: string;
    };
    /**
     * Indicates a missing function parameter in the SAC contract context.
     */
    111: {
        message: string;
    };
    /**
     * Indicates an invalid function parameter in the SAC contract context.
     */
    112: {
        message: string;
    };
    /**
     * The user is not allowed to perform this operation
     */
    113: {
        message: string;
    };
    /**
     * The user is blocked and cannot perform this operation
     */
    114: {
        message: string;
    };
};
/**
 * Storage keys for the data associated with the consecutive extension of
 * `NonFungibleToken`
 */
export type NFTConsecutiveStorageKey = {
    tag: "Approval";
    values: readonly [u32];
} | {
    tag: "Owner";
    values: readonly [u32];
} | {
    tag: "OwnershipBucket";
    values: readonly [u32];
} | {
    tag: "BurnedToken";
    values: readonly [u32];
};
export interface OwnerTokensKey {
    index: u32;
    owner: string;
}
/**
 * Storage keys for the data associated with the enumerable extension of
 * `NonFungibleToken`
 */
export type NFTEnumerableStorageKey = {
    tag: "TotalSupply";
    values: void;
} | {
    tag: "OwnerTokens";
    values: readonly [OwnerTokensKey];
} | {
    tag: "OwnerTokensIndex";
    values: readonly [u32];
} | {
    tag: "GlobalTokens";
    values: readonly [u32];
} | {
    tag: "GlobalTokensIndex";
    values: readonly [u32];
};
/**
 * Storage container for royalty information
 */
export interface RoyaltyInfo {
    basis_points: u32;
    receiver: string;
}
/**
 * Storage keys for royalty data
 */
export type NFTRoyaltiesStorageKey = {
    tag: "DefaultRoyalty";
    values: void;
} | {
    tag: "TokenRoyalty";
    values: readonly [u32];
};
/**
 * Storage container for the token for which an approval is granted
 * and the ledger number at which this approval expires.
 */
export interface ApprovalData {
    approved: string;
    live_until_ledger: u32;
}
/**
 * Storage container for token metadata
 */
export interface Metadata {
    base_uri: string;
    name: string;
    symbol: string;
}
/**
 * Storage keys for the data associated with `NonFungibleToken`
 */
export type NFTStorageKey = {
    tag: "Owner";
    values: readonly [u32];
} | {
    tag: "Balance";
    values: readonly [string];
} | {
    tag: "Approval";
    values: readonly [u32];
} | {
    tag: "ApprovalForAll";
    values: readonly [string, string];
} | {
    tag: "Metadata";
    values: void;
};
export type NFTSequentialStorageKey = {
    tag: "TokenIdCounter";
    values: void;
};
export declare const NonFungibleTokenError: {
    /**
     * Indicates a non-existent `token_id`.
     */
    200: {
        message: string;
    };
    /**
     * Indicates an error related to the ownership over a particular token.
     * Used in transfers.
     */
    201: {
        message: string;
    };
    /**
     * Indicates a failure with the `operator`s approval. Used in transfers.
     */
    202: {
        message: string;
    };
    /**
     * Indicates a failure with the `approver` of a token to be approved. Used
     * in approvals.
     */
    203: {
        message: string;
    };
    /**
     * Indicates an invalid value for `live_until_ledger` when setting
     * approvals.
     */
    204: {
        message: string;
    };
    /**
     * Indicates overflow when adding two values
     */
    205: {
        message: string;
    };
    /**
     * Indicates all possible `token_id`s are already in use.
     */
    206: {
        message: string;
    };
    /**
     * Indicates an invalid amount to batch mint in `consecutive` extension.
     */
    207: {
        message: string;
    };
    /**
     * Indicates the token does not exist in owner's list.
     */
    208: {
        message: string;
    };
    /**
     * Indicates the token does not exist in global list.
     */
    209: {
        message: string;
    };
    /**
     * Indicates access to unset metadata.
     */
    210: {
        message: string;
    };
    /**
     * Indicates the length of the base URI exceeds the maximum allowed.
     */
    211: {
        message: string;
    };
    /**
     * Indicates the royalty amount is higher than 10_000 (100%) basis points.
     */
    212: {
        message: string;
    };
};
export interface Client {
    /**
     * Construct and simulate a total_supply transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
     * `TokenInterface` doesn't require implementing `total_supply()` because
     * of the need for backwards compatibility with Stellar classic assets.
     */
    total_supply: (options?: {
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
    }) => Promise<AssembledTransaction<i128>>;
    /**
     * Construct and simulate a mint transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
     */
    mint: ({ account, amount }: {
        account: string;
        amount: i128;
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
    }) => Promise<AssembledTransaction<null>>;
    /**
     * Construct and simulate a paused transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
     */
    paused: (options?: {
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
    }) => Promise<AssembledTransaction<boolean>>;
    /**
     * Construct and simulate a pause transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
     */
    pause: ({ caller }: {
        caller: string;
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
    }) => Promise<AssembledTransaction<null>>;
    /**
     * Construct and simulate a unpause transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
     */
    unpause: ({ caller }: {
        caller: string;
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
    }) => Promise<AssembledTransaction<null>>;
    /**
     * Construct and simulate a balance transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
     */
    balance: ({ account }: {
        account: string;
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
    }) => Promise<AssembledTransaction<i128>>;
    /**
     * Construct and simulate a allowance transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
     */
    allowance: ({ owner, spender }: {
        owner: string;
        spender: string;
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
    }) => Promise<AssembledTransaction<i128>>;
    /**
     * Construct and simulate a transfer transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
     */
    transfer: ({ from, to, amount }: {
        from: string;
        to: string;
        amount: i128;
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
    }) => Promise<AssembledTransaction<null>>;
    /**
     * Construct and simulate a transfer_from transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
     */
    transfer_from: ({ spender, from, to, amount }: {
        spender: string;
        from: string;
        to: string;
        amount: i128;
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
    }) => Promise<AssembledTransaction<null>>;
    /**
     * Construct and simulate a approve transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
     */
    approve: ({ owner, spender, amount, live_until_ledger }: {
        owner: string;
        spender: string;
        amount: i128;
        live_until_ledger: u32;
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
    }) => Promise<AssembledTransaction<null>>;
    /**
     * Construct and simulate a burn transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
     */
    burn: ({ from, amount }: {
        from: string;
        amount: i128;
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
    }) => Promise<AssembledTransaction<null>>;
    /**
     * Construct and simulate a burn_from transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
     */
    burn_from: ({ spender, from, amount }: {
        spender: string;
        from: string;
        amount: i128;
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
    }) => Promise<AssembledTransaction<null>>;
    /**
     * Construct and simulate a decimals transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
     */
    decimals: (options?: {
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
    }) => Promise<AssembledTransaction<u32>>;
    /**
     * Construct and simulate a name transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
     */
    name: (options?: {
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
     * Construct and simulate a symbol transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
     */
    symbol: (options?: {
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
    /** Constructor/Initialization Args for the contract's `__constructor` method */
    { owner, initial_supply }: {
        owner: string;
        initial_supply: i128;
    }, 
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
        total_supply: (json: string) => AssembledTransaction<bigint>;
        mint: (json: string) => AssembledTransaction<null>;
        paused: (json: string) => AssembledTransaction<boolean>;
        pause: (json: string) => AssembledTransaction<null>;
        unpause: (json: string) => AssembledTransaction<null>;
        balance: (json: string) => AssembledTransaction<bigint>;
        allowance: (json: string) => AssembledTransaction<bigint>;
        transfer: (json: string) => AssembledTransaction<null>;
        transfer_from: (json: string) => AssembledTransaction<null>;
        approve: (json: string) => AssembledTransaction<null>;
        burn: (json: string) => AssembledTransaction<null>;
        burn_from: (json: string) => AssembledTransaction<null>;
        decimals: (json: string) => AssembledTransaction<number>;
        name: (json: string) => AssembledTransaction<string>;
        symbol: (json: string) => AssembledTransaction<string>;
    };
}
