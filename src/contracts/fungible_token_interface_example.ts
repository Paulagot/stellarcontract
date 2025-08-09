import * as Client from 'fungible_token_interface_example';
import { rpcUrl } from './util';

export default new Client.Client({
  networkPassphrase: 'Test SDF Network ; September 2015',
  contractId: 'CDDS4YPZ5DWCVJKHVJTSZAB5YXDT44MNDZGZKDPMFZDEDCQ7SEOBJN47',
  rpcUrl,
  publicKey: undefined,
});
