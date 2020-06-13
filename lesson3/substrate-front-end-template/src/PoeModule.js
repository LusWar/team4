import React, { useEffect, useState } from 'react';
import { Form, Input, Grid } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';
import { blake2AsHex } from '@polkadot/util-crypto';

function Main (props) {
  const { api } = useSubstrate();
  const { accountPair } = props;

  // The transaction submission status
  const [status, setStatus] = useState('');
  const [digest, setDigest] = useState('');
  const [owner, setOwner] = useState('');
  const [blockNumber, setBlockNumber] = useState(0);
  const [receiver, setReceiver] = useState('');

  useEffect(() => {
    let unsubscribe;
    api.query.poeModule.proofs(digest, (result) => {
        setOwner(result[0].toString());
        setBlockNumber(result[1].toNumber());
    }).then(unsub => {
      unsubscribe = unsub;
    })
      .catch(console.error);

    return () => unsubscribe && unsubscribe();
  }, [digest, api.query.poeModule]);

  let fileReader;

  // Takes our file, and creates a digest using the Blake2 256 hash function.
  const bufferToDigest = () => {
    // Turns the file content to a hexadecimal representation.
    const content = Array.from(new Uint8Array(fileReader.result))
      .map((b) => b.toString(16).padStart(2, '0'))
      .join('');

    const hash = blake2AsHex(content, 256);
    setDigest(hash);
  };

  // Callback function for when a new file is selected.
  const handleFileChosen = (file) => {
    fileReader = new FileReader();
    fileReader.onloadend = bufferToDigest;
    fileReader.readAsArrayBuffer(file);
  };

  return (
    <Grid.Column width={8}>
      <h1>Poe Module</h1>
      <Form>
        <Form.Field>
          <Input
            type='file'
            id='file'
            label={'Your file'}
            onChange={ (e) => handleFileChosen(e.target.files[0])}
          />
          <Input 
            type='string'
            id='receiver'
            label={'Receiver'}
            onChange = { (_, value) => setReceiver(value.value)}
          />
        </Form.Field>
        <Form.Field>
          <TxButton 
            accountPair={accountPair}
            label='create claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'createClaim',
              inputParams: [digest],
              paramFields:[true]
            }}
          />
          <TxButton 
            accountPair={accountPair}
            label='revoke claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'revokeClaim',
              inputParams: [digest],
              paramFields:[true]
            }}
          />
          <TxButton 
            accountPair={accountPair}
            label='transfer claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'transferClaim',
              inputParams: [digest,receiver],
              paramFields:[true]
            }}
          />
        </Form.Field>
        <div>{status}</div>
          <div>{`Claim Info owner: ${owner} blockNumber ${blockNumber}`}</div>
      </Form>
    </Grid.Column>
  );
}

export default function PoeModule (props) {
  const { api } = useSubstrate();
  return (api.query.poeModule && api.query.poeModule.proofs
    ? <Main {...props} /> : null);
}
