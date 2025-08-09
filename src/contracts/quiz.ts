import * as Client from 'quiz';
import { rpcUrl } from './util';

export default new Client.Client({
  networkPassphrase: 'Test SDF Network ; September 2015',
  contractId: 'CCJNWS5X3WHQNPXVVMRJV2SHCS43Y744E22V5QNI7K6OFXICKXPRNYYM',
  rpcUrl,
  publicKey: undefined,
});
